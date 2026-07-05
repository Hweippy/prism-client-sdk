use solana_pubkey::Pubkey;

pub const PROGRAM_ID: Pubkey = Pubkey::from_str_const("Prism8hsRo6Ww5jiN5Zeh3YDPLZHqHduCPSAV7JF7qv");
pub const FIND_ARB_V2_DISCRIMINATOR: u8 = 7;

pub(crate) const FLAG_FLASHLOAN: u8 = 1 << 0;
pub(crate) const FLAG_FAIL_IF_NO_PROFIT: u8 = 1 << 1;

pub const WSOL_MINT: Pubkey = Pubkey::from_str_const("So11111111111111111111111111111111111111112");
pub const USDC_MINT: Pubkey = Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const USDT_MINT: Pubkey = Pubkey::from_str_const("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
pub const USD1_MINT: Pubkey = Pubkey::from_str_const("USD1ttGY1N17NEEHLmELoaybftRBUSErhqYiQzvEmuB");
pub const VAULT_AUTH: Pubkey = Pubkey::from_str_const("6jM8tJbomfcjgP343TsT13duo7bxvEVq5jftjPytDq6M");
pub const FEE_OWNER: Pubkey = VAULT_AUTH;
pub const VAULT_ATA_WSOL: Pubkey = Pubkey::from_str_const("AsnFKUNo4oXzHuu91WTgB2mUT1bcsx9kKyurhAyeur56");
pub const VAULT_ATA_USDC: Pubkey = Pubkey::from_str_const("ApvRzpyA7EXKqGaZ78JPXyjyFUFTqRaSDenPj95dPows");
pub const FEE_ATA_USDT: Pubkey = Pubkey::from_str_const("9a76tENsmkTumYbA46r6FD9raFb74RKNcJ4ey4R61gwt");
pub const FEE_ATA_USD1: Pubkey = Pubkey::from_str_const("Egv1XeHAL4LxU59jDgQm3Q7jeqAm5SSyq8xGjEdHDZzh");

pub const SPL_TOKEN: Pubkey = Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub(crate) const SPL_TOKEN_2022: Pubkey = Pubkey::from_str_const("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
pub(crate) const SPL_MEMO: Pubkey = Pubkey::from_str_const("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
pub const SPL_ATA_PROGRAM: Pubkey = Pubkey::from_str_const("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
pub(crate) const SYSTEM_PROGRAM: Pubkey = Pubkey::from_str_const("11111111111111111111111111111111");
pub(crate) const SYSVAR_CLOCK: Pubkey = Pubkey::from_str_const("SysvarC1ock11111111111111111111111111111111");
pub(crate) const SYSVAR_INSTRUCTIONS: Pubkey = Pubkey::from_str_const("Sysvar1nstructions1111111111111111111111111");
