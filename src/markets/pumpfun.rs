use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::{SPL_ATA_PROGRAM, SYSTEM_PROGRAM};

use super::{push_ro, push_w};

pub(crate) const PUMPFUN_AMM: Pubkey = Pubkey::from_str_const("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");
pub(crate) const PUMPFUN_GLOBAL_CONFIG: Pubkey =
    Pubkey::from_str_const("ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw");
pub(crate) const PUMPFUN_EVENT_AUTHORITY: Pubkey =
    Pubkey::from_str_const("GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR");
pub(crate) const PUMPFUN_GLOBAL_ACCUMULATOR: Pubkey =
    Pubkey::from_str_const("C2aFPdENg4A2HQsmrd5rTw5TaYBX5Ku887cWjbFKtZpw");
pub(crate) const PUMPFUN_FEE_CONFIG: Pubkey = Pubkey::from_str_const("5PHirr8joyTMp9JMm6nW7hNDVyEYdkzDqazxPD7RaTjx");
pub(crate) const PUMPFUN_FEE_PROGRAM: Pubkey = Pubkey::from_str_const("pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PumpfunAmmAccounts {
    pub pool: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub pool_base_vault: Pubkey,
    pub pool_quote_vault: Pubkey,
    pub fee_recipient: Pubkey,
    pub fee_recipient_ata: Pubkey,
    pub base_token_program: Pubkey,
    pub quote_token_program: Pubkey,
    pub creator_vault_ata: Pubkey,
    pub creator_vault_authority: Pubkey,
    pub user_volume_accumulator: Pubkey,
    /// Slot 20. Required for cashback pools. When `None`, the SDK emits the
    /// documented read-only no-cashback placeholder, the Pumpfun AMM program id.
    pub user_volume_accumulator_wsol_ata: Option<Pubkey>,
    /// Slot 21. Required for creator-feature pools. When `None`, the SDK emits
    /// the documented pre-creator-feature placeholder, the Pumpfun AMM program id.
    pub pool_v2: Option<Pubkey>,
    pub swap_fee_recipient: Pubkey,
    pub swap_fee_ata: Pubkey,
}

pub(super) fn append_amm(out: &mut Vec<AccountMeta>, accounts: PumpfunAmmAccounts) {
    let user_volume_wsol_ata = accounts.user_volume_accumulator_wsol_ata.unwrap_or(PUMPFUN_AMM);
    push_w(out, accounts.pool);
    push_ro(out, PUMPFUN_GLOBAL_CONFIG);
    push_ro(out, accounts.base_mint);
    push_ro(out, accounts.quote_mint);
    push_w(out, accounts.pool_base_vault);
    push_w(out, accounts.pool_quote_vault);
    push_ro(out, accounts.fee_recipient);
    push_w(out, accounts.fee_recipient_ata);
    push_ro(out, accounts.base_token_program);
    push_ro(out, accounts.quote_token_program);
    push_ro(out, SYSTEM_PROGRAM);
    push_ro(out, SPL_ATA_PROGRAM);
    push_ro(out, PUMPFUN_EVENT_AUTHORITY);
    push_ro(out, PUMPFUN_AMM);
    push_w(out, accounts.creator_vault_ata);
    push_ro(out, accounts.creator_vault_authority);
    push_ro(out, PUMPFUN_GLOBAL_ACCUMULATOR);
    push_w(out, accounts.user_volume_accumulator);
    push_ro(out, PUMPFUN_FEE_CONFIG);
    push_ro(out, PUMPFUN_FEE_PROGRAM);
    if user_volume_wsol_ata == PUMPFUN_AMM {
        push_ro(out, user_volume_wsol_ata);
    } else {
        push_w(out, user_volume_wsol_ata);
    }
    push_ro(out, accounts.pool_v2.unwrap_or(PUMPFUN_AMM));
    push_ro(out, accounts.swap_fee_recipient);
    push_w(out, accounts.swap_fee_ata);
}
