use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::Read;
#[derive(Clone, Debug, PartialEq)]
pub enum SplStakePoolProgramIx {
    Initialize(InitializeIxArgs),
    AddValidatorToPool(AddValidatorToPoolIxArgs),
    RemoveValidatorFromPool,
    UpdateValidatorListBalance(UpdateValidatorListBalanceIxArgs),
    UpdateStakePoolBalance,
    CleanupRemovedValidatorEntries,
    SetManager,
    SetFee(SetFeeIxArgs),
    SetStaker,
    SetFundingAuthority,
    IncreaseAdditionalValidatorStake(IncreaseAdditionalValidatorStakeIxArgs),
    DecreaseAdditionalValidatorStake(DecreaseAdditionalValidatorStakeIxArgs),
}
impl SplStakePoolProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        match maybe_discm {
            INITIALIZE_IX_DISCM => Ok(Self::Initialize(InitializeIxArgs::deserialize(
                &mut reader,
            )?)),
            ADD_VALIDATOR_TO_POOL_IX_DISCM => Ok(Self::AddValidatorToPool(
                AddValidatorToPoolIxArgs::deserialize(&mut reader)?,
            )),
            REMOVE_VALIDATOR_FROM_POOL_IX_DISCM => Ok(Self::RemoveValidatorFromPool),
            UPDATE_VALIDATOR_LIST_BALANCE_IX_DISCM => Ok(Self::UpdateValidatorListBalance(
                UpdateValidatorListBalanceIxArgs::deserialize(&mut reader)?,
            )),
            UPDATE_STAKE_POOL_BALANCE_IX_DISCM => Ok(Self::UpdateStakePoolBalance),
            CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_DISCM => Ok(Self::CleanupRemovedValidatorEntries),
            SET_MANAGER_IX_DISCM => Ok(Self::SetManager),
            SET_FEE_IX_DISCM => Ok(Self::SetFee(SetFeeIxArgs::deserialize(&mut reader)?)),
            SET_STAKER_IX_DISCM => Ok(Self::SetStaker),
            SET_FUNDING_AUTHORITY_IX_DISCM => Ok(Self::SetFundingAuthority),
            INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM => {
                Ok(Self::IncreaseAdditionalValidatorStake(
                    IncreaseAdditionalValidatorStakeIxArgs::deserialize(&mut reader)?,
                ))
            }
            DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM => {
                Ok(Self::DecreaseAdditionalValidatorStake(
                    DecreaseAdditionalValidatorStakeIxArgs::deserialize(&mut reader)?,
                ))
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::Initialize(args) => {
                writer.write_all(&[INITIALIZE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::AddValidatorToPool(args) => {
                writer.write_all(&[ADD_VALIDATOR_TO_POOL_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::RemoveValidatorFromPool => {
                writer.write_all(&[REMOVE_VALIDATOR_FROM_POOL_IX_DISCM])
            }
            Self::UpdateValidatorListBalance(args) => {
                writer.write_all(&[UPDATE_VALIDATOR_LIST_BALANCE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::UpdateStakePoolBalance => writer.write_all(&[UPDATE_STAKE_POOL_BALANCE_IX_DISCM]),
            Self::CleanupRemovedValidatorEntries => {
                writer.write_all(&[CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_DISCM])
            }
            Self::SetManager => writer.write_all(&[SET_MANAGER_IX_DISCM]),
            Self::SetFee(args) => {
                writer.write_all(&[SET_FEE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::SetStaker => writer.write_all(&[SET_STAKER_IX_DISCM]),
            Self::SetFundingAuthority => writer.write_all(&[SET_FUNDING_AUTHORITY_IX_DISCM]),
            Self::IncreaseAdditionalValidatorStake(args) => {
                writer.write_all(&[INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
            Self::DecreaseAdditionalValidatorStake(args) => {
                writer.write_all(&[DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM])?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    ///New StakePool to create
    pub stake_pool: &'me AccountInfo<'info>,
    ///Manager
    pub manager: &'me AccountInfo<'info>,
    ///Staker
    pub staker: &'me AccountInfo<'info>,
    ///Stake pool withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
    ///Uninitialized validator stake list storage account
    pub validator_list: &'me AccountInfo<'info>,
    ///Reserve stake account must be initialized, have zero balance, and staker / withdrawer authority set to pool withdraw authority
    pub reserve_stake: &'me AccountInfo<'info>,
    ///Pool token mint. Must have zero supply, owned by withdraw authority.
    pub pool_token_mint: &'me AccountInfo<'info>,
    ///Pool account to deposit the generated fee for manager.
    pub manager_fee_account: &'me AccountInfo<'info>,
    ///Pool token's token program. Optional deposit authority account follows; if omitted, anyone can deposit into the pool.
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeKeys {
    ///New StakePool to create
    pub stake_pool: Pubkey,
    ///Manager
    pub manager: Pubkey,
    ///Staker
    pub staker: Pubkey,
    ///Stake pool withdraw authority
    pub withdraw_authority: Pubkey,
    ///Uninitialized validator stake list storage account
    pub validator_list: Pubkey,
    ///Reserve stake account must be initialized, have zero balance, and staker / withdrawer authority set to pool withdraw authority
    pub reserve_stake: Pubkey,
    ///Pool token mint. Must have zero supply, owned by withdraw authority.
    pub pool_token_mint: Pubkey,
    ///Pool account to deposit the generated fee for manager.
    pub manager_fee_account: Pubkey,
    ///Pool token's token program. Optional deposit authority account follows; if omitted, anyone can deposit into the pool.
    pub token_program: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            manager: *accounts.manager.key,
            staker: *accounts.staker.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            validator_list: *accounts.validator_list.key,
            reserve_stake: *accounts.reserve_stake.key,
            pool_token_mint: *accounts.pool_token_mint.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staker,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_stake,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool_token_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            manager: pubkeys[1],
            staker: pubkeys[2],
            withdraw_authority: pubkeys[3],
            validator_list: pubkeys[4],
            reserve_stake: pubkeys[5],
            pool_token_mint: pubkeys[6],
            manager_fee_account: pubkeys[7],
            token_program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.manager.clone(),
            accounts.staker.clone(),
            accounts.withdraw_authority.clone(),
            accounts.validator_list.clone(),
            accounts.reserve_stake.clone(),
            accounts.pool_token_mint.clone(),
            accounts.manager_fee_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            manager: &arr[1],
            staker: &arr[2],
            withdraw_authority: &arr[3],
            validator_list: &arr[4],
            reserve_stake: &arr[5],
            pool_token_mint: &arr[6],
            manager_fee_account: &arr[7],
            token_program: &arr[8],
        }
    }
}
pub const INITIALIZE_IX_DISCM: u8 = 0u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeIxArgs {
    pub fee: Fee,
    pub withdrawal_fee: Fee,
    pub deposit_fee: Fee,
    pub referral_fee: u8,
    pub max_validators: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeIxData(pub InitializeIxArgs);
impl From<InitializeIxArgs> for InitializeIxData {
    fn from(args: InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INITIALIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INITIALIZE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_ix(keys: InitializeKeys, args: InitializeIxArgs) -> std::io::Result<Instruction> {
    initialize_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.manager.key, &keys.manager),
        (accounts.staker.key, &keys.staker),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.reserve_stake.key, &keys.reserve_stake),
        (accounts.pool_token_mint.key, &keys.pool_token_mint),
        (accounts.manager_fee_account.key, &keys.manager_fee_account),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake_pool, accounts.validator_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_signer_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    initialize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADD_VALIDATOR_TO_POOL_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct AddValidatorToPoolAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Staker
    pub staker: &'me AccountInfo<'info>,
    ///Reserve stake account
    pub reserve_stake: &'me AccountInfo<'info>,
    ///Stake pool withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
    ///Validator list
    pub validator_list: &'me AccountInfo<'info>,
    ///Stake account to add to the pool
    pub stake_account: &'me AccountInfo<'info>,
    ///Validator this stake account will be delegated to
    pub validator: &'me AccountInfo<'info>,
    ///Rent sysvar
    pub rent: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///Stake history sysvar
    pub stake_history: &'me AccountInfo<'info>,
    ///Stake config sysvar
    pub stake_config: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Stake program
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddValidatorToPoolKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Staker
    pub staker: Pubkey,
    ///Reserve stake account
    pub reserve_stake: Pubkey,
    ///Stake pool withdraw authority
    pub withdraw_authority: Pubkey,
    ///Validator list
    pub validator_list: Pubkey,
    ///Stake account to add to the pool
    pub stake_account: Pubkey,
    ///Validator this stake account will be delegated to
    pub validator: Pubkey,
    ///Rent sysvar
    pub rent: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///Stake history sysvar
    pub stake_history: Pubkey,
    ///Stake config sysvar
    pub stake_config: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Stake program
    pub stake_program: Pubkey,
}
impl From<AddValidatorToPoolAccounts<'_, '_>> for AddValidatorToPoolKeys {
    fn from(accounts: AddValidatorToPoolAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            staker: *accounts.staker.key,
            reserve_stake: *accounts.reserve_stake.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            validator_list: *accounts.validator_list.key,
            stake_account: *accounts.stake_account.key,
            validator: *accounts.validator.key,
            rent: *accounts.rent.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_config: *accounts.stake_config.key,
            system_program: *accounts.system_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<AddValidatorToPoolKeys> for [AccountMeta; ADD_VALIDATOR_TO_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: AddValidatorToPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.validator,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ADD_VALIDATOR_TO_POOL_IX_ACCOUNTS_LEN]> for AddValidatorToPoolKeys {
    fn from(pubkeys: [Pubkey; ADD_VALIDATOR_TO_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            staker: pubkeys[1],
            reserve_stake: pubkeys[2],
            withdraw_authority: pubkeys[3],
            validator_list: pubkeys[4],
            stake_account: pubkeys[5],
            validator: pubkeys[6],
            rent: pubkeys[7],
            clock: pubkeys[8],
            stake_history: pubkeys[9],
            stake_config: pubkeys[10],
            system_program: pubkeys[11],
            stake_program: pubkeys[12],
        }
    }
}
impl<'info> From<AddValidatorToPoolAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_VALIDATOR_TO_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AddValidatorToPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.staker.clone(),
            accounts.reserve_stake.clone(),
            accounts.withdraw_authority.clone(),
            accounts.validator_list.clone(),
            accounts.stake_account.clone(),
            accounts.validator.clone(),
            accounts.rent.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_config.clone(),
            accounts.system_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_VALIDATOR_TO_POOL_IX_ACCOUNTS_LEN]>
    for AddValidatorToPoolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_VALIDATOR_TO_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            staker: &arr[1],
            reserve_stake: &arr[2],
            withdraw_authority: &arr[3],
            validator_list: &arr[4],
            stake_account: &arr[5],
            validator: &arr[6],
            rent: &arr[7],
            clock: &arr[8],
            stake_history: &arr[9],
            stake_config: &arr[10],
            system_program: &arr[11],
            stake_program: &arr[12],
        }
    }
}
pub const ADD_VALIDATOR_TO_POOL_IX_DISCM: u8 = 1u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddValidatorToPoolIxArgs {
    pub optional_seed: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddValidatorToPoolIxData(pub AddValidatorToPoolIxArgs);
impl From<AddValidatorToPoolIxArgs> for AddValidatorToPoolIxData {
    fn from(args: AddValidatorToPoolIxArgs) -> Self {
        Self(args)
    }
}
impl AddValidatorToPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != ADD_VALIDATOR_TO_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_VALIDATOR_TO_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddValidatorToPoolIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[ADD_VALIDATOR_TO_POOL_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn add_validator_to_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: AddValidatorToPoolKeys,
    args: AddValidatorToPoolIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; ADD_VALIDATOR_TO_POOL_IX_ACCOUNTS_LEN] = keys.into();
    let data: AddValidatorToPoolIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_validator_to_pool_ix(
    keys: AddValidatorToPoolKeys,
    args: AddValidatorToPoolIxArgs,
) -> std::io::Result<Instruction> {
    add_validator_to_pool_ix_with_program_id(crate::ID, keys, args)
}
pub fn add_validator_to_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AddValidatorToPoolAccounts<'_, '_>,
    args: AddValidatorToPoolIxArgs,
) -> ProgramResult {
    let keys: AddValidatorToPoolKeys = accounts.into();
    let ix = add_validator_to_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn add_validator_to_pool_invoke(
    accounts: AddValidatorToPoolAccounts<'_, '_>,
    args: AddValidatorToPoolIxArgs,
) -> ProgramResult {
    add_validator_to_pool_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn add_validator_to_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AddValidatorToPoolAccounts<'_, '_>,
    args: AddValidatorToPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AddValidatorToPoolKeys = accounts.into();
    let ix = add_validator_to_pool_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn add_validator_to_pool_invoke_signed(
    accounts: AddValidatorToPoolAccounts<'_, '_>,
    args: AddValidatorToPoolIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    add_validator_to_pool_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn add_validator_to_pool_verify_account_keys(
    accounts: AddValidatorToPoolAccounts<'_, '_>,
    keys: AddValidatorToPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.staker.key, &keys.staker),
        (accounts.reserve_stake.key, &keys.reserve_stake),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.stake_account.key, &keys.stake_account),
        (accounts.validator.key, &keys.validator),
        (accounts.rent.key, &keys.rent),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.stake_config.key, &keys.stake_config),
        (accounts.system_program.key, &keys.system_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn add_validator_to_pool_verify_writable_privileges<'me, 'info>(
    accounts: AddValidatorToPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.stake_pool,
        accounts.reserve_stake,
        accounts.validator_list,
        accounts.stake_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn add_validator_to_pool_verify_signer_privileges<'me, 'info>(
    accounts: AddValidatorToPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn add_validator_to_pool_verify_account_privileges<'me, 'info>(
    accounts: AddValidatorToPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    add_validator_to_pool_verify_writable_privileges(accounts)?;
    add_validator_to_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REMOVE_VALIDATOR_FROM_POOL_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RemoveValidatorFromPoolAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Staker
    pub staker: &'me AccountInfo<'info>,
    ///Stake pool withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
    ///Validator list
    pub validator_list: &'me AccountInfo<'info>,
    ///Stake account to remove from the pool
    pub stake_account: &'me AccountInfo<'info>,
    ///Transient stake account, to deactivate if necessary
    pub transient_stake_account: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///Stake program
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveValidatorFromPoolKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Staker
    pub staker: Pubkey,
    ///Stake pool withdraw authority
    pub withdraw_authority: Pubkey,
    ///Validator list
    pub validator_list: Pubkey,
    ///Stake account to remove from the pool
    pub stake_account: Pubkey,
    ///Transient stake account, to deactivate if necessary
    pub transient_stake_account: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///Stake program
    pub stake_program: Pubkey,
}
impl From<RemoveValidatorFromPoolAccounts<'_, '_>> for RemoveValidatorFromPoolKeys {
    fn from(accounts: RemoveValidatorFromPoolAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            staker: *accounts.staker.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            validator_list: *accounts.validator_list.key,
            stake_account: *accounts.stake_account.key,
            transient_stake_account: *accounts.transient_stake_account.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<RemoveValidatorFromPoolKeys>
    for [AccountMeta; REMOVE_VALIDATOR_FROM_POOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: RemoveValidatorFromPoolKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.staker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transient_stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REMOVE_VALIDATOR_FROM_POOL_IX_ACCOUNTS_LEN]> for RemoveValidatorFromPoolKeys {
    fn from(pubkeys: [Pubkey; REMOVE_VALIDATOR_FROM_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            staker: pubkeys[1],
            withdraw_authority: pubkeys[2],
            validator_list: pubkeys[3],
            stake_account: pubkeys[4],
            transient_stake_account: pubkeys[5],
            clock: pubkeys[6],
            stake_program: pubkeys[7],
        }
    }
}
impl<'info> From<RemoveValidatorFromPoolAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_VALIDATOR_FROM_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RemoveValidatorFromPoolAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.staker.clone(),
            accounts.withdraw_authority.clone(),
            accounts.validator_list.clone(),
            accounts.stake_account.clone(),
            accounts.transient_stake_account.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_VALIDATOR_FROM_POOL_IX_ACCOUNTS_LEN]>
    for RemoveValidatorFromPoolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_VALIDATOR_FROM_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            staker: &arr[1],
            withdraw_authority: &arr[2],
            validator_list: &arr[3],
            stake_account: &arr[4],
            transient_stake_account: &arr[5],
            clock: &arr[6],
            stake_program: &arr[7],
        }
    }
}
pub const REMOVE_VALIDATOR_FROM_POOL_IX_DISCM: u8 = 2u8;
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveValidatorFromPoolIxData;
impl RemoveValidatorFromPoolIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != REMOVE_VALIDATOR_FROM_POOL_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_VALIDATOR_FROM_POOL_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_VALIDATOR_FROM_POOL_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn remove_validator_from_pool_ix_with_program_id(
    program_id: Pubkey,
    keys: RemoveValidatorFromPoolKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REMOVE_VALIDATOR_FROM_POOL_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: RemoveValidatorFromPoolIxData.try_to_vec()?,
    })
}
pub fn remove_validator_from_pool_ix(
    keys: RemoveValidatorFromPoolKeys,
) -> std::io::Result<Instruction> {
    remove_validator_from_pool_ix_with_program_id(crate::ID, keys)
}
pub fn remove_validator_from_pool_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RemoveValidatorFromPoolAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RemoveValidatorFromPoolKeys = accounts.into();
    let ix = remove_validator_from_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn remove_validator_from_pool_invoke(
    accounts: RemoveValidatorFromPoolAccounts<'_, '_>,
) -> ProgramResult {
    remove_validator_from_pool_invoke_with_program_id(crate::ID, accounts)
}
pub fn remove_validator_from_pool_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RemoveValidatorFromPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RemoveValidatorFromPoolKeys = accounts.into();
    let ix = remove_validator_from_pool_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn remove_validator_from_pool_invoke_signed(
    accounts: RemoveValidatorFromPoolAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    remove_validator_from_pool_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn remove_validator_from_pool_verify_account_keys(
    accounts: RemoveValidatorFromPoolAccounts<'_, '_>,
    keys: RemoveValidatorFromPoolKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.staker.key, &keys.staker),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.stake_account.key, &keys.stake_account),
        (
            accounts.transient_stake_account.key,
            &keys.transient_stake_account,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_validator_from_pool_verify_writable_privileges<'me, 'info>(
    accounts: RemoveValidatorFromPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.stake_pool,
        accounts.validator_list,
        accounts.stake_account,
        accounts.transient_stake_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn remove_validator_from_pool_verify_signer_privileges<'me, 'info>(
    accounts: RemoveValidatorFromPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn remove_validator_from_pool_verify_account_privileges<'me, 'info>(
    accounts: RemoveValidatorFromPoolAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    remove_validator_from_pool_verify_writable_privileges(accounts)?;
    remove_validator_from_pool_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct UpdateValidatorListBalanceAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Stake pool withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
    ///Validator list
    pub validator_list: &'me AccountInfo<'info>,
    ///Reserve stake account
    pub reserve_stake: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///Stake history sysvar
    pub stake_history: &'me AccountInfo<'info>,
    ///Stake program. N pairs of validator and transient stake accounts follow.
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateValidatorListBalanceKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Stake pool withdraw authority
    pub withdraw_authority: Pubkey,
    ///Validator list
    pub validator_list: Pubkey,
    ///Reserve stake account
    pub reserve_stake: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///Stake history sysvar
    pub stake_history: Pubkey,
    ///Stake program. N pairs of validator and transient stake accounts follow.
    pub stake_program: Pubkey,
}
impl From<UpdateValidatorListBalanceAccounts<'_, '_>> for UpdateValidatorListBalanceKeys {
    fn from(accounts: UpdateValidatorListBalanceAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            validator_list: *accounts.validator_list.key,
            reserve_stake: *accounts.reserve_stake.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<UpdateValidatorListBalanceKeys>
    for [AccountMeta; UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN]
{
    fn from(keys: UpdateValidatorListBalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN]>
    for UpdateValidatorListBalanceKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            withdraw_authority: pubkeys[1],
            validator_list: pubkeys[2],
            reserve_stake: pubkeys[3],
            clock: pubkeys[4],
            stake_history: pubkeys[5],
            stake_program: pubkeys[6],
        }
    }
}
impl<'info> From<UpdateValidatorListBalanceAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: UpdateValidatorListBalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.withdraw_authority.clone(),
            accounts.validator_list.clone(),
            accounts.reserve_stake.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN]>
    for UpdateValidatorListBalanceAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            withdraw_authority: &arr[1],
            validator_list: &arr[2],
            reserve_stake: &arr[3],
            clock: &arr[4],
            stake_history: &arr[5],
            stake_program: &arr[6],
        }
    }
}
pub const UPDATE_VALIDATOR_LIST_BALANCE_IX_DISCM: u8 = 6u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateValidatorListBalanceIxArgs {
    pub start_index: u32,
    pub no_merge: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateValidatorListBalanceIxData(pub UpdateValidatorListBalanceIxArgs);
impl From<UpdateValidatorListBalanceIxArgs> for UpdateValidatorListBalanceIxData {
    fn from(args: UpdateValidatorListBalanceIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateValidatorListBalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_VALIDATOR_LIST_BALANCE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_VALIDATOR_LIST_BALANCE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateValidatorListBalanceIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_VALIDATOR_LIST_BALANCE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_validator_list_balance_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateValidatorListBalanceKeys,
    args: UpdateValidatorListBalanceIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_VALIDATOR_LIST_BALANCE_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateValidatorListBalanceIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_validator_list_balance_ix(
    keys: UpdateValidatorListBalanceKeys,
    args: UpdateValidatorListBalanceIxArgs,
) -> std::io::Result<Instruction> {
    update_validator_list_balance_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_validator_list_balance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateValidatorListBalanceAccounts<'_, '_>,
    args: UpdateValidatorListBalanceIxArgs,
) -> ProgramResult {
    let keys: UpdateValidatorListBalanceKeys = accounts.into();
    let ix = update_validator_list_balance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_validator_list_balance_invoke(
    accounts: UpdateValidatorListBalanceAccounts<'_, '_>,
    args: UpdateValidatorListBalanceIxArgs,
) -> ProgramResult {
    update_validator_list_balance_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_validator_list_balance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateValidatorListBalanceAccounts<'_, '_>,
    args: UpdateValidatorListBalanceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateValidatorListBalanceKeys = accounts.into();
    let ix = update_validator_list_balance_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_validator_list_balance_invoke_signed(
    accounts: UpdateValidatorListBalanceAccounts<'_, '_>,
    args: UpdateValidatorListBalanceIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_validator_list_balance_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn update_validator_list_balance_verify_account_keys(
    accounts: UpdateValidatorListBalanceAccounts<'_, '_>,
    keys: UpdateValidatorListBalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.reserve_stake.key, &keys.reserve_stake),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_validator_list_balance_verify_writable_privileges<'me, 'info>(
    accounts: UpdateValidatorListBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.validator_list, accounts.reserve_stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_validator_list_balance_verify_account_privileges<'me, 'info>(
    accounts: UpdateValidatorListBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_validator_list_balance_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_STAKE_POOL_BALANCE_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct UpdateStakePoolBalanceAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Stake pool withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
    ///Validator list
    pub validator_list: &'me AccountInfo<'info>,
    ///Reserve stake account
    pub reserve_stake: &'me AccountInfo<'info>,
    ///Account to receive pool fee tokens
    pub manager_fee_account: &'me AccountInfo<'info>,
    ///Pool token mint.
    pub pool_mint: &'me AccountInfo<'info>,
    ///Pool token's token program.
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateStakePoolBalanceKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Stake pool withdraw authority
    pub withdraw_authority: Pubkey,
    ///Validator list
    pub validator_list: Pubkey,
    ///Reserve stake account
    pub reserve_stake: Pubkey,
    ///Account to receive pool fee tokens
    pub manager_fee_account: Pubkey,
    ///Pool token mint.
    pub pool_mint: Pubkey,
    ///Pool token's token program.
    pub token_program: Pubkey,
}
impl From<UpdateStakePoolBalanceAccounts<'_, '_>> for UpdateStakePoolBalanceKeys {
    fn from(accounts: UpdateStakePoolBalanceAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            validator_list: *accounts.validator_list.key,
            reserve_stake: *accounts.reserve_stake.key,
            manager_fee_account: *accounts.manager_fee_account.key,
            pool_mint: *accounts.pool_mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<UpdateStakePoolBalanceKeys> for [AccountMeta; UPDATE_STAKE_POOL_BALANCE_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateStakePoolBalanceKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_stake,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.manager_fee_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_STAKE_POOL_BALANCE_IX_ACCOUNTS_LEN]> for UpdateStakePoolBalanceKeys {
    fn from(pubkeys: [Pubkey; UPDATE_STAKE_POOL_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            withdraw_authority: pubkeys[1],
            validator_list: pubkeys[2],
            reserve_stake: pubkeys[3],
            manager_fee_account: pubkeys[4],
            pool_mint: pubkeys[5],
            token_program: pubkeys[6],
        }
    }
}
impl<'info> From<UpdateStakePoolBalanceAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_STAKE_POOL_BALANCE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: UpdateStakePoolBalanceAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.withdraw_authority.clone(),
            accounts.validator_list.clone(),
            accounts.reserve_stake.clone(),
            accounts.manager_fee_account.clone(),
            accounts.pool_mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_STAKE_POOL_BALANCE_IX_ACCOUNTS_LEN]>
    for UpdateStakePoolBalanceAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_STAKE_POOL_BALANCE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            withdraw_authority: &arr[1],
            validator_list: &arr[2],
            reserve_stake: &arr[3],
            manager_fee_account: &arr[4],
            pool_mint: &arr[5],
            token_program: &arr[6],
        }
    }
}
pub const UPDATE_STAKE_POOL_BALANCE_IX_DISCM: u8 = 7u8;
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateStakePoolBalanceIxData;
impl UpdateStakePoolBalanceIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != UPDATE_STAKE_POOL_BALANCE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_STAKE_POOL_BALANCE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_STAKE_POOL_BALANCE_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_stake_pool_balance_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateStakePoolBalanceKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_STAKE_POOL_BALANCE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: UpdateStakePoolBalanceIxData.try_to_vec()?,
    })
}
pub fn update_stake_pool_balance_ix(
    keys: UpdateStakePoolBalanceKeys,
) -> std::io::Result<Instruction> {
    update_stake_pool_balance_ix_with_program_id(crate::ID, keys)
}
pub fn update_stake_pool_balance_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateStakePoolBalanceAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpdateStakePoolBalanceKeys = accounts.into();
    let ix = update_stake_pool_balance_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_stake_pool_balance_invoke(
    accounts: UpdateStakePoolBalanceAccounts<'_, '_>,
) -> ProgramResult {
    update_stake_pool_balance_invoke_with_program_id(crate::ID, accounts)
}
pub fn update_stake_pool_balance_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateStakePoolBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateStakePoolBalanceKeys = accounts.into();
    let ix = update_stake_pool_balance_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_stake_pool_balance_invoke_signed(
    accounts: UpdateStakePoolBalanceAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_stake_pool_balance_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn update_stake_pool_balance_verify_account_keys(
    accounts: UpdateStakePoolBalanceAccounts<'_, '_>,
    keys: UpdateStakePoolBalanceKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.reserve_stake.key, &keys.reserve_stake),
        (accounts.manager_fee_account.key, &keys.manager_fee_account),
        (accounts.pool_mint.key, &keys.pool_mint),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_stake_pool_balance_verify_writable_privileges<'me, 'info>(
    accounts: UpdateStakePoolBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.stake_pool,
        accounts.validator_list,
        accounts.manager_fee_account,
        accounts.pool_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_stake_pool_balance_verify_account_privileges<'me, 'info>(
    accounts: UpdateStakePoolBalanceAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_stake_pool_balance_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct CleanupRemovedValidatorEntriesAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Validator list
    pub validator_list: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CleanupRemovedValidatorEntriesKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Validator list
    pub validator_list: Pubkey,
}
impl From<CleanupRemovedValidatorEntriesAccounts<'_, '_>> for CleanupRemovedValidatorEntriesKeys {
    fn from(accounts: CleanupRemovedValidatorEntriesAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            validator_list: *accounts.validator_list.key,
        }
    }
}
impl From<CleanupRemovedValidatorEntriesKeys>
    for [AccountMeta; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCOUNTS_LEN]
{
    fn from(keys: CleanupRemovedValidatorEntriesKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCOUNTS_LEN]>
    for CleanupRemovedValidatorEntriesKeys
{
    fn from(pubkeys: [Pubkey; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            validator_list: pubkeys[1],
        }
    }
}
impl<'info> From<CleanupRemovedValidatorEntriesAccounts<'_, 'info>>
    for [AccountInfo<'info>; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CleanupRemovedValidatorEntriesAccounts<'_, 'info>) -> Self {
        [accounts.stake_pool.clone(), accounts.validator_list.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCOUNTS_LEN]>
    for CleanupRemovedValidatorEntriesAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            stake_pool: &arr[0],
            validator_list: &arr[1],
        }
    }
}
pub const CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_DISCM: u8 = 8u8;
#[derive(Clone, Debug, PartialEq)]
pub struct CleanupRemovedValidatorEntriesIxData;
impl CleanupRemovedValidatorEntriesIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cleanup_removed_validator_entries_ix_with_program_id(
    program_id: Pubkey,
    keys: CleanupRemovedValidatorEntriesKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLEANUP_REMOVED_VALIDATOR_ENTRIES_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CleanupRemovedValidatorEntriesIxData.try_to_vec()?,
    })
}
pub fn cleanup_removed_validator_entries_ix(
    keys: CleanupRemovedValidatorEntriesKeys,
) -> std::io::Result<Instruction> {
    cleanup_removed_validator_entries_ix_with_program_id(crate::ID, keys)
}
pub fn cleanup_removed_validator_entries_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CleanupRemovedValidatorEntriesAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CleanupRemovedValidatorEntriesKeys = accounts.into();
    let ix = cleanup_removed_validator_entries_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cleanup_removed_validator_entries_invoke(
    accounts: CleanupRemovedValidatorEntriesAccounts<'_, '_>,
) -> ProgramResult {
    cleanup_removed_validator_entries_invoke_with_program_id(crate::ID, accounts)
}
pub fn cleanup_removed_validator_entries_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CleanupRemovedValidatorEntriesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CleanupRemovedValidatorEntriesKeys = accounts.into();
    let ix = cleanup_removed_validator_entries_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cleanup_removed_validator_entries_invoke_signed(
    accounts: CleanupRemovedValidatorEntriesAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cleanup_removed_validator_entries_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn cleanup_removed_validator_entries_verify_account_keys(
    accounts: CleanupRemovedValidatorEntriesAccounts<'_, '_>,
    keys: CleanupRemovedValidatorEntriesKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.validator_list.key, &keys.validator_list),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn cleanup_removed_validator_entries_verify_writable_privileges<'me, 'info>(
    accounts: CleanupRemovedValidatorEntriesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.validator_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cleanup_removed_validator_entries_verify_account_privileges<'me, 'info>(
    accounts: CleanupRemovedValidatorEntriesAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cleanup_removed_validator_entries_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const SET_MANAGER_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct SetManagerAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Current manager
    pub manager: &'me AccountInfo<'info>,
    ///New manager
    pub new_manager: &'me AccountInfo<'info>,
    ///New manager fee account
    pub new_manager_fee_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetManagerKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Current manager
    pub manager: Pubkey,
    ///New manager
    pub new_manager: Pubkey,
    ///New manager fee account
    pub new_manager_fee_account: Pubkey,
}
impl From<SetManagerAccounts<'_, '_>> for SetManagerKeys {
    fn from(accounts: SetManagerAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            manager: *accounts.manager.key,
            new_manager: *accounts.new_manager.key,
            new_manager_fee_account: *accounts.new_manager_fee_account.key,
        }
    }
}
impl From<SetManagerKeys> for [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetManagerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_manager_fee_account,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_MANAGER_IX_ACCOUNTS_LEN]> for SetManagerKeys {
    fn from(pubkeys: [Pubkey; SET_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            manager: pubkeys[1],
            new_manager: pubkeys[2],
            new_manager_fee_account: pubkeys[3],
        }
    }
}
impl<'info> From<SetManagerAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetManagerAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.manager.clone(),
            accounts.new_manager.clone(),
            accounts.new_manager_fee_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]>
    for SetManagerAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_MANAGER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            manager: &arr[1],
            new_manager: &arr[2],
            new_manager_fee_account: &arr[3],
        }
    }
}
pub const SET_MANAGER_IX_DISCM: u8 = 11u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SetManagerIxData;
impl SetManagerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_MANAGER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_MANAGER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_MANAGER_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_manager_ix_with_program_id(
    program_id: Pubkey,
    keys: SetManagerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_MANAGER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetManagerIxData.try_to_vec()?,
    })
}
pub fn set_manager_ix(keys: SetManagerKeys) -> std::io::Result<Instruction> {
    set_manager_ix_with_program_id(crate::ID, keys)
}
pub fn set_manager_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetManagerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetManagerKeys = accounts.into();
    let ix = set_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_manager_invoke(accounts: SetManagerAccounts<'_, '_>) -> ProgramResult {
    set_manager_invoke_with_program_id(crate::ID, accounts)
}
pub fn set_manager_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetManagerKeys = accounts.into();
    let ix = set_manager_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_manager_invoke_signed(
    accounts: SetManagerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_manager_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn set_manager_verify_account_keys(
    accounts: SetManagerAccounts<'_, '_>,
    keys: SetManagerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.manager.key, &keys.manager),
        (accounts.new_manager.key, &keys.new_manager),
        (
            accounts.new_manager_fee_account.key,
            &keys.new_manager_fee_account,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_manager_verify_writable_privileges<'me, 'info>(
    accounts: SetManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake_pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_manager_verify_signer_privileges<'me, 'info>(
    accounts: SetManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager, accounts.new_manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_manager_verify_account_privileges<'me, 'info>(
    accounts: SetManagerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_manager_verify_writable_privileges(accounts)?;
    set_manager_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Current manager
    pub manager: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetFeeKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Current manager
    pub manager: Pubkey,
}
impl From<SetFeeAccounts<'_, '_>> for SetFeeKeys {
    fn from(accounts: SetFeeAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            manager: *accounts.manager.key,
        }
    }
}
impl From<SetFeeKeys> for [AccountMeta; SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FEE_IX_ACCOUNTS_LEN]> for SetFeeKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            manager: pubkeys[1],
        }
    }
}
impl<'info> From<SetFeeAccounts<'_, 'info>> for [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: SetFeeAccounts<'_, 'info>) -> Self {
        [accounts.stake_pool.clone(), accounts.manager.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN]>
    for SetFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            manager: &arr[1],
        }
    }
}
pub const SET_FEE_IX_DISCM: u8 = 12u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetFeeIxArgs {
    pub fee: FeeType,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetFeeIxData(pub SetFeeIxArgs);
impl From<SetFeeIxArgs> for SetFeeIxData {
    fn from(args: SetFeeIxArgs) -> Self {
        Self(args)
    }
}
impl SetFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_FEE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_FEE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_FEE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFeeKeys,
    args: SetFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: SetFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_ix(keys: SetFeeKeys, args: SetFeeIxArgs) -> std::io::Result<Instruction> {
    set_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn set_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAccounts<'_, '_>,
    args: SetFeeIxArgs,
) -> ProgramResult {
    let keys: SetFeeKeys = accounts.into();
    let ix = set_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_fee_invoke(accounts: SetFeeAccounts<'_, '_>, args: SetFeeIxArgs) -> ProgramResult {
    set_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFeeAccounts<'_, '_>,
    args: SetFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFeeKeys = accounts.into();
    let ix = set_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_fee_invoke_signed(
    accounts: SetFeeAccounts<'_, '_>,
    args: SetFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_fee_verify_account_keys(
    accounts: SetFeeAccounts<'_, '_>,
    keys: SetFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.manager.key, &keys.manager),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_fee_verify_writable_privileges<'me, 'info>(
    accounts: SetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake_pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_fee_verify_signer_privileges<'me, 'info>(
    accounts: SetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_fee_verify_account_privileges<'me, 'info>(
    accounts: SetFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_fee_verify_writable_privileges(accounts)?;
    set_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_STAKER_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetStakerAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Current manager or staker
    pub signer: &'me AccountInfo<'info>,
    ///New staker pubkey
    pub new_staker: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetStakerKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Current manager or staker
    pub signer: Pubkey,
    ///New staker pubkey
    pub new_staker: Pubkey,
}
impl From<SetStakerAccounts<'_, '_>> for SetStakerKeys {
    fn from(accounts: SetStakerAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            signer: *accounts.signer.key,
            new_staker: *accounts.new_staker.key,
        }
    }
}
impl From<SetStakerKeys> for [AccountMeta; SET_STAKER_IX_ACCOUNTS_LEN] {
    fn from(keys: SetStakerKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.signer,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_staker,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_STAKER_IX_ACCOUNTS_LEN]> for SetStakerKeys {
    fn from(pubkeys: [Pubkey; SET_STAKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            signer: pubkeys[1],
            new_staker: pubkeys[2],
        }
    }
}
impl<'info> From<SetStakerAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_STAKER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetStakerAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.signer.clone(),
            accounts.new_staker.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_STAKER_IX_ACCOUNTS_LEN]>
    for SetStakerAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_STAKER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            signer: &arr[1],
            new_staker: &arr[2],
        }
    }
}
pub const SET_STAKER_IX_DISCM: u8 = 13u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SetStakerIxData;
impl SetStakerIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_STAKER_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_STAKER_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_STAKER_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_staker_ix_with_program_id(
    program_id: Pubkey,
    keys: SetStakerKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_STAKER_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetStakerIxData.try_to_vec()?,
    })
}
pub fn set_staker_ix(keys: SetStakerKeys) -> std::io::Result<Instruction> {
    set_staker_ix_with_program_id(crate::ID, keys)
}
pub fn set_staker_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetStakerAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetStakerKeys = accounts.into();
    let ix = set_staker_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_staker_invoke(accounts: SetStakerAccounts<'_, '_>) -> ProgramResult {
    set_staker_invoke_with_program_id(crate::ID, accounts)
}
pub fn set_staker_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetStakerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetStakerKeys = accounts.into();
    let ix = set_staker_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_staker_invoke_signed(
    accounts: SetStakerAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_staker_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn set_staker_verify_account_keys(
    accounts: SetStakerAccounts<'_, '_>,
    keys: SetStakerKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.signer.key, &keys.signer),
        (accounts.new_staker.key, &keys.new_staker),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_staker_verify_writable_privileges<'me, 'info>(
    accounts: SetStakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake_pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_staker_verify_signer_privileges<'me, 'info>(
    accounts: SetStakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.signer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_staker_verify_account_privileges<'me, 'info>(
    accounts: SetStakerAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_staker_verify_writable_privileges(accounts)?;
    set_staker_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_FUNDING_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetFundingAuthorityAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Current manager
    pub manager: &'me AccountInfo<'info>,
    ///New funding authority. If omitted, sets it to None
    pub new_funding_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetFundingAuthorityKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Current manager
    pub manager: Pubkey,
    ///New funding authority. If omitted, sets it to None
    pub new_funding_authority: Pubkey,
}
impl From<SetFundingAuthorityAccounts<'_, '_>> for SetFundingAuthorityKeys {
    fn from(accounts: SetFundingAuthorityAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            manager: *accounts.manager.key,
            new_funding_authority: *accounts.new_funding_authority.key,
        }
    }
}
impl From<SetFundingAuthorityKeys> for [AccountMeta; SET_FUNDING_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: SetFundingAuthorityKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.manager,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_funding_authority,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_FUNDING_AUTHORITY_IX_ACCOUNTS_LEN]> for SetFundingAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_FUNDING_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            manager: pubkeys[1],
            new_funding_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SetFundingAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_FUNDING_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetFundingAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.manager.clone(),
            accounts.new_funding_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FUNDING_AUTHORITY_IX_ACCOUNTS_LEN]>
    for SetFundingAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_FUNDING_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: &arr[0],
            manager: &arr[1],
            new_funding_authority: &arr[2],
        }
    }
}
pub const SET_FUNDING_AUTHORITY_IX_DISCM: u8 = 15u8;
#[derive(Clone, Debug, PartialEq)]
pub struct SetFundingAuthorityIxData;
impl SetFundingAuthorityIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != SET_FUNDING_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_FUNDING_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_FUNDING_AUTHORITY_IX_DISCM])
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn set_funding_authority_ix_with_program_id(
    program_id: Pubkey,
    keys: SetFundingAuthorityKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; SET_FUNDING_AUTHORITY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: SetFundingAuthorityIxData.try_to_vec()?,
    })
}
pub fn set_funding_authority_ix(keys: SetFundingAuthorityKeys) -> std::io::Result<Instruction> {
    set_funding_authority_ix_with_program_id(crate::ID, keys)
}
pub fn set_funding_authority_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetFundingAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    let keys: SetFundingAuthorityKeys = accounts.into();
    let ix = set_funding_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn set_funding_authority_invoke(
    accounts: SetFundingAuthorityAccounts<'_, '_>,
) -> ProgramResult {
    set_funding_authority_invoke_with_program_id(crate::ID, accounts)
}
pub fn set_funding_authority_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetFundingAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetFundingAuthorityKeys = accounts.into();
    let ix = set_funding_authority_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_funding_authority_invoke_signed(
    accounts: SetFundingAuthorityAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_funding_authority_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn set_funding_authority_verify_account_keys(
    accounts: SetFundingAuthorityAccounts<'_, '_>,
    keys: SetFundingAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.manager.key, &keys.manager),
        (
            accounts.new_funding_authority.key,
            &keys.new_funding_authority,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_funding_authority_verify_writable_privileges<'me, 'info>(
    accounts: SetFundingAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake_pool] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_funding_authority_verify_signer_privileges<'me, 'info>(
    accounts: SetFundingAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.manager] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_funding_authority_verify_account_privileges<'me, 'info>(
    accounts: SetFundingAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_funding_authority_verify_writable_privileges(accounts)?;
    set_funding_authority_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct IncreaseAdditionalValidatorStakeAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Current staker
    pub staker: &'me AccountInfo<'info>,
    ///Stake pool withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
    ///Validator list
    pub validator_list: &'me AccountInfo<'info>,
    ///Reserve stake account
    pub reserve_stake: &'me AccountInfo<'info>,
    ///Uninitialized ephemeral stake account to receive stake
    pub ephemeral_stake_account: &'me AccountInfo<'info>,
    ///Transient stake account
    pub transient_stake_account: &'me AccountInfo<'info>,
    ///Validator stake account
    pub validator_stake_account: &'me AccountInfo<'info>,
    ///Validator vote account to delegate to
    pub vote_account: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///Stake history sysvar
    pub stake_history: &'me AccountInfo<'info>,
    ///Stake config sysvar
    pub stake_config: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Stake program
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct IncreaseAdditionalValidatorStakeKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Current staker
    pub staker: Pubkey,
    ///Stake pool withdraw authority
    pub withdraw_authority: Pubkey,
    ///Validator list
    pub validator_list: Pubkey,
    ///Reserve stake account
    pub reserve_stake: Pubkey,
    ///Uninitialized ephemeral stake account to receive stake
    pub ephemeral_stake_account: Pubkey,
    ///Transient stake account
    pub transient_stake_account: Pubkey,
    ///Validator stake account
    pub validator_stake_account: Pubkey,
    ///Validator vote account to delegate to
    pub vote_account: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///Stake history sysvar
    pub stake_history: Pubkey,
    ///Stake config sysvar
    pub stake_config: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Stake program
    pub stake_program: Pubkey,
}
impl From<IncreaseAdditionalValidatorStakeAccounts<'_, '_>>
    for IncreaseAdditionalValidatorStakeKeys
{
    fn from(accounts: IncreaseAdditionalValidatorStakeAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            staker: *accounts.staker.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            validator_list: *accounts.validator_list.key,
            reserve_stake: *accounts.reserve_stake.key,
            ephemeral_stake_account: *accounts.ephemeral_stake_account.key,
            transient_stake_account: *accounts.transient_stake_account.key,
            validator_stake_account: *accounts.validator_stake_account.key,
            vote_account: *accounts.vote_account.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_config: *accounts.stake_config.key,
            system_program: *accounts.system_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<IncreaseAdditionalValidatorStakeKeys>
    for [AccountMeta; INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: IncreaseAdditionalValidatorStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ephemeral_stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transient_stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.validator_stake_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vote_account,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]>
    for IncreaseAdditionalValidatorStakeKeys
{
    fn from(pubkeys: [Pubkey; INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            staker: pubkeys[1],
            withdraw_authority: pubkeys[2],
            validator_list: pubkeys[3],
            reserve_stake: pubkeys[4],
            ephemeral_stake_account: pubkeys[5],
            transient_stake_account: pubkeys[6],
            validator_stake_account: pubkeys[7],
            vote_account: pubkeys[8],
            clock: pubkeys[9],
            stake_history: pubkeys[10],
            stake_config: pubkeys[11],
            system_program: pubkeys[12],
            stake_program: pubkeys[13],
        }
    }
}
impl<'info> From<IncreaseAdditionalValidatorStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: IncreaseAdditionalValidatorStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.staker.clone(),
            accounts.withdraw_authority.clone(),
            accounts.validator_list.clone(),
            accounts.reserve_stake.clone(),
            accounts.ephemeral_stake_account.clone(),
            accounts.transient_stake_account.clone(),
            accounts.validator_stake_account.clone(),
            accounts.vote_account.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_config.clone(),
            accounts.system_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]>
    for IncreaseAdditionalValidatorStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            stake_pool: &arr[0],
            staker: &arr[1],
            withdraw_authority: &arr[2],
            validator_list: &arr[3],
            reserve_stake: &arr[4],
            ephemeral_stake_account: &arr[5],
            transient_stake_account: &arr[6],
            validator_stake_account: &arr[7],
            vote_account: &arr[8],
            clock: &arr[9],
            stake_history: &arr[10],
            stake_config: &arr[11],
            system_program: &arr[12],
            stake_program: &arr[13],
        }
    }
}
pub const INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM: u8 = 19u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncreaseAdditionalValidatorStakeIxArgs {
    pub args: AdditionalValidatorStakeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct IncreaseAdditionalValidatorStakeIxData(pub IncreaseAdditionalValidatorStakeIxArgs);
impl From<IncreaseAdditionalValidatorStakeIxArgs> for IncreaseAdditionalValidatorStakeIxData {
    fn from(args: IncreaseAdditionalValidatorStakeIxArgs) -> Self {
        Self(args)
    }
}
impl IncreaseAdditionalValidatorStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(IncreaseAdditionalValidatorStakeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn increase_additional_validator_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: IncreaseAdditionalValidatorStakeKeys,
    args: IncreaseAdditionalValidatorStakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INCREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: IncreaseAdditionalValidatorStakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn increase_additional_validator_stake_ix(
    keys: IncreaseAdditionalValidatorStakeKeys,
    args: IncreaseAdditionalValidatorStakeIxArgs,
) -> std::io::Result<Instruction> {
    increase_additional_validator_stake_ix_with_program_id(crate::ID, keys, args)
}
pub fn increase_additional_validator_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseAdditionalValidatorStakeAccounts<'_, '_>,
    args: IncreaseAdditionalValidatorStakeIxArgs,
) -> ProgramResult {
    let keys: IncreaseAdditionalValidatorStakeKeys = accounts.into();
    let ix = increase_additional_validator_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn increase_additional_validator_stake_invoke(
    accounts: IncreaseAdditionalValidatorStakeAccounts<'_, '_>,
    args: IncreaseAdditionalValidatorStakeIxArgs,
) -> ProgramResult {
    increase_additional_validator_stake_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn increase_additional_validator_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: IncreaseAdditionalValidatorStakeAccounts<'_, '_>,
    args: IncreaseAdditionalValidatorStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: IncreaseAdditionalValidatorStakeKeys = accounts.into();
    let ix = increase_additional_validator_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn increase_additional_validator_stake_invoke_signed(
    accounts: IncreaseAdditionalValidatorStakeAccounts<'_, '_>,
    args: IncreaseAdditionalValidatorStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    increase_additional_validator_stake_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn increase_additional_validator_stake_verify_account_keys(
    accounts: IncreaseAdditionalValidatorStakeAccounts<'_, '_>,
    keys: IncreaseAdditionalValidatorStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.staker.key, &keys.staker),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.reserve_stake.key, &keys.reserve_stake),
        (
            accounts.ephemeral_stake_account.key,
            &keys.ephemeral_stake_account,
        ),
        (
            accounts.transient_stake_account.key,
            &keys.transient_stake_account,
        ),
        (
            accounts.validator_stake_account.key,
            &keys.validator_stake_account,
        ),
        (accounts.vote_account.key, &keys.vote_account),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.stake_config.key, &keys.stake_config),
        (accounts.system_program.key, &keys.system_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn increase_additional_validator_stake_verify_writable_privileges<'me, 'info>(
    accounts: IncreaseAdditionalValidatorStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.validator_list,
        accounts.reserve_stake,
        accounts.ephemeral_stake_account,
        accounts.transient_stake_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn increase_additional_validator_stake_verify_signer_privileges<'me, 'info>(
    accounts: IncreaseAdditionalValidatorStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn increase_additional_validator_stake_verify_account_privileges<'me, 'info>(
    accounts: IncreaseAdditionalValidatorStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    increase_additional_validator_stake_verify_writable_privileges(accounts)?;
    increase_additional_validator_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN: usize = 12;
#[derive(Copy, Clone, Debug)]
pub struct DecreaseAdditionalValidatorStakeAccounts<'me, 'info> {
    ///Stake pool
    pub stake_pool: &'me AccountInfo<'info>,
    ///Current staker
    pub staker: &'me AccountInfo<'info>,
    ///Stake pool withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
    ///Validator list
    pub validator_list: &'me AccountInfo<'info>,
    ///Reserve stake account
    pub reserve_stake: &'me AccountInfo<'info>,
    ///Validator stake account to split stake from
    pub validator_stake_account: &'me AccountInfo<'info>,
    ///Uninitialized ephemeral stake account to receive stake
    pub ephemeral_stake_account: &'me AccountInfo<'info>,
    ///Transient stake account
    pub transient_stake_account: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///Stake history sysvar
    pub stake_history: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Stake program
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DecreaseAdditionalValidatorStakeKeys {
    ///Stake pool
    pub stake_pool: Pubkey,
    ///Current staker
    pub staker: Pubkey,
    ///Stake pool withdraw authority
    pub withdraw_authority: Pubkey,
    ///Validator list
    pub validator_list: Pubkey,
    ///Reserve stake account
    pub reserve_stake: Pubkey,
    ///Validator stake account to split stake from
    pub validator_stake_account: Pubkey,
    ///Uninitialized ephemeral stake account to receive stake
    pub ephemeral_stake_account: Pubkey,
    ///Transient stake account
    pub transient_stake_account: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///Stake history sysvar
    pub stake_history: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Stake program
    pub stake_program: Pubkey,
}
impl From<DecreaseAdditionalValidatorStakeAccounts<'_, '_>>
    for DecreaseAdditionalValidatorStakeKeys
{
    fn from(accounts: DecreaseAdditionalValidatorStakeAccounts) -> Self {
        Self {
            stake_pool: *accounts.stake_pool.key,
            staker: *accounts.staker.key,
            withdraw_authority: *accounts.withdraw_authority.key,
            validator_list: *accounts.validator_list.key,
            reserve_stake: *accounts.reserve_stake.key,
            validator_stake_account: *accounts.validator_stake_account.key,
            ephemeral_stake_account: *accounts.ephemeral_stake_account.key,
            transient_stake_account: *accounts.transient_stake_account.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            system_program: *accounts.system_program.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<DecreaseAdditionalValidatorStakeKeys>
    for [AccountMeta; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: DecreaseAdditionalValidatorStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake_pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staker,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.validator_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.reserve_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.validator_stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.ephemeral_stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.transient_stake_account,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]>
    for DecreaseAdditionalValidatorStakeKeys
{
    fn from(pubkeys: [Pubkey; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_pool: pubkeys[0],
            staker: pubkeys[1],
            withdraw_authority: pubkeys[2],
            validator_list: pubkeys[3],
            reserve_stake: pubkeys[4],
            validator_stake_account: pubkeys[5],
            ephemeral_stake_account: pubkeys[6],
            transient_stake_account: pubkeys[7],
            clock: pubkeys[8],
            stake_history: pubkeys[9],
            system_program: pubkeys[10],
            stake_program: pubkeys[11],
        }
    }
}
impl<'info> From<DecreaseAdditionalValidatorStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DecreaseAdditionalValidatorStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_pool.clone(),
            accounts.staker.clone(),
            accounts.withdraw_authority.clone(),
            accounts.validator_list.clone(),
            accounts.reserve_stake.clone(),
            accounts.validator_stake_account.clone(),
            accounts.ephemeral_stake_account.clone(),
            accounts.transient_stake_account.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.system_program.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN]>
    for DecreaseAdditionalValidatorStakeAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            stake_pool: &arr[0],
            staker: &arr[1],
            withdraw_authority: &arr[2],
            validator_list: &arr[3],
            reserve_stake: &arr[4],
            validator_stake_account: &arr[5],
            ephemeral_stake_account: &arr[6],
            transient_stake_account: &arr[7],
            clock: &arr[8],
            stake_history: &arr[9],
            system_program: &arr[10],
            stake_program: &arr[11],
        }
    }
}
pub const DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM: u8 = 20u8;
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecreaseAdditionalValidatorStakeIxArgs {
    pub args: AdditionalValidatorStakeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DecreaseAdditionalValidatorStakeIxData(pub DecreaseAdditionalValidatorStakeIxArgs);
impl From<DecreaseAdditionalValidatorStakeIxArgs> for DecreaseAdditionalValidatorStakeIxData {
    fn from(args: DecreaseAdditionalValidatorStakeIxArgs) -> Self {
        Self(args)
    }
}
impl DecreaseAdditionalValidatorStakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm_buf = [0u8; 1];
        reader.read_exact(&mut maybe_discm_buf)?;
        let maybe_discm = maybe_discm_buf[0];
        if maybe_discm != DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DecreaseAdditionalValidatorStakeIxArgs::deserialize(
            &mut reader,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn decrease_additional_validator_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: DecreaseAdditionalValidatorStakeKeys,
    args: DecreaseAdditionalValidatorStakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DECREASE_ADDITIONAL_VALIDATOR_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: DecreaseAdditionalValidatorStakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn decrease_additional_validator_stake_ix(
    keys: DecreaseAdditionalValidatorStakeKeys,
    args: DecreaseAdditionalValidatorStakeIxArgs,
) -> std::io::Result<Instruction> {
    decrease_additional_validator_stake_ix_with_program_id(crate::ID, keys, args)
}
pub fn decrease_additional_validator_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseAdditionalValidatorStakeAccounts<'_, '_>,
    args: DecreaseAdditionalValidatorStakeIxArgs,
) -> ProgramResult {
    let keys: DecreaseAdditionalValidatorStakeKeys = accounts.into();
    let ix = decrease_additional_validator_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn decrease_additional_validator_stake_invoke(
    accounts: DecreaseAdditionalValidatorStakeAccounts<'_, '_>,
    args: DecreaseAdditionalValidatorStakeIxArgs,
) -> ProgramResult {
    decrease_additional_validator_stake_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn decrease_additional_validator_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DecreaseAdditionalValidatorStakeAccounts<'_, '_>,
    args: DecreaseAdditionalValidatorStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DecreaseAdditionalValidatorStakeKeys = accounts.into();
    let ix = decrease_additional_validator_stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn decrease_additional_validator_stake_invoke_signed(
    accounts: DecreaseAdditionalValidatorStakeAccounts<'_, '_>,
    args: DecreaseAdditionalValidatorStakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    decrease_additional_validator_stake_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn decrease_additional_validator_stake_verify_account_keys(
    accounts: DecreaseAdditionalValidatorStakeAccounts<'_, '_>,
    keys: DecreaseAdditionalValidatorStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake_pool.key, &keys.stake_pool),
        (accounts.staker.key, &keys.staker),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
        (accounts.validator_list.key, &keys.validator_list),
        (accounts.reserve_stake.key, &keys.reserve_stake),
        (
            accounts.validator_stake_account.key,
            &keys.validator_stake_account,
        ),
        (
            accounts.ephemeral_stake_account.key,
            &keys.ephemeral_stake_account,
        ),
        (
            accounts.transient_stake_account.key,
            &keys.transient_stake_account,
        ),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.system_program.key, &keys.system_program),
        (accounts.stake_program.key, &keys.stake_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn decrease_additional_validator_stake_verify_writable_privileges<'me, 'info>(
    accounts: DecreaseAdditionalValidatorStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.validator_list,
        accounts.reserve_stake,
        accounts.validator_stake_account,
        accounts.ephemeral_stake_account,
        accounts.transient_stake_account,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn decrease_additional_validator_stake_verify_signer_privileges<'me, 'info>(
    accounts: DecreaseAdditionalValidatorStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.staker] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn decrease_additional_validator_stake_verify_account_privileges<'me, 'info>(
    accounts: DecreaseAdditionalValidatorStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    decrease_additional_validator_stake_verify_writable_privileges(accounts)?;
    decrease_additional_validator_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
