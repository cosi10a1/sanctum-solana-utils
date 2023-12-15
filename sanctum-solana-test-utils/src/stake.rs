use borsh::BorshSerialize;
use solana_program::{
    pubkey::Pubkey,
    stake::state::{Authorized, Delegation, Lockup, Meta, Stake, StakeState},
    stake_history::Epoch,
};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::account::Account;

use crate::{est_rent_exempt_lamports, ExtendedProgramTest, IntoAccount};

#[derive(Clone, Copy, Debug)]
pub struct StakeStateAndLamports {
    pub stake_state: StakeState,
    /// staked amount ~ total_lamports - stake_state.meta.rent_exempt_reserve
    pub total_lamports: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct SingleAuthorityAuthorized(pub Pubkey);

impl From<SingleAuthorityAuthorized> for Authorized {
    fn from(SingleAuthorityAuthorized(pk): SingleAuthorityAuthorized) -> Self {
        Self {
            withdrawer: pk,
            staker: pk,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LiveStakeAccountParams {
    pub staked_lamports: u64,
    pub voter: Pubkey,
    pub authorized: Authorized,
    pub activation_epoch: Epoch,
    pub deactivation_epoch: Epoch,
    pub lockup: Lockup,
    pub credits_observed: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct ActiveOrActivatingUnlockedStakeAccount {
    pub staked_lamports: u64,
    pub voter: Pubkey,
    pub authorized: Authorized,
    pub activation_epoch: Epoch,
    pub credits_observed: u64,
}

impl From<ActiveOrActivatingUnlockedStakeAccount> for LiveStakeAccountParams {
    fn from(
        ActiveOrActivatingUnlockedStakeAccount {
            staked_lamports,
            voter,
            authorized,
            activation_epoch,
            credits_observed,
        }: ActiveOrActivatingUnlockedStakeAccount,
    ) -> Self {
        Self {
            staked_lamports,
            voter,
            authorized,
            activation_epoch,
            deactivation_epoch: u64::MAX,
            lockup: Default::default(),
            credits_observed,
        }
    }
}

pub trait StakeProgramTest {
    fn add_stake_account(self, addr: Pubkey, account: StakeStateAndLamports) -> Self;

    fn add_fresh_inactive_stake_account(
        self,
        addr: Pubkey,
        total_lamports: u64,
        authorized: Authorized,
    ) -> Self;

    /// Add a `StakeState::State` stake account
    fn add_live_stake_account(self, addr: Pubkey, params: LiveStakeAccountParams) -> Self;

    fn add_active_unlocked_stake_account(
        self,
        addr: Pubkey,
        params: ActiveOrActivatingUnlockedStakeAccount,
    ) -> Self;
}

impl<T: ExtendedProgramTest> StakeProgramTest for T {
    fn add_stake_account(self, addr: Pubkey, account: StakeStateAndLamports) -> Self {
        self.add_keyed_account(KeyedAccount {
            pubkey: addr,
            account: account.into_account(),
        })
    }

    fn add_fresh_inactive_stake_account(
        self,
        addr: Pubkey,
        total_lamports: u64,
        authorized: Authorized,
    ) -> Self {
        self.add_stake_account(
            addr,
            StakeStateAndLamports {
                stake_state: StakeState::Initialized(Meta {
                    rent_exempt_reserve: est_rent_exempt_lamports(StakeState::size_of()),
                    authorized,
                    lockup: Default::default(),
                }),
                total_lamports,
            },
        )
    }

    fn add_live_stake_account(
        self,
        addr: Pubkey,
        LiveStakeAccountParams {
            staked_lamports,
            voter,
            authorized,
            activation_epoch,
            deactivation_epoch,
            lockup,
            credits_observed,
        }: LiveStakeAccountParams,
    ) -> Self {
        let rent_exempt_reserve = est_rent_exempt_lamports(StakeState::size_of());
        let stake_state = StakeState::Stake(
            Meta {
                rent_exempt_reserve,
                authorized,
                lockup,
            },
            Stake {
                delegation: Delegation {
                    voter_pubkey: voter,
                    stake: staked_lamports,
                    activation_epoch,
                    deactivation_epoch,
                    ..Default::default()
                },
                credits_observed,
            },
        );
        self.add_stake_account(
            addr,
            StakeStateAndLamports {
                total_lamports: staked_lamports + rent_exempt_reserve,
                stake_state,
            },
        )
    }

    fn add_active_unlocked_stake_account(
        self,
        addr: Pubkey,
        params: ActiveOrActivatingUnlockedStakeAccount,
    ) -> Self {
        self.add_live_stake_account(addr, params.into())
    }
}

impl IntoAccount for StakeStateAndLamports {
    fn into_account(self) -> Account {
        let mut data = Vec::new();
        self.stake_state.serialize(&mut data).unwrap();
        Account {
            lamports: self.total_lamports,
            data,
            owner: solana_program::stake::program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        }
    }
}
