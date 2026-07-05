use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::SYSVAR_INSTRUCTIONS;

use super::{push_ro, push_w};

pub(crate) const SOLFI_V2: Pubkey = Pubkey::from_str_const("SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolfiV2Accounts {
    pub pair: Pubkey,
    pub oracle: Pubkey,
    pub global_config: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
}

pub(super) fn append_v2(out: &mut Vec<AccountMeta>, accounts: SolfiV2Accounts) {
    push_w(out, accounts.pair);
    push_ro(out, accounts.oracle);
    push_ro(out, accounts.global_config);
    push_w(out, accounts.base_vault);
    push_w(out, accounts.quote_vault);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, accounts.base_mint);
    push_ro(out, accounts.quote_mint);
    push_ro(out, accounts.base_token_program);
    push_ro(out, accounts.quote_token_program);
    push_ro(out, SOLFI_V2);
}
