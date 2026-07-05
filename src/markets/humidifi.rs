use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::{SYSVAR_CLOCK, SYSVAR_INSTRUCTIONS};

use super::{push_ro, push_w};

pub(crate) const HUMIDIFI: Pubkey = Pubkey::from_str_const("9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HumidifiSwapV2Accounts {
    pub pool: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub side_table: Pubkey,
    pub jito_vote: Pubkey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HumidifiSwapAccounts {
    pub pool: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub jito_vote: Pubkey,
}

pub(super) fn append_swap_v2(out: &mut Vec<AccountMeta>, accounts: HumidifiSwapV2Accounts) {
    push_w(out, accounts.pool);
    push_w(out, accounts.base_vault);
    push_w(out, accounts.quote_vault);
    push_ro(out, SYSVAR_CLOCK);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, accounts.base_mint);
    push_ro(out, accounts.quote_mint);
    push_w(out, accounts.side_table);
    push_ro(out, accounts.jito_vote);
    push_ro(out, HUMIDIFI);
}

pub(super) fn append_swap(out: &mut Vec<AccountMeta>, accounts: HumidifiSwapAccounts) {
    push_w(out, accounts.pool);
    push_w(out, accounts.base_vault);
    push_w(out, accounts.quote_vault);
    push_ro(out, SYSVAR_CLOCK);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, accounts.jito_vote);
    push_ro(out, HUMIDIFI);
}
