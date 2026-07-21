use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::{SPL_TOKEN, SPL_TOKEN_2022, SYSVAR_INSTRUCTIONS};

use super::{push_ro, push_w};

pub(crate) const GOONFI_V2: Pubkey = Pubkey::from_str_const("goonuddtQRrWqqn5nFyczVKaie28f3kDkHWkHtURSLE");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GoonfiV2Accounts {
    pub pair: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub side_price: Pubkey,
    pub global_state: Pubkey,
}

pub(super) fn append_v2(out: &mut Vec<AccountMeta>, accounts: GoonfiV2Accounts) {
    push_w(out, accounts.pair);
    push_w(out, accounts.vault_a);
    push_w(out, accounts.vault_b);
    push_ro(out, accounts.mint_a);
    push_ro(out, accounts.mint_b);
    push_ro(out, accounts.side_price);
    push_w(out, accounts.global_state);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, SPL_TOKEN);
    push_ro(out, GOONFI_V2);
}

pub(super) fn append_v2_t22(out: &mut Vec<AccountMeta>, accounts: GoonfiV2Accounts) {
    push_w(out, accounts.pair);
    push_w(out, accounts.vault_a);
    push_w(out, accounts.vault_b);
    push_ro(out, accounts.mint_a);
    push_ro(out, accounts.mint_b);
    push_ro(out, accounts.side_price);
    push_w(out, accounts.global_state);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, SPL_TOKEN);
    push_ro(out, SPL_TOKEN_2022);
    push_ro(out, GOONFI_V2);
}
