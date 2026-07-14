use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::{constants::SYSVAR_INSTRUCTIONS, PROGRAM_ID};

use super::{push_ro, push_w};

pub const BISONFI: Pubkey = Pubkey::from_str_const("BiSoNHVpsVZW2F7rx2eQ59yQwKxzU5NvBcmKshCSUypi");
pub const BISONFI_TRAILING_ACCOUNT: Pubkey =
    Pubkey::from_str_const("J1to1yufRnoWn81KYg1XkTWzmKjnYSnmE2VY8DGUJ9Qv");

/// Accounts for Prism's BisonFi market adapter.
///
/// Prism currently supports only the pool's canonical mint A to mint B
/// direction. Callers must not use this shape for mint B to mint A routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BisonFiAccounts {
    pub pool: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
}

pub(super) fn append(out: &mut Vec<AccountMeta>, accounts: BisonFiAccounts) {
    push_w(out, accounts.pool);
    push_w(out, accounts.vault_a);
    push_w(out, accounts.vault_b);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, BISONFI_TRAILING_ACCOUNT);
    push_ro(out, BISONFI);
    push_ro(out, PROGRAM_ID);
}
