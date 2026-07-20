use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::SYSVAR_INSTRUCTIONS;

use super::{push_ro, push_w};

pub const ZEROFI: Pubkey = Pubkey::from_str_const("ZERor4xhbUycZ6gb9ntrhqscUcZmAbQDjEAtCf4hbZY");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroFiAccounts {
    pub pool: Pubkey,
    pub oracle: Pubkey,
    pub side_a: Pubkey,
    pub vault_a: Pubkey,
    pub side_b: Pubkey,
    pub vault_b: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
}

pub(super) fn append(out: &mut Vec<AccountMeta>, accounts: ZeroFiAccounts) {
    push_w(out, accounts.pool);
    push_ro(out, accounts.oracle);
    push_w(out, accounts.side_a);
    push_w(out, accounts.vault_a);
    push_w(out, accounts.side_b);
    push_w(out, accounts.vault_b);
    push_ro(out, accounts.mint_a);
    push_ro(out, accounts.mint_b);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, ZEROFI);
}
