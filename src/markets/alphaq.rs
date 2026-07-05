use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::{SPL_TOKEN, SPL_TOKEN_2022, SYSVAR_INSTRUCTIONS};

use super::{push_ro, push_w};

pub(crate) const ALPHAQ: Pubkey = Pubkey::from_str_const("ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlphaQAccounts {
    pub pool: Pubkey,
    pub market_stats: Pubkey,
    pub vault_left: Pubkey,
    pub vault_right: Pubkey,
    pub mint_left: Pubkey,
    pub mint_right: Pubkey,
}

pub(super) fn append(out: &mut Vec<AccountMeta>, accounts: AlphaQAccounts, token_2022: bool) {
    push_w(out, accounts.pool);
    push_w(out, accounts.market_stats);
    push_w(out, accounts.vault_left);
    push_w(out, accounts.vault_right);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, SPL_TOKEN);
    push_ro(out, accounts.mint_left);
    push_ro(out, accounts.mint_right);
    push_ro(out, ALPHAQ);
    if token_2022 {
        push_ro(out, SPL_TOKEN_2022);
    }
}
