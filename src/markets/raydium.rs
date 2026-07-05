use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use super::{append_clmm_base, append_clmm_t22_base, push_ro, push_w};

pub(crate) const RAYDIUM_V4: Pubkey = Pubkey::from_str_const("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
pub(crate) const RAYDIUM_V4_AUTHORITY: Pubkey =
    Pubkey::from_str_const("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1");
pub(crate) const RAYDIUM_CP: Pubkey = Pubkey::from_str_const("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
pub(crate) const RAYDIUM_CP_VAULT_AUTHORITY: Pubkey =
    Pubkey::from_str_const("GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL");
pub(crate) const RAYDIUM_CLMM: Pubkey = Pubkey::from_str_const("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RaydiumV4Accounts {
    pub pool_state: Pubkey,
    pub coin_vault: Pubkey,
    pub pc_vault: Pubkey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RaydiumCpAccounts {
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub observation_state: Pubkey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RaydiumClmmAccounts {
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_state: Pubkey,
    pub tick_array_bitmap_ext: Pubkey,
    pub tick_array_prev: Pubkey,
    /// May be the Raydium CLMM program id when the mathematical current
    /// tick-array PDA does not exist and prev/next carry the first valid arrays.
    pub tick_array_cur: Pubkey,
    pub tick_array_next: Pubkey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RaydiumClmmT22Accounts {
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_state: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub tick_array_bitmap_ext: Pubkey,
    pub tick_array_prev: Pubkey,
    /// May be the Raydium CLMM program id when the mathematical current
    /// tick-array PDA does not exist and prev/next carry the first valid arrays.
    pub tick_array_cur: Pubkey,
    pub tick_array_next: Pubkey,
}

pub(super) fn append_v4(out: &mut Vec<AccountMeta>, accounts: RaydiumV4Accounts) {
    push_w(out, accounts.pool_state);
    push_ro(out, RAYDIUM_V4_AUTHORITY);
    push_w(out, accounts.coin_vault);
    push_w(out, accounts.pc_vault);
    push_ro(out, RAYDIUM_V4);
}

pub(super) fn append_cp(out: &mut Vec<AccountMeta>, accounts: RaydiumCpAccounts) {
    push_w(out, accounts.pool_state);
    push_ro(out, accounts.amm_config);
    push_ro(out, RAYDIUM_CP_VAULT_AUTHORITY);
    push_w(out, accounts.token_0_vault);
    push_w(out, accounts.token_1_vault);
    push_ro(out, accounts.token_0_mint);
    push_ro(out, accounts.token_1_mint);
    push_w(out, accounts.observation_state);
    push_ro(out, RAYDIUM_CP);
}

pub(super) fn append_clmm(out: &mut Vec<AccountMeta>, accounts: RaydiumClmmAccounts) {
    append_clmm_base(
        out,
        accounts.pool_state,
        accounts.amm_config,
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.observation_state,
        RAYDIUM_CLMM,
        accounts.tick_array_bitmap_ext,
        accounts.tick_array_prev,
        accounts.tick_array_cur,
        accounts.tick_array_next,
    );
}

pub(super) fn append_clmm_t22(out: &mut Vec<AccountMeta>, accounts: RaydiumClmmT22Accounts) {
    append_clmm_t22_base(
        out,
        accounts.pool_state,
        accounts.amm_config,
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.observation_state,
        accounts.token_mint_0,
        accounts.token_mint_1,
        RAYDIUM_CLMM,
        accounts.tick_array_bitmap_ext,
        accounts.tick_array_prev,
        accounts.tick_array_cur,
        accounts.tick_array_next,
    );
}
