# prism-client-sdk

Self-contained Rust builder for one Prism `find_arb_v2` instruction.

Prism is an on-chain Solana arbitrage execution program with a just-in-time routing engine. Callers submit a pool menu plus the required market accounts in a fully specified `find_arb_v2` instruction; at execution time, Prism reads those supplied pools on-chain, finds an executable route across them, automatically chooses the swap input amount to maximize value for the submitted arb opportunity, and executes it through the supported DEX programs. The live program is [`Prism8hsRo6Ww5jiN5Zeh3YDPLZHqHduCPSAV7JF7qv`](https://solscan.io/account/Prism8hsRo6Ww5jiN5Zeh3YDPLZHqHduCPSAV7JF7qv).

<img width="600" height="150" alt="image" src="https://github.com/user-attachments/assets/be89d44a-f0c2-4b20-9573-dfe07a691a23" />

This crate is intentionally narrow. Callers must provide every account pubkey from their own indexer, parser, cache, or configuration. The SDK does not fetch accounts, discover pools, create ATAs, infer markets, send transactions, choose lookup tables, manage compute budget, or validate account-lock limits.

## Contents

- [Supported Programs](#supported-programs)
- [Route Shape](#route-shape)
- [Rust Example](#rust-example)
- [Instruction Data](#instruction-data)
- [Settlement Accounts](#settlement-accounts)
- [Account Prefix](#account-prefix)
- [Market IDs](#market-ids)
- [Market Accounts](#market-accounts)
- [Emitted Pool Slices](#emitted-pool-slices)
- [Common On-chain Errors](#common-on-chain-errors)
- [Build Errors](#build-errors)

## Supported Programs

| DEX | Program ID |
| --- | --- |
| Raydium v4 | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| Raydium CP | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| Raydium CLMM | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| Whirlpool | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` |
| Meteora DLMM | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo` |
| Meteora DAMM V2 | `cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG` |
| Meteora Pools | `Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB` |
| PumpfunAmm | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |
| PancakeSwap | `HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq` |
| Byreal | `REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2` |
| Humidifi | `9H6tua7jkLhdm3w8BvgpTn5LZNU7g4ZynDmCiNN3q6Rp` |
| Manifest | `MNFSTqtC93rEfYHB6hF82sKdZpUDFWkViLByLd1k1Ms` |
| AlphaQ | `ALPHAQmeA7bjrVuccPsYPiCvsi428SNwte66Srvs4pHA` |
| GoonFi v2 (SPL Token / Token-2022) | `goonuddtQRrWqqn5nFyczVKaie28f3kDkHWkHtURSLE` |
| SolFi v2 | `SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF` |
| Futarchy | `FUTARELBfJfQ8RDGhg1wdhddq1odMAJUePHFuBYfUxKq` |
| Fusion | `fUSioN9YKKSa3CUC2YUc4tPkHJ5Y6XW1yz8y6F7qWz9` |
| BisonFi | `BiSoNHVpsVZW2F7rx2eQ59yQwKxzU5NvBcmKshCSUypi` |
| Tessera | `TessVdML9pBGgG9yGks7o4HewRaXVAMuoVj4x83GLQH` |
| ZeroFi | `ZERor4xhbUycZ6gb9ntrhqscUcZmAbQDjEAtCf4hbZY` |

## Route Shape

`find_arb_v2` supports 2-hop and constrained 3-hop arbitrage only. It does not build or execute routes with more than three swap legs.

The `base` mint and `route_mints` fields have the same meaning for both shapes. `base` is the cycle's base token, and `route_mints` contains every non-base mint the submitted pool graph may touch. For a 2-hop route, that is usually one target mint. For a 3-hop route, include both non-base mints in the triangle.

`pools` is an unordered pool menu, not an ordered route. Prism reads each pool's endpoint mints on-chain, builds the candidate graph, and chooses the executable route order itself. Pools that touch `base` can become direct edges for 2-hop routes. Pools that do not touch `base`, but connect two submitted `route_mints`, can become the middle bridge edge of a 3-hop route. Submitting bridge pools for possible 3-hop routes does not disable 2-hop evaluation; Prism evaluates direct 2-hop candidates from the same pool menu when matching base-touching pools are present. The order of `pools` only controls instruction serialization; it does not force route execution order.

## Rust Example

The SDK starts after you already know the accounts. With those pubkeys already in scope, pass them into the typed structs and build the Prism instruction:

```rust
use prism_client_sdk::{
    build_find_arb_v2_instruction, FindArbV2Params, MintAccount,
    markets::{
        raydium::{RaydiumCpAccounts, RaydiumV4Accounts},
        MarketAccounts,
    },
};
use solana_pubkey::Pubkey;

const WSOL_MINT: Pubkey = Pubkey::from_str_const("So11111111111111111111111111111111111111112");
const USDC_MINT: Pubkey = Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const SPL_TOKEN: Pubkey = Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

let ix = build_find_arb_v2_instruction(FindArbV2Params {
    signer,
    base: MintAccount {
        mint: WSOL_MINT,
        token_program: SPL_TOKEN,
        user_ata: wsol_user_ata,
    },
    flashloan: true,
    fail_if_no_profit: true,
    min_profit_base_units: 10_000,
    max_dynamic_walk_steps: 12,
    route_mints: vec![MintAccount {
        mint: USDC_MINT,
        token_program: SPL_TOKEN,
        user_ata: usdc_user_ata,
    }],
    pools: vec![
        MarketAccounts::RaydiumV4(RaydiumV4Accounts {
            pool_state: Pubkey::from_str_const("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2"),
            coin_vault: Pubkey::from_str_const("DQyrAcCrDXQ7NeoqGgDCZwBvWDcYmFCjSb9JtteuvPpz"),
            pc_vault: Pubkey::from_str_const("HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz"),
        }),
        MarketAccounts::RaydiumCp(RaydiumCpAccounts {
            pool_state: Pubkey::from_str_const("47hq28mcL7q5GhBg7epyGF2dnuJd4MKFt8QhT7CzYUp4"),
            amm_config: Pubkey::from_str_const("BgxH5ifebqHDuiADWKhLjXGP5hWZeZLoCdmeWJLkRqLP"),
            token_0_vault: Pubkey::from_str_const("9tAoQUNB1wKnBSUyc6ukg2pWbeofZJVHxur2UaRBZNZc"),
            token_1_vault: Pubkey::from_str_const("BArWLNarRzS7N1GCwBjNgVnC4866c8oJNRqhb5axf6y4"),
            token_0_mint: WSOL_MINT,
            token_1_mint: USDC_MINT,
            observation_state: Pubkey::from_str_const("AwXcFysn5vntDyiGo9hXDhTMFSg2in7hxj8ED51oZgqo"),
        }),
    ],
})?;
```

This example builds a two-pool menu for a 2-hop candidate. The caller is still responsible for selecting pools whose endpoint mints match `base` and `route_mints`; the SDK only assembles the instruction.

## Instruction Data

`find_arb_v2` data layout:

```text
[7, flags, max_dynamic_walk_steps, num_mints, num_pools, min_profit_base_units_le_u64, market_ids...]
```

`max_dynamic_walk_steps` is Prism's wire byte for dynamic walk-table depth. Prism uses it as the per-direction DLMM bin-walk request and as the produced-step cap for selected CL-family tick walks, then clamps it to each program-side capacity. For example, `20` asks Prism to walk up to 20 DLMM bins or selected CL tick steps per direction. Raising this value can find deeper walked liquidity, but increases compute and account-window pressure; lowering it keeps attempts lighter, but can miss routes that need a wider walk.

`min_profit_base_units` is denominated in the submitted base mint's atomic
units. For WSOL this is native lamports. For USDC, USDT, and USD1 it is
6-decimal token units; callers that use a native SOL lander tip must convert
that tip value into base units before building the instruction.

Flags:

```text
bit 0: flashloan
bit 1: fail_if_no_profit
```

`num_mints` must be non-zero. The route mint list must not contain the base mint and must not contain duplicate mint pubkeys. The SDK does not expose Prism's high internal safety caps as client-side routing limits.

## Settlement Accounts

Prism has one vault authority:

```text
VAULT_AUTH = PDA([b"vault_auth", WSOL_MINT], PRISM_PROGRAM_ID)
           = 6jM8tJbomfcjgP343TsT13duo7bxvEVq5jftjPytDq6M
```

Prism supports four SPL Token base mints for settlement: WSOL, USDC, USDT, and USD1. Any other base mint is rejected before route execution.

Flashloan mode is available only for WSOL and USDC base mints. The vault account is the SPL-token ATA owned by `VAULT_AUTH` for the selected base mint:

```text
WSOL vault = ATA(VAULT_AUTH, WSOL_MINT, SPL_TOKEN)
           = AsnFKUNo4oXzHuu91WTgB2mUT1bcsx9kKyurhAyeur56

USDC vault = ATA(VAULT_AUTH, USDC_MINT, SPL_TOKEN)
           = ApvRzpyA7EXKqGaZ78JPXyjyFUFTqRaSDenPj95dPows
```

When `flashloan = true`, the SDK emits the selected vault ATA plus `VAULT_AUTH` in the account prefix. Prism borrows the current vault balance into the signer's base ATA, executes the selected route, then transfers principal plus protocol fee from the signer's base ATA back into the same vault. That means successful flashloan fees accrue directly in the WSOL or USDC flashloan vault.

When `flashloan = false`, there is no vault authority account in the prefix. `FEE_OWNER` is the same pubkey as `VAULT_AUTH`, and the SDK emits only the writable fee recipient ATA:

```text
WSOL fee ATA = ATA(FEE_OWNER, WSOL_MINT, SPL_TOKEN)
             = AsnFKUNo4oXzHuu91WTgB2mUT1bcsx9kKyurhAyeur56

USDC fee ATA = ATA(FEE_OWNER, USDC_MINT, SPL_TOKEN)
             = ApvRzpyA7EXKqGaZ78JPXyjyFUFTqRaSDenPj95dPows

USDT fee ATA = ATA(FEE_OWNER, USDT_MINT, SPL_TOKEN)
             = 9a76tENsmkTumYbA46r6FD9raFb74RKNcJ4ey4R61gwt

USD1 fee ATA = ATA(FEE_OWNER, USD1_MINT, SPL_TOKEN)
             = Egv1XeHAL4LxU59jDgQm3Q7jeqAm5SSyq8xGjEdHDZzh
```

This non-flashloan path supports WSOL, USDC, USDT, and USD1. The transfer is fee-only; no principal is repaid because Prism did not borrow from a vault.

Prism charges a 7% protocol fee only on realized profit from profitable arbitrage. The fee is paid through the selected flashloan vault or non-flashloan fee-recipient ATA. Failed attempts and attempts where Prism finds no profitable route do not pay this protocol fee.

The SDK computes these addresses internally. It also exposes `prism_flashloan_vault_accounts(base_mint)`, `prism_fee_recipient_ata(base_mint, base_token_program)`, `prism_base_mint_supported(base_mint)`, and `associated_token_address(owner, mint, token_program)` for callers that want to inspect or pre-create the same accounts.

## Account Prefix

Base prefix:

```text
0 signer                 writable signer
1 base user ATA          writable
2 base mint              readonly
3 base token program     readonly
```

Flashloan prefix appends:

```text
4 Prism base vault ATA   writable
5 Prism base vault auth  readonly
```

Non-flashloan prefix appends:

```text
4 Prism fee ATA          writable
```

The flashloan prefix supports WSOL and USDC. The non-flashloan prefix supports WSOL, USDC, USDT, and USD1.

Each route mint appends:

```text
token program            readonly
user ATA                 writable
```

## Market IDs

```text
0  RaydiumV4
1  RaydiumCp
2  RaydiumClmm
3  OrcaWhirlpool
4  MeteoraDlmm
5  MeteoraDammV2
6  MeteoraPools
7  PumpfunAmm
8  Pancakeswap
9  MeteoraDlmmT22
10 RaydiumClmmT22
11 PancakeswapT22
12 OrcaWhirlpoolT22
13 ByrealClmm
14 ByrealClmmT22
15 HumidifiSwapV2
16 HumidifiSwap
17 Manifest
18 AlphaQ
19 AlphaQT22
20 GoonfiV2
21 SolfiV2
22 FutarchySpot
23 Fusion
24 BisonFi
25 Tessera
26 ZeroFi
28 GoonfiV2T22
```

## Market Accounts

Callers pass `FindArbV2Params { pools: Vec<MarketAccounts> }`, where `MarketAccounts` lives under `prism_client_sdk::markets`. Each variant wraps a market-specific struct from its family module, such as `markets::raydium::RaydiumV4Accounts` or `markets::meteora::MeteoraDlmmAccounts`, with only the dynamic pubkeys the caller must know. The SDK derives the market id from the enum variant, inserts Prism's fixed program ids, authorities, sysvars, event authorities, and token-program constants, then emits the full remaining-account list in Prism's order with the correct writable/readonly flags.

There is no separate market id field to keep in sync with a raw pubkey slice. If the variant is `MarketAccounts::RaydiumV4`, the instruction data receives market id `0` and the Raydium v4 account layout is emitted.

Documented optional placeholders are represented as `Option<Pubkey>`:

```text
MeteoraDlmmAccounts.host_fee_in
  None emits the DLMM program id placeholder.

MeteoraDammV2Accounts.instructions_sysvar_or_program_sentinel
  Some(SYSVAR_INSTRUCTIONS) for rate-limiter pools.
  None emits the DAMM v2 program id sentinel.

PumpfunAmmAccounts.user_volume_accumulator_wsol_ata
  Some(account) for cashback pools.
  None emits the Pumpfun AMM program id placeholder.

PumpfunAmmAccounts.pool_v2
  Some(account) for creator-feature pools.
  None emits the Pumpfun AMM program id placeholder.
```

CLMM current tick-array placeholder:

`RaydiumClmm`, `RaydiumClmmT22`, `Pancakeswap`, `PancakeswapT22`, `ByrealClmm`, and `ByrealClmmT22` also accept the variant's DEX program id in `tick_array_cur`. Use this only after checking that the mathematical current tick-array PDA does not exist, or when another supplied tick-array slot intentionally carries the first valid tick array for the swap direction. `tick_array_prev` and `tick_array_next` must not be program-id placeholders; any slot Prism may use as the first CPI tick array must be a loadable tick-array account.

All other fields are required. The SDK does not invent dynamic pool accounts, derive user ATAs, infer token programs, choose tick arrays, check tick-array existence, or fetch state.

## Emitted Pool Slices

The SDK emits each pool as one contiguous account slice. The slice order must match the `MarketAccounts` variant order in `FindArbV2Params.pools`, because Prism slices the remaining accounts by the emitted `market_ids`.

Legend:

```text
W   writable
    read-only
SDK constant  inserted by the SDK
caller        supplied through the typed account struct
```

### Raydium V4

```text
0  W   pool_state                         caller
1      Raydium v4 authority               SDK constant
2  W   coin_vault                         caller
3  W   pc_vault                           caller
4      Raydium v4 program                 SDK constant
```

### Raydium CP

```text
0  W   pool_state                         caller
1      amm_config                         caller
2      Raydium CP vault authority         SDK constant
3  W   token_0_vault                      caller
4  W   token_1_vault                      caller
5      token_0_mint                       caller
6      token_1_mint                       caller
7  W   observation_state                  caller
8      Raydium CP program                 SDK constant
```

### CLMM Base

Used by `RaydiumClmm`, `Pancakeswap`, and `ByrealClmm`.

```text
0      amm_config                         caller
1  W   pool_state                         caller
2  W   token_vault_0                      caller
3  W   token_vault_1                      caller
4  W   observation_state                  caller
5      SPL Token program                  SDK constant
6      DEX program                        SDK constant for variant
7  W   tick_array_bitmap_ext              caller
8  W   tick_array_prev                    caller
9  W   tick_array_cur                     caller, or DEX program id placeholder
10 W   tick_array_next                    caller
```

### CLMM Token-2022

Used by `RaydiumClmmT22`, `PancakeswapT22`, and `ByrealClmmT22`.

```text
0      amm_config                         caller
1  W   pool_state                         caller
2  W   token_vault_0                      caller
3  W   token_vault_1                      caller
4  W   observation_state                  caller
5      SPL Token program                  SDK constant
6      Token-2022 program                 SDK constant
7      Memo v3 program                    SDK constant
8      token_mint_0                       caller
9      token_mint_1                       caller
10     DEX program                        SDK constant for variant
11 W   tick_array_bitmap_ext              caller
12 W   tick_array_prev                    caller
13 W   tick_array_cur                     caller, or DEX program id placeholder
14 W   tick_array_next                    caller
```

### Orca Whirlpool

```text
0      SPL Token program                  SDK constant
1  W   whirlpool                          caller
2  W   vault_a                            caller
3  W   vault_b                            caller
4  W   tick_array_prev                    caller
5  W   tick_array_cur                     caller
6  W   tick_array_next                    caller
7  W   oracle                             caller
8      Orca Whirlpool program             SDK constant
```

### Orca Whirlpool Token-2022

```text
0      token_program_a                    caller
1      token_program_b                    caller
2      Memo v3 program                    SDK constant
3  W   whirlpool                          caller
4      token_mint_a                       caller
5      token_mint_b                       caller
6  W   vault_a                            caller
7  W   vault_b                            caller
8  W   tick_array_prev                    caller
9  W   tick_array_cur                     caller
10 W   tick_array_next                    caller
11 W   oracle                             caller
12     Orca Whirlpool program             SDK constant
```

### Meteora DLMM

```text
0  W   lb_pair                            caller
1  W   bin_array_bitmap_ext               caller
2  W   reserve_x                          caller
3  W   reserve_y                          caller
4      token_x_mint                       caller
5      token_y_mint                       caller
6  W   oracle                             caller
7  W   host_fee_in or DLMM placeholder    caller option or SDK constant
8      token_x_program                    caller
9      token_y_program                    caller
10     DLMM event authority               SDK constant
11     DLMM program                       SDK constant
12 W   bin_array_prev                     caller
13 W   bin_array_cur                      caller
14 W   bin_array_next                     caller
```

### Meteora DLMM Token-2022

```text
0  W   lb_pair                            caller
1  W   bin_array_bitmap_ext               caller
2  W   reserve_x                          caller
3  W   reserve_y                          caller
4      token_x_mint                       caller
5      token_y_mint                       caller
6  W   oracle                             caller
7  W   host_fee_in or DLMM placeholder    caller option or SDK constant
8      token_x_program                    caller
9      token_y_program                    caller
10     Memo v3 program                    SDK constant
11     DLMM event authority               SDK constant
12     DLMM program                       SDK constant
13 W   bin_array_prev                     caller
14 W   bin_array_cur                      caller
15 W   bin_array_next                     caller
```

### Meteora DAMM v2

```text
0      pool authority                     SDK constant
1  W   pool                               caller
2  W   token_a_vault                      caller
3  W   token_b_vault                      caller
4      token_a_mint                       caller
5      token_b_mint                       caller
6      token_a_program                    caller
7      token_b_program                    caller
8      DAMM v2 event authority            SDK constant
9      DAMM v2 program                    SDK constant
10     instructions sysvar or sentinel    caller option or SDK constant
```

### Meteora Pools

```text
0  W   pool                               caller
1  W   a_vault                            caller
2  W   b_vault                            caller
3  W   a_token_vault                      caller
4  W   b_token_vault                      caller
5  W   a_vault_lp_mint                    caller
6  W   b_vault_lp_mint                    caller
7  W   a_vault_lp                         caller
8  W   b_vault_lp                         caller
9  W   protocol_fee_a                     caller
10 W   protocol_fee_b                     caller
11     vault_program                      caller
12     SPL Token program                  SDK constant
13     Meteora Pools program              SDK constant
```

### Pumpfun AMM

```text
0  W   pool                               caller
1      global config                      SDK constant
2      base_mint                          caller
3      quote_mint                         caller
4  W   pool_base_vault                    caller
5  W   pool_quote_vault                   caller
6      fee_recipient                      caller
7  W   fee_recipient_ata                  caller
8      base_token_program                 caller
9      quote_token_program                caller
10     system program                     SDK constant
11     associated token program           SDK constant
12     event authority                    SDK constant
13     Pumpfun AMM program                SDK constant
14 W   creator_vault_ata                  caller
15     creator_vault_authority            caller
16     global accumulator                 SDK constant
17 W   user_volume_accumulator            caller
18     fee_config                         SDK constant
19     fee_program                        SDK constant
20 W   user_volume_accumulator_wsol_ata   caller option when present
20     Pumpfun AMM placeholder            SDK constant when option is None
21     pool_v2 or Pumpfun placeholder     caller option or SDK constant
22     swap_fee_recipient                 caller
23 W   swap_fee_ata                       caller
```

### HumidiFi Swap v2

```text
0  W   pool                               caller
1  W   base_vault                         caller
2  W   quote_vault                        caller
3      clock sysvar                       SDK constant
4      instructions sysvar                SDK constant
5      base_mint                          caller
6      quote_mint                         caller
7  W   side_table                         caller
8      jito_vote                          caller
9      HumidiFi program                   SDK constant
```

### HumidiFi Swap

```text
0  W   pool                               caller
1  W   base_vault                         caller
2  W   quote_vault                        caller
3      clock sysvar                       SDK constant
4      instructions sysvar                SDK constant
5      jito_vote                          caller
6      HumidiFi program                   SDK constant
```

### Manifest

```text
0  W   market                             caller
1  W   base_vault                         caller
2  W   quote_vault                        caller
3      base_mint                          caller
4      quote_mint                         caller
5      system program                     SDK constant
6      Manifest program                   SDK constant
```

### AlphaQ

```text
0  W   pool                               caller
1  W   market_stats                       caller
2  W   vault_left                         caller
3  W   vault_right                        caller
4      instructions sysvar                SDK constant
5      SPL Token program                  SDK constant
6      mint_left                          caller
7      mint_right                         caller
8      AlphaQ program                     SDK constant
```

### AlphaQ Token-2022

```text
0  W   pool                               caller
1  W   market_stats                       caller
2  W   vault_left                         caller
3  W   vault_right                        caller
4      instructions sysvar                SDK constant
5      SPL Token program                  SDK constant
6      mint_left                          caller
7      mint_right                         caller
8      AlphaQ program                     SDK constant
9      Token-2022 program                 SDK constant
```

### GoonFi v2

```text
0  W   pair                               caller
1  W   vault_a                            caller
2  W   vault_b                            caller
3      mint_a                             caller
4      mint_b                             caller
5      side_price                         caller
6  W   global_state                       caller
7      instructions sysvar                SDK constant
8      SPL Token program                  SDK constant
9      GoonFi v2 program                  SDK constant
```

### GoonFi v2 Token-2022

```text
0  W   pair                               caller
1  W   vault_a                            caller
2  W   vault_b                            caller
3      mint_a                             caller
4      mint_b                             caller
5      side_price                         caller
6  W   global_state                       caller
7      instructions sysvar                SDK constant
8      SPL Token program                  SDK constant
9      Token-2022 program                 SDK constant
10     GoonFi v2 program                  SDK constant
```

### ZeroFi

```text
0  W   pool                               caller
1      oracle                             caller
2  W   side_a                             caller
3  W   vault_a                            caller
4  W   side_b                             caller
5  W   vault_b                            caller
6      mint_a                             caller
7      mint_b                             caller
8      instructions sysvar                SDK constant
9      ZeroFi program                     SDK constant
```

### SolFi v2

```text
0  W   pair                               caller
1      oracle                             caller
2      global_config                      caller
3  W   base_vault                         caller
4  W   quote_vault                        caller
5      instructions sysvar                SDK constant
6      base_mint                          caller
7      quote_mint                         caller
8      base_token_program                 caller
9      quote_token_program                caller
10     SolFi v2 program                   SDK constant
```

### Futarchy Spot

```text
0  W   dao                                caller
1  W   amm_base_vault                     caller
2  W   amm_quote_vault                    caller
3      event authority                    SDK constant
4      Futarchy program                   SDK constant
```

### Fusion

```text
0      token_program_a                    caller
1      token_program_b                    caller
2      Memo v3 program                    SDK constant
3  W   pool                               caller
4      token_mint_a                       caller
5      token_mint_b                       caller
6  W   token_vault_a                      caller
7  W   token_vault_b                      caller
8  W   tick_array_cur                     caller
9  W   tick_array_above_1                 caller
10 W   tick_array_above_2                 caller
11 W   tick_array_below_1                 caller
12 W   tick_array_below_2                 caller
13     Fusion program                     SDK constant
```

### BisonFi

```text
0  W   pool                               caller
1  W   vault_a                            caller
2  W   vault_b                            caller
3      instructions sysvar                SDK constant
4      trailing account                   SDK constant
5      BisonFi program                    SDK constant
6      Prism program                      SDK constant
```

To comply with BisonFi's account enforcement, the SDK automatically includes
the Jito vote account `J1to1yufRnoWn81KYg1XkTWzmKjnYSnmE2VY8DGUJ9Qv`.
Because Jito bundles reject transactions containing vote accounts, routes that
include BisonFi cannot be submitted through Jito bundles.

Only the pool's canonical mint A -> mint B direction is supported. Supply
`BisonFiAccounts` only when the desired route consumes mint A and produces mint B;
Prism does not scout or execute the reverse mint B -> mint A edge.

### Tessera

```text
0      global_authority                   caller
1  W   pool                               caller
2  W   mint_a_vault                       caller
3  W   mint_b_vault                       caller
4      mint_a                             caller
5      mint_b                             caller
6      instructions sysvar                SDK constant
7      Tessera program                    SDK constant
```

Tessera uses the public opcode `0x10` path. The instructions sysvar is quote-critical because
Tessera scans top-level instruction program ids to determine whether its pool fee is waived. The
alternate router-profile opcode `0x11` is intentionally not part of Prism's adapter contract.

The caller is responsible for using the correct dynamic pool, vault, mint, token program, tick/bin array, side-table, fee, and optional sentinel accounts for the selected market.

## Common On-chain Errors

These are Prism `Custom(u32)` errors callers commonly see after simulating or sending a `find_arb_v2` instruction. Solana logs the same value in hexadecimal as `custom program error: 0x...`.

Account and pool layout errors:

- `Custom(2000)` / `0x7d0` / `NotEnoughAccounts`: the account list is shorter than the declared market layouts require.
- `Custom(2012)` / `0x7dc` / `UnsupportedBaseMint`: the submitted base mint is not WSOL, USDC, USDT, or USD1.
- `Custom(2013)` / `0x7dd` / `InvalidRouteTokenAccount`: a submitted `route_mints[].user_ata` is missing, uninitialized, malformed, or owned by a different token program. Refresh it after closing/recreating the account, and supply its matching SPL Token or Token-2022 program.
- `Custom(2014)` / `0x7de` / `InvalidSignerBaseTokenAccount`: the submitted base token account is missing, uninitialized, malformed, or has a different mint from `base.mint`.
- `Custom(2015)` / `0x7df` / `InvalidFlashloanVaultAccount`: the canonical flashloan vault account is missing, uninitialized, malformed, or has a different mint from `base.mint`.
- `Custom(2016)` / `0x7e0` / `DuplicateRouteMint`: two `route_mints` entries resolve to the same mint, or one resolves to `base.mint`.
- `Custom(3000)` / `0xbb8` / `PoolOwnerMismatch`: a pool account is not owned by the expected DEX program, usually because the market variant does not match the pool.
- `Custom(3001)` / `0xbb9` / `PoolDataSize`: a pool account is too short for the expected layout, or the supplied account is stale/wrong for that market.
- `Custom(3003)` / `0xbbb` / `PoolMintMismatch`: a pool's endpoint mints do not match the submitted base mint and route mint set.
- `Custom(3005)` / `0xbbd` / `BinArrayOutOfRange`: a Meteora DLMM walk exhausted the supplied bin arrays.
- `Custom(3007)` / `0xbbf` / `BinArrayOrderInvalid`: Meteora DLMM bin arrays were supplied in the wrong order.
- `Custom(3008)` / `0xbc0` / `TickArrayWindowStale`: a CLMM tick-array window is stale for the pool's current tick.
- `Custom(3009)` / `0xbc1` / `TickArrayDirectionDead`: a CLMM pool has no initialized tick arrays on one side, so that direction cannot be executed.

Arb outcome and execution errors:

- `Custom(5000)` / `0x1388` / `NoProfit`: Prism found no profitable route. This reverts only when `fail_if_no_profit = true`; otherwise the instruction can succeed as a no-op.
- `Custom(5004)` / `0x138c` / `Unprofitable`: the strict post-swap actual-profit gate failed after execution checks.
- `Custom(5005)` / `0x138d` / `SwapCpiFailed`: a downstream DEX CPI failed; inspect the transaction logs for the DEX-level cause.
- `Custom(5006)` / `0x138e` / `FeeShortfall`: the signer's base ATA cannot cover the required settlement transfer after the swaps.

## Build Errors

`BuildError` is a client-side construction error. It means the SDK refused to build an instruction that Prism would reject or that the SDK cannot represent.

| Error | When | Notes |
| --- | --- | --- |
| `UnsupportedBaseMint(pubkey)` | `FindArbV2Params.base.mint` is not WSOL, USDC, USDT, or USD1, or the base token program is not SPL Token. | Choose one of Prism's supported settlement base mints. |
| `UnsupportedFlashloanBaseMint(pubkey)` | `flashloan = true` and `FindArbV2Params.base.mint` is not WSOL or USDC. | Disable flashloan for USDT/USD1 routes, or use a WSOL/USDC base mint with an initialized and funded Prism vault. |
| `RouteMintCountOverflow(count)` | `route_mints.len()` does not fit the `u8` count field in the instruction data. | This is the wire-format ceiling, not a routing-size recommendation. Real transactions should hit account, byte, or compute budgets first. |
| `MissingRouteMints` | `route_mints` is empty. | Prism needs at least one route mint header to map pool endpoints to user token accounts. Add one `MintAccount` for each target or bridge mint used by the pool graph. |
| `BaseMintInRouteMints(pubkey)` | A route mint entry repeats `base.mint`. | The base mint is already represented by `FindArbV2Params.base`; remove it from `route_mints`. |
| `DuplicateRouteMint(pubkey)` | The same route mint appears more than once. | Deduplicate by mint pubkey and keep the matching token program plus user ATA in the remaining `MintAccount`. |
| `MissingPools` | `pools` is empty. | Add at least one `MarketAccounts` variant. The SDK does not infer pools from mints or route intent. |
| `PoolCountOverflow(count)` | `pools.len()` does not fit the `u8` count field in the instruction data. | This is the wire-format ceiling, not a routing-size recommendation. Real transactions should hit account, byte, or compute budgets first. |
| `UnsupportedMarketId(byte)` | `markets::MarketId::try_from(byte)` received a byte outside the SDK's supported market id range. | This does not occur when building with typed `MarketAccounts` variants, because the variant supplies the market id. |

The SDK does not validate account existence, token account ownership, pool state, pool endpoint mints, route profitability, lookup table fit, compute budget, or transaction account-lock count. Those checks belong to the caller's indexer, simulator, transaction builder, or Prism itself.
