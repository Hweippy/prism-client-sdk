use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::SYSVAR_INSTRUCTIONS;

use super::{push_ro, push_w};

pub const TESSERA: Pubkey = Pubkey::from_str_const("TessVdML9pBGgG9yGks7o4HewRaXVAMuoVj4x83GLQH");
pub const TESSERA_BAT: Pubkey = Pubkey::from_str_const("BAT1Ndpu5gbLTp2AZkSXP79LJBZfCH4B3zGhi6LtvdhK");
pub const TESSERA_TICK: Pubkey = Pubkey::from_str_const("4cG31VNF9TzFinNc7BmnjhFvGjxkY3sCETVMtMgbrhPs");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TesseraAccounts {
    pub global_authority: Pubkey,
    pub pool: Pubkey,
    pub mint_a_vault: Pubkey,
    pub mint_b_vault: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
}

pub(super) fn append(out: &mut Vec<AccountMeta>, accounts: TesseraAccounts) {
    push_ro(out, accounts.global_authority);
    push_w(out, accounts.pool);
    push_w(out, accounts.mint_a_vault);
    push_w(out, accounts.mint_b_vault);
    push_ro(out, accounts.mint_a);
    push_ro(out, accounts.mint_b);
    push_ro(out, SYSVAR_INSTRUCTIONS);
    push_ro(out, TESSERA);
    push_ro(out, TESSERA_BAT);
    push_ro(out, TESSERA_TICK);
}
