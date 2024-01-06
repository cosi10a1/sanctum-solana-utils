use solana_program::program_error::ProgramError;
use solana_readonly_account::{ReadonlyAccountData, ReadonlyAccountPubkey};
use spl_token_interface::{BurnCheckedKeys, BurnKeys};

use crate::{InitializedTokenAccount, ReadonlyTokenAccount};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BurnFreeAccounts<A> {
    pub token_account: A,
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> BurnFreeAccounts<A> {
    pub fn resolve(&self) -> Result<BurnKeys, ProgramError> {
        let t = self.initialized_token_account()?;
        Ok(BurnKeys {
            token_account: *self.token_account.pubkey(),
            authority: t.token_account_authority(),
            mint: t.token_account_mint(),
        })
    }

    pub fn resolve_checked(&self) -> Result<BurnCheckedKeys, ProgramError> {
        let t = self.initialized_token_account()?;
        Ok(BurnCheckedKeys {
            token_account: *self.token_account.pubkey(),
            authority: t.token_account_authority(),
            mint: t.token_account_mint(),
        })
    }

    fn initialized_token_account(&self) -> Result<InitializedTokenAccount<&A>, ProgramError> {
        ReadonlyTokenAccount(&self.token_account)
            .try_into_valid()?
            .try_into_initialized()
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<BurnFreeAccounts<A>> for BurnKeys {
    type Error = ProgramError;

    fn try_from(value: BurnFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve()
    }
}

impl<A: ReadonlyAccountData + ReadonlyAccountPubkey> TryFrom<BurnFreeAccounts<A>>
    for BurnCheckedKeys
{
    type Error = ProgramError;

    fn try_from(value: BurnFreeAccounts<A>) -> Result<Self, Self::Error> {
        value.resolve_checked()
    }
}
