use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::{
    constants::{SPL_MEMO, SPL_TOKEN, SPL_TOKEN_2022},
    BuildError,
};

pub mod alphaq;
pub mod byreal;
pub mod fusion;
pub mod futarchy;
pub mod goonfi;
pub mod humidifi;
pub mod manifest;
pub mod meteora;
pub mod orca;
pub mod pancakeswap;
pub mod pumpfun;
pub mod raydium;
pub mod solfi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MarketId {
    RaydiumV4 = 0,
    RaydiumCp = 1,
    RaydiumClmm = 2,
    OrcaWhirlpool = 3,
    MeteoraDlmm = 4,
    MeteoraDammV2 = 5,
    MeteoraPools = 6,
    PumpfunAmm = 7,
    Pancakeswap = 8,
    MeteoraDlmmT22 = 9,
    RaydiumClmmT22 = 10,
    PancakeswapT22 = 11,
    OrcaWhirlpoolT22 = 12,
    ByrealClmm = 13,
    ByrealClmmT22 = 14,
    HumidifiSwapV2 = 15,
    HumidifiSwap = 16,
    Manifest = 17,
    AlphaQ = 18,
    AlphaQT22 = 19,
    GoonfiV2 = 20,
    SolfiV2 = 21,
    FutarchySpot = 22,
    Fusion = 23,
}

impl MarketId {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::RaydiumV4 => "RaydiumV4",
            Self::RaydiumCp => "RaydiumCp",
            Self::RaydiumClmm => "RaydiumClmm",
            Self::OrcaWhirlpool => "OrcaWhirlpool",
            Self::MeteoraDlmm => "MeteoraDlmm",
            Self::MeteoraDammV2 => "MeteoraDammV2",
            Self::MeteoraPools => "MeteoraPools",
            Self::PumpfunAmm => "PumpfunAmm",
            Self::Pancakeswap => "Pancakeswap",
            Self::MeteoraDlmmT22 => "MeteoraDlmmT22",
            Self::RaydiumClmmT22 => "RaydiumClmmT22",
            Self::PancakeswapT22 => "PancakeswapT22",
            Self::OrcaWhirlpoolT22 => "OrcaWhirlpoolT22",
            Self::ByrealClmm => "ByrealClmm",
            Self::ByrealClmmT22 => "ByrealClmmT22",
            Self::HumidifiSwapV2 => "HumidifiSwapV2",
            Self::HumidifiSwap => "HumidifiSwap",
            Self::Manifest => "Manifest",
            Self::AlphaQ => "AlphaQ",
            Self::AlphaQT22 => "AlphaQT22",
            Self::GoonfiV2 => "GoonfiV2",
            Self::SolfiV2 => "SolfiV2",
            Self::FutarchySpot => "FutarchySpot",
            Self::Fusion => "Fusion",
        }
    }
}

impl TryFrom<u8> for MarketId {
    type Error = BuildError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RaydiumV4),
            1 => Ok(Self::RaydiumCp),
            2 => Ok(Self::RaydiumClmm),
            3 => Ok(Self::OrcaWhirlpool),
            4 => Ok(Self::MeteoraDlmm),
            5 => Ok(Self::MeteoraDammV2),
            6 => Ok(Self::MeteoraPools),
            7 => Ok(Self::PumpfunAmm),
            8 => Ok(Self::Pancakeswap),
            9 => Ok(Self::MeteoraDlmmT22),
            10 => Ok(Self::RaydiumClmmT22),
            11 => Ok(Self::PancakeswapT22),
            12 => Ok(Self::OrcaWhirlpoolT22),
            13 => Ok(Self::ByrealClmm),
            14 => Ok(Self::ByrealClmmT22),
            15 => Ok(Self::HumidifiSwapV2),
            16 => Ok(Self::HumidifiSwap),
            17 => Ok(Self::Manifest),
            18 => Ok(Self::AlphaQ),
            19 => Ok(Self::AlphaQT22),
            20 => Ok(Self::GoonfiV2),
            21 => Ok(Self::SolfiV2),
            22 => Ok(Self::FutarchySpot),
            23 => Ok(Self::Fusion),
            other => Err(BuildError::UnsupportedMarketId(other)),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketAccounts {
    RaydiumV4(raydium::RaydiumV4Accounts),
    RaydiumCp(raydium::RaydiumCpAccounts),
    RaydiumClmm(raydium::RaydiumClmmAccounts),
    OrcaWhirlpool(orca::OrcaWhirlpoolAccounts),
    MeteoraDlmm(meteora::MeteoraDlmmAccounts),
    MeteoraDammV2(meteora::MeteoraDammV2Accounts),
    MeteoraPools(meteora::MeteoraPoolsAccounts),
    PumpfunAmm(pumpfun::PumpfunAmmAccounts),
    Pancakeswap(pancakeswap::PancakeswapAccounts),
    MeteoraDlmmT22(meteora::MeteoraDlmmAccounts),
    RaydiumClmmT22(raydium::RaydiumClmmT22Accounts),
    PancakeswapT22(pancakeswap::PancakeswapT22Accounts),
    OrcaWhirlpoolT22(orca::OrcaWhirlpoolT22Accounts),
    ByrealClmm(byreal::ByrealClmmAccounts),
    ByrealClmmT22(byreal::ByrealClmmT22Accounts),
    HumidifiSwapV2(humidifi::HumidifiSwapV2Accounts),
    HumidifiSwap(humidifi::HumidifiSwapAccounts),
    Manifest(manifest::ManifestAccounts),
    AlphaQ(alphaq::AlphaQAccounts),
    AlphaQT22(alphaq::AlphaQAccounts),
    GoonfiV2(goonfi::GoonfiV2Accounts),
    SolfiV2(solfi::SolfiV2Accounts),
    FutarchySpot(futarchy::FutarchySpotAccounts),
    Fusion(fusion::FusionAccounts),
}

impl MarketAccounts {
    pub const fn market_id(self) -> MarketId {
        match self {
            Self::RaydiumV4(_) => MarketId::RaydiumV4,
            Self::RaydiumCp(_) => MarketId::RaydiumCp,
            Self::RaydiumClmm(_) => MarketId::RaydiumClmm,
            Self::OrcaWhirlpool(_) => MarketId::OrcaWhirlpool,
            Self::MeteoraDlmm(_) => MarketId::MeteoraDlmm,
            Self::MeteoraDammV2(_) => MarketId::MeteoraDammV2,
            Self::MeteoraPools(_) => MarketId::MeteoraPools,
            Self::PumpfunAmm(_) => MarketId::PumpfunAmm,
            Self::Pancakeswap(_) => MarketId::Pancakeswap,
            Self::MeteoraDlmmT22(_) => MarketId::MeteoraDlmmT22,
            Self::RaydiumClmmT22(_) => MarketId::RaydiumClmmT22,
            Self::PancakeswapT22(_) => MarketId::PancakeswapT22,
            Self::OrcaWhirlpoolT22(_) => MarketId::OrcaWhirlpoolT22,
            Self::ByrealClmm(_) => MarketId::ByrealClmm,
            Self::ByrealClmmT22(_) => MarketId::ByrealClmmT22,
            Self::HumidifiSwapV2(_) => MarketId::HumidifiSwapV2,
            Self::HumidifiSwap(_) => MarketId::HumidifiSwap,
            Self::Manifest(_) => MarketId::Manifest,
            Self::AlphaQ(_) => MarketId::AlphaQ,
            Self::AlphaQT22(_) => MarketId::AlphaQT22,
            Self::GoonfiV2(_) => MarketId::GoonfiV2,
            Self::SolfiV2(_) => MarketId::SolfiV2,
            Self::FutarchySpot(_) => MarketId::FutarchySpot,
            Self::Fusion(_) => MarketId::Fusion,
        }
    }

    pub const fn variant_name(self) -> &'static str {
        self.market_id().name()
    }

    pub const fn account_count(self) -> usize {
        match self {
            Self::RaydiumV4(_) => 5,
            Self::RaydiumCp(_) => 9,
            Self::RaydiumClmm(_) => 11,
            Self::OrcaWhirlpool(_) => 9,
            Self::MeteoraDlmm(_) => 15,
            Self::MeteoraDammV2(_) => 11,
            Self::MeteoraPools(_) => 14,
            Self::PumpfunAmm(_) => 24,
            Self::Pancakeswap(_) => 11,
            Self::MeteoraDlmmT22(_) => 16,
            Self::RaydiumClmmT22(_) => 15,
            Self::PancakeswapT22(_) => 15,
            Self::OrcaWhirlpoolT22(_) => 13,
            Self::ByrealClmm(_) => 11,
            Self::ByrealClmmT22(_) => 15,
            Self::HumidifiSwapV2(_) => 10,
            Self::HumidifiSwap(_) => 7,
            Self::Manifest(_) => 7,
            Self::AlphaQ(_) => 9,
            Self::AlphaQT22(_) => 10,
            Self::GoonfiV2(_) => 10,
            Self::SolfiV2(_) => 11,
            Self::FutarchySpot(_) => 5,
            Self::Fusion(_) => 14,
        }
    }

    pub(crate) fn append_account_metas(self, out: &mut Vec<AccountMeta>) {
        match self {
            Self::RaydiumV4(accounts) => raydium::append_v4(out, accounts),
            Self::RaydiumCp(accounts) => raydium::append_cp(out, accounts),
            Self::RaydiumClmm(accounts) => raydium::append_clmm(out, accounts),
            Self::OrcaWhirlpool(accounts) => orca::append_whirlpool(out, accounts),
            Self::MeteoraDlmm(accounts) => meteora::append_dlmm(out, accounts, false),
            Self::MeteoraDammV2(accounts) => meteora::append_damm_v2(out, accounts),
            Self::MeteoraPools(accounts) => meteora::append_pools(out, accounts),
            Self::PumpfunAmm(accounts) => pumpfun::append_amm(out, accounts),
            Self::Pancakeswap(accounts) => pancakeswap::append(out, accounts),
            Self::MeteoraDlmmT22(accounts) => meteora::append_dlmm(out, accounts, true),
            Self::RaydiumClmmT22(accounts) => raydium::append_clmm_t22(out, accounts),
            Self::PancakeswapT22(accounts) => pancakeswap::append_t22(out, accounts),
            Self::OrcaWhirlpoolT22(accounts) => orca::append_whirlpool_t22(out, accounts),
            Self::ByrealClmm(accounts) => byreal::append_clmm(out, accounts),
            Self::ByrealClmmT22(accounts) => byreal::append_clmm_t22(out, accounts),
            Self::HumidifiSwapV2(accounts) => humidifi::append_swap_v2(out, accounts),
            Self::HumidifiSwap(accounts) => humidifi::append_swap(out, accounts),
            Self::Manifest(accounts) => manifest::append(out, accounts),
            Self::AlphaQ(accounts) => alphaq::append(out, accounts, false),
            Self::AlphaQT22(accounts) => alphaq::append(out, accounts, true),
            Self::GoonfiV2(accounts) => goonfi::append_v2(out, accounts),
            Self::SolfiV2(accounts) => solfi::append_v2(out, accounts),
            Self::FutarchySpot(accounts) => futarchy::append_spot(out, accounts),
            Self::Fusion(accounts) => fusion::append(out, accounts),
        }
    }
}

fn push_w(out: &mut Vec<AccountMeta>, key: Pubkey) {
    out.push(AccountMeta::new(key, false));
}

fn push_ro(out: &mut Vec<AccountMeta>, key: Pubkey) {
    out.push(AccountMeta::new_readonly(key, false));
}

#[allow(clippy::too_many_arguments)]
fn append_clmm_base(
    out: &mut Vec<AccountMeta>,
    pool_state: Pubkey,
    amm_config: Pubkey,
    token_vault_0: Pubkey,
    token_vault_1: Pubkey,
    observation_state: Pubkey,
    program: Pubkey,
    tick_array_bitmap_ext: Pubkey,
    tick_array_prev: Pubkey,
    tick_array_cur: Pubkey,
    tick_array_next: Pubkey,
) {
    push_ro(out, amm_config);
    push_w(out, pool_state);
    push_w(out, token_vault_0);
    push_w(out, token_vault_1);
    push_w(out, observation_state);
    push_ro(out, SPL_TOKEN);
    push_ro(out, program);
    push_w(out, tick_array_bitmap_ext);
    push_w(out, tick_array_prev);
    push_w(out, tick_array_cur);
    push_w(out, tick_array_next);
}

#[allow(clippy::too_many_arguments)]
fn append_clmm_t22_base(
    out: &mut Vec<AccountMeta>,
    pool_state: Pubkey,
    amm_config: Pubkey,
    token_vault_0: Pubkey,
    token_vault_1: Pubkey,
    observation_state: Pubkey,
    token_mint_0: Pubkey,
    token_mint_1: Pubkey,
    program: Pubkey,
    tick_array_bitmap_ext: Pubkey,
    tick_array_prev: Pubkey,
    tick_array_cur: Pubkey,
    tick_array_next: Pubkey,
) {
    push_ro(out, amm_config);
    push_w(out, pool_state);
    push_w(out, token_vault_0);
    push_w(out, token_vault_1);
    push_w(out, observation_state);
    push_ro(out, SPL_TOKEN);
    push_ro(out, SPL_TOKEN_2022);
    push_ro(out, SPL_MEMO);
    push_ro(out, token_mint_0);
    push_ro(out, token_mint_1);
    push_ro(out, program);
    push_w(out, tick_array_bitmap_ext);
    push_w(out, tick_array_prev);
    push_w(out, tick_array_cur);
    push_w(out, tick_array_next);
}
