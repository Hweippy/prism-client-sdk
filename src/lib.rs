//! Pure client-side instruction builder for Prism `find_arb_v2`.
//!
//! This crate does not discover routes, fetch accounts, derive user token
//! accounts, create ATAs, choose lookup tables, build transactions, or submit
//! anything to Solana. Callers provide pool-specific account pubkeys through
//! typed market structs, and the SDK serializes those structs into Prism's
//! documented account order.

mod constants;

pub mod markets;

use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use thiserror::Error;

use crate::{
    constants::{FLAG_FAIL_IF_NO_PROFIT, FLAG_FLASHLOAN},
    markets::MarketAccounts,
};

pub use constants::{
    FEE_ATA_USD1, FEE_ATA_USDT, FEE_OWNER, FIND_ARB_V2_DISCRIMINATOR, PROGRAM_ID, SPL_ATA_PROGRAM,
    SPL_TOKEN, USDC_MINT, USD1_MINT, USDT_MINT, VAULT_ATA_USDC, VAULT_ATA_WSOL, VAULT_AUTH, WSOL_MINT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MintAccount {
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub user_ata: Pubkey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindArbV2Params {
    pub signer: Pubkey,
    pub base: MintAccount,
    pub flashloan: bool,
    pub fail_if_no_profit: bool,
    /// Minimum realized profit floor in `base.mint` atomic units.
    ///
    /// For WSOL this is native lamports. For USDC/USDT/USD1 this is 6-decimal
    /// token units, so callers must convert any native SOL tip into base units
    /// before building the instruction.
    pub min_profit_base_units: u64,
    /// Wire byte forwarded to Prism for dynamic walk-table depth.
    ///
    /// Prism uses this as the per-direction DLMM bin-walk request and as the
    /// produced-step cap for selected CL-family tick walks, then clamps it to
    /// each program-side capacity.
    pub max_dynamic_walk_steps: u8,
    pub route_mints: Vec<MintAccount>,
    pub pools: Vec<MarketAccounts>,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    #[error("unsupported Prism base mint {0}")]
    UnsupportedBaseMint(Pubkey),
    #[error("unsupported Prism flashloan base mint {0}")]
    UnsupportedFlashloanBaseMint(Pubkey),
    #[error("route mint count does not fit find_arb_v2 wire format ({0})")]
    RouteMintCountOverflow(usize),
    #[error("missing route mints")]
    MissingRouteMints,
    #[error("route mint duplicates base mint {0}")]
    BaseMintInRouteMints(Pubkey),
    #[error("duplicate route mint {0}")]
    DuplicateRouteMint(Pubkey),
    #[error("pool count does not fit find_arb_v2 wire format ({0})")]
    PoolCountOverflow(usize),
    #[error("missing pools")]
    MissingPools,
    #[error("unsupported market id {0}")]
    UnsupportedMarketId(u8),
}

pub fn build_find_arb_v2_instruction(params: FindArbV2Params) -> Result<Instruction, BuildError> {
    let fee_recipient_ata = prism_fee_recipient_ata(params.base.mint, params.base.token_program)?;
    validate_route_mints(params.base.mint, &params.route_mints)?;
    if params.pools.is_empty() {
        return Err(BuildError::MissingPools);
    }
    let num_mints = u8::try_from(params.route_mints.len())
        .map_err(|_| BuildError::RouteMintCountOverflow(params.route_mints.len()))?;
    let num_pools = u8::try_from(params.pools.len())
        .map_err(|_| BuildError::PoolCountOverflow(params.pools.len()))?;

    let mut data = Vec::with_capacity(13 + params.pools.len());
    data.push(FIND_ARB_V2_DISCRIMINATOR);
    data.push(flags(params.flashloan, params.fail_if_no_profit));
    data.push(params.max_dynamic_walk_steps);
    data.push(num_mints);
    data.push(num_pools);
    data.extend_from_slice(&params.min_profit_base_units.to_le_bytes());
    for pool in &params.pools {
        data.push(pool.market_id().as_u8());
    }

    let pool_account_count: usize = params.pools.iter().map(|pool| pool.account_count()).sum();
    let mut accounts = Vec::with_capacity(
        5 + usize::from(params.flashloan) + params.route_mints.len() * 2 + pool_account_count,
    );
    accounts.push(AccountMeta::new(params.signer, true));
    accounts.push(AccountMeta::new(params.base.user_ata, false));
    accounts.push(AccountMeta::new_readonly(params.base.mint, false));
    accounts.push(AccountMeta::new_readonly(params.base.token_program, false));
    if params.flashloan {
        let (vault_ata, vault_auth) = prism_flashloan_vault_accounts(params.base.mint)?;
        accounts.push(AccountMeta::new(vault_ata, false));
        accounts.push(AccountMeta::new_readonly(vault_auth, false));
    } else {
        accounts.push(AccountMeta::new(fee_recipient_ata, false));
    }

    for mint in &params.route_mints {
        accounts.push(AccountMeta::new_readonly(mint.token_program, false));
        accounts.push(AccountMeta::new(mint.user_ata, false));
    }

    for pool in params.pools {
        pool.append_account_metas(&mut accounts);
    }

    Ok(Instruction {
        program_id: PROGRAM_ID,
        accounts,
        data,
    })
}

fn validate_route_mints(base_mint: Pubkey, route_mints: &[MintAccount]) -> Result<(), BuildError> {
    if route_mints.is_empty() {
        return Err(BuildError::MissingRouteMints);
    }
    for (index, mint) in route_mints.iter().enumerate() {
        if mint.mint == base_mint {
            return Err(BuildError::BaseMintInRouteMints(mint.mint));
        }
        if route_mints[..index]
            .iter()
            .any(|existing| existing.mint == mint.mint)
        {
            return Err(BuildError::DuplicateRouteMint(mint.mint));
        }
    }

    Ok(())
}

pub fn prism_flashloan_vault_accounts(base_mint: Pubkey) -> Result<(Pubkey, Pubkey), BuildError> {
    if base_mint == WSOL_MINT {
        Ok((VAULT_ATA_WSOL, VAULT_AUTH))
    } else if base_mint == USDC_MINT {
        Ok((VAULT_ATA_USDC, VAULT_AUTH))
    } else {
        Err(BuildError::UnsupportedFlashloanBaseMint(base_mint))
    }
}

pub fn prism_base_mint_supported(base_mint: Pubkey) -> bool {
    base_mint == WSOL_MINT
        || base_mint == USDC_MINT
        || base_mint == USDT_MINT
        || base_mint == USD1_MINT
}

pub fn prism_fee_recipient_ata(
    base_mint: Pubkey,
    base_token_program: Pubkey,
) -> Result<Pubkey, BuildError> {
    if base_token_program != SPL_TOKEN {
        return Err(BuildError::UnsupportedBaseMint(base_mint));
    }
    if base_mint == WSOL_MINT {
        Ok(VAULT_ATA_WSOL)
    } else if base_mint == USDC_MINT {
        Ok(VAULT_ATA_USDC)
    } else if base_mint == USDT_MINT {
        Ok(FEE_ATA_USDT)
    } else if base_mint == USD1_MINT {
        Ok(FEE_ATA_USD1)
    } else {
        Err(BuildError::UnsupportedBaseMint(base_mint))
    }
}

pub fn associated_token_address(owner: Pubkey, mint: Pubkey, token_program: Pubkey) -> Pubkey {
    Pubkey::derive_program_address(
        &[owner.as_ref(), token_program.as_ref(), mint.as_ref()],
        &SPL_ATA_PROGRAM,
    )
    .expect("associated token address PDA derivation must succeed")
    .0
}

fn flags(flashloan: bool, fail_if_no_profit: bool) -> u8 {
    let mut out = 0;
    if flashloan {
        out |= FLAG_FLASHLOAN;
    }
    if fail_if_no_profit {
        out |= FLAG_FAIL_IF_NO_PROFIT;
    }
    out
}

#[cfg(test)]
mod tests;
