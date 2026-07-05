use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::{SPL_MEMO, SPL_TOKEN};

use super::{push_ro, push_w};

pub(crate) const ORCA_WHIRLPOOL: Pubkey = Pubkey::from_str_const("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrcaWhirlpoolAccounts {
    pub whirlpool: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub tick_array_prev: Pubkey,
    pub tick_array_cur: Pubkey,
    pub tick_array_next: Pubkey,
    pub oracle: Pubkey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrcaWhirlpoolT22Accounts {
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub whirlpool: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub tick_array_prev: Pubkey,
    pub tick_array_cur: Pubkey,
    pub tick_array_next: Pubkey,
    pub oracle: Pubkey,
}

pub(super) fn append_whirlpool(out: &mut Vec<AccountMeta>, accounts: OrcaWhirlpoolAccounts) {
    push_ro(out, SPL_TOKEN);
    push_w(out, accounts.whirlpool);
    push_w(out, accounts.vault_a);
    push_w(out, accounts.vault_b);
    push_w(out, accounts.tick_array_prev);
    push_w(out, accounts.tick_array_cur);
    push_w(out, accounts.tick_array_next);
    push_w(out, accounts.oracle);
    push_ro(out, ORCA_WHIRLPOOL);
}

pub(super) fn append_whirlpool_t22(out: &mut Vec<AccountMeta>, accounts: OrcaWhirlpoolT22Accounts) {
    push_ro(out, accounts.token_program_a);
    push_ro(out, accounts.token_program_b);
    push_ro(out, SPL_MEMO);
    push_w(out, accounts.whirlpool);
    push_ro(out, accounts.token_mint_a);
    push_ro(out, accounts.token_mint_b);
    push_w(out, accounts.vault_a);
    push_w(out, accounts.vault_b);
    push_w(out, accounts.tick_array_prev);
    push_w(out, accounts.tick_array_cur);
    push_w(out, accounts.tick_array_next);
    push_w(out, accounts.oracle);
    push_ro(out, ORCA_WHIRLPOOL);
}
