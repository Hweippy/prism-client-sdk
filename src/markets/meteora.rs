use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::{SPL_MEMO, SPL_TOKEN};

use super::{push_ro, push_w};

pub(crate) const METEORA_DLMM: Pubkey = Pubkey::from_str_const("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");
pub(crate) const METEORA_DLMM_EVENT_AUTHORITY: Pubkey =
    Pubkey::from_str_const("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6");
pub(crate) const METEORA_DAMM_V2: Pubkey = Pubkey::from_str_const("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG");
pub(crate) const METEORA_DAMM_V2_POOL_AUTHORITY: Pubkey =
    Pubkey::from_str_const("HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC");
pub(crate) const METEORA_DAMM_V2_EVENT_AUTHORITY: Pubkey =
    Pubkey::from_str_const("3rmHSu74h1ZcmAisVcWerTCiRDQbUrBKmcwptYGjHfet");
pub(crate) const METEORA_POOLS: Pubkey = Pubkey::from_str_const("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeteoraDlmmAccounts {
    pub lb_pair: Pubkey,
    pub bin_array_bitmap_ext: Pubkey,
    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,
    pub token_x_mint: Pubkey,
    pub token_y_mint: Pubkey,
    pub oracle: Pubkey,
    /// Optional host fee receiver. When `None`, the SDK emits the documented
    /// no-host-fee dummy account, the DLMM program id.
    pub host_fee_in: Option<Pubkey>,
    pub token_x_program: Pubkey,
    pub token_y_program: Pubkey,
    pub bin_array_prev: Pubkey,
    pub bin_array_cur: Pubkey,
    pub bin_array_next: Pubkey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeteoraDammV2Accounts {
    pub pool: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_program: Pubkey,
    pub token_b_program: Pubkey,
    /// Slot 10. Use `Some(SYSVAR_INSTRUCTIONS)` for rate-limiter pools. When
    /// `None`, the SDK emits the documented non-rate-limiter program sentinel.
    pub instructions_sysvar_or_program_sentinel: Option<Pubkey>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeteoraPoolsAccounts {
    pub pool: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub protocol_fee_a: Pubkey,
    pub protocol_fee_b: Pubkey,
    pub vault_program: Pubkey,
}

pub(super) fn append_dlmm(out: &mut Vec<AccountMeta>, accounts: MeteoraDlmmAccounts, token_2022: bool) {
    push_w(out, accounts.lb_pair);
    push_w(out, accounts.bin_array_bitmap_ext);
    push_w(out, accounts.reserve_x);
    push_w(out, accounts.reserve_y);
    push_ro(out, accounts.token_x_mint);
    push_ro(out, accounts.token_y_mint);
    push_w(out, accounts.oracle);
    push_w(out, accounts.host_fee_in.unwrap_or(METEORA_DLMM));
    push_ro(out, accounts.token_x_program);
    push_ro(out, accounts.token_y_program);
    if token_2022 {
        push_ro(out, SPL_MEMO);
    }
    push_ro(out, METEORA_DLMM_EVENT_AUTHORITY);
    push_ro(out, METEORA_DLMM);
    push_w(out, accounts.bin_array_prev);
    push_w(out, accounts.bin_array_cur);
    push_w(out, accounts.bin_array_next);
}

pub(super) fn append_damm_v2(out: &mut Vec<AccountMeta>, accounts: MeteoraDammV2Accounts) {
    push_ro(out, METEORA_DAMM_V2_POOL_AUTHORITY);
    push_w(out, accounts.pool);
    push_w(out, accounts.token_a_vault);
    push_w(out, accounts.token_b_vault);
    push_ro(out, accounts.token_a_mint);
    push_ro(out, accounts.token_b_mint);
    push_ro(out, accounts.token_a_program);
    push_ro(out, accounts.token_b_program);
    push_ro(out, METEORA_DAMM_V2_EVENT_AUTHORITY);
    push_ro(out, METEORA_DAMM_V2);
    push_ro(
        out,
        accounts
            .instructions_sysvar_or_program_sentinel
            .unwrap_or(METEORA_DAMM_V2),
    );
}

pub(super) fn append_pools(out: &mut Vec<AccountMeta>, accounts: MeteoraPoolsAccounts) {
    push_w(out, accounts.pool);
    push_w(out, accounts.a_vault);
    push_w(out, accounts.b_vault);
    push_w(out, accounts.a_token_vault);
    push_w(out, accounts.b_token_vault);
    push_w(out, accounts.a_vault_lp_mint);
    push_w(out, accounts.b_vault_lp_mint);
    push_w(out, accounts.a_vault_lp);
    push_w(out, accounts.b_vault_lp);
    push_w(out, accounts.protocol_fee_a);
    push_w(out, accounts.protocol_fee_b);
    push_ro(out, accounts.vault_program);
    push_ro(out, SPL_TOKEN);
    push_ro(out, METEORA_POOLS);
}
