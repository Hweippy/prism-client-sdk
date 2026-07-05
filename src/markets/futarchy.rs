use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use super::{push_ro, push_w};

pub(crate) const FUTARCHY: Pubkey = Pubkey::from_str_const("FUTARELBfJfQ8RDGhg1wdhddq1odMAJUePHFuBYfUxKq");
pub(crate) const FUTARCHY_EVENT_AUTHORITY: Pubkey =
    Pubkey::from_str_const("DGEympSS4qLvdr9r3uGHTfACdN8snShk4iGdJtZPxuBC");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutarchySpotAccounts {
    pub dao: Pubkey,
    pub amm_base_vault: Pubkey,
    pub amm_quote_vault: Pubkey,
}

pub(super) fn append_spot(out: &mut Vec<AccountMeta>, accounts: FutarchySpotAccounts) {
    push_w(out, accounts.dao);
    push_w(out, accounts.amm_base_vault);
    push_w(out, accounts.amm_quote_vault);
    push_ro(out, FUTARCHY_EVENT_AUTHORITY);
    push_ro(out, FUTARCHY);
}
