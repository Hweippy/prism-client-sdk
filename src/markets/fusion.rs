use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::SPL_MEMO;

use super::{push_ro, push_w};

pub(crate) const FUSION: Pubkey = Pubkey::from_str_const("fUSioN9YKKSa3CUC2YUc4tPkHJ5Y6XW1yz8y6F7qWz9");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FusionAccounts {
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub pool: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_array_cur: Pubkey,
    pub tick_array_above_1: Pubkey,
    pub tick_array_above_2: Pubkey,
    pub tick_array_below_1: Pubkey,
    pub tick_array_below_2: Pubkey,
}

pub(super) fn append(out: &mut Vec<AccountMeta>, accounts: FusionAccounts) {
    push_ro(out, accounts.token_program_a);
    push_ro(out, accounts.token_program_b);
    push_ro(out, SPL_MEMO);
    push_w(out, accounts.pool);
    push_ro(out, accounts.token_mint_a);
    push_ro(out, accounts.token_mint_b);
    push_w(out, accounts.token_vault_a);
    push_w(out, accounts.token_vault_b);
    push_w(out, accounts.tick_array_cur);
    push_w(out, accounts.tick_array_above_1);
    push_w(out, accounts.tick_array_above_2);
    push_w(out, accounts.tick_array_below_1);
    push_w(out, accounts.tick_array_below_2);
    push_ro(out, FUSION);
}
