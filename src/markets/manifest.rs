use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::SYSTEM_PROGRAM;

use super::{push_ro, push_w};

pub(crate) const MANIFEST: Pubkey = Pubkey::from_str_const("MNFSTqtC93rEfYHB6hF82sKdZpUDFWkViLByLd1k1Ms");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestAccounts {
    pub market: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
}

pub(super) fn append(out: &mut Vec<AccountMeta>, accounts: ManifestAccounts) {
    push_w(out, accounts.market);
    push_w(out, accounts.base_vault);
    push_w(out, accounts.quote_vault);
    push_ro(out, accounts.base_mint);
    push_ro(out, accounts.quote_mint);
    push_ro(out, SYSTEM_PROGRAM);
    push_ro(out, MANIFEST);
}
