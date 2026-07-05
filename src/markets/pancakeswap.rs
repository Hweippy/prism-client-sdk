use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use super::{append_clmm_base, append_clmm_t22_base};

pub(crate) const PANCAKESWAP: Pubkey = Pubkey::from_str_const("HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PancakeswapAccounts {
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_state: Pubkey,
    pub tick_array_bitmap_ext: Pubkey,
    pub tick_array_prev: Pubkey,
    /// May be the PancakeSwap program id when the mathematical current
    /// tick-array PDA does not exist and prev/next carry the first valid arrays.
    pub tick_array_cur: Pubkey,
    pub tick_array_next: Pubkey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PancakeswapT22Accounts {
    pub pool_state: Pubkey,
    pub amm_config: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_state: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub tick_array_bitmap_ext: Pubkey,
    pub tick_array_prev: Pubkey,
    /// May be the PancakeSwap program id when the mathematical current
    /// tick-array PDA does not exist and prev/next carry the first valid arrays.
    pub tick_array_cur: Pubkey,
    pub tick_array_next: Pubkey,
}

pub(super) fn append(out: &mut Vec<AccountMeta>, accounts: PancakeswapAccounts) {
    append_clmm_base(
        out,
        accounts.pool_state,
        accounts.amm_config,
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.observation_state,
        PANCAKESWAP,
        accounts.tick_array_bitmap_ext,
        accounts.tick_array_prev,
        accounts.tick_array_cur,
        accounts.tick_array_next,
    );
}

pub(super) fn append_t22(out: &mut Vec<AccountMeta>, accounts: PancakeswapT22Accounts) {
    append_clmm_t22_base(
        out,
        accounts.pool_state,
        accounts.amm_config,
        accounts.token_vault_0,
        accounts.token_vault_1,
        accounts.observation_state,
        accounts.token_mint_0,
        accounts.token_mint_1,
        PANCAKESWAP,
        accounts.tick_array_bitmap_ext,
        accounts.tick_array_prev,
        accounts.tick_array_cur,
        accounts.tick_array_next,
    );
}
