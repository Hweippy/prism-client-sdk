use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

use crate::constants::{SPL_MEMO, SPL_TOKEN_2022, SYSVAR_INSTRUCTIONS};
use crate::*;
use crate::markets::{
    alphaq::*, bisonfi::*, byreal::*, fusion::*, futarchy::*, goonfi::*, humidifi::*, manifest::*,
    meteora::*, orca::*, pancakeswap::*, pumpfun::*, raydium::*, solfi::*, tessera::*, zerofi::*,
    MarketAccounts, MarketId,
};

fn unique(seed: u8) -> Pubkey {
    Pubkey::new_from_array([seed; 32])
}

fn base() -> MintAccount {
    MintAccount {
        mint: WSOL_MINT,
        token_program: spl_token_interface::ID,
        user_ata: unique(201),
    }
}

fn futarchy(seed: u8) -> MarketAccounts {
    MarketAccounts::FutarchySpot(FutarchySpotAccounts {
        dao: unique(seed),
        amm_base_vault: unique(seed + 1),
        amm_quote_vault: unique(seed + 2),
    })
}

fn params(pool: MarketAccounts) -> FindArbV2Params {
    FindArbV2Params {
        signer: unique(200),
        base: base(),
        flashloan: true,
        fail_if_no_profit: true,
        min_profit_base_units: 12_345,
        max_dynamic_walk_steps: 24,
        route_mints: vec![MintAccount {
            mint: unique(202),
            token_program: spl_token_interface::ID,
            user_ata: unique(203),
        }],
        pools: vec![pool],
    }
}

fn alphaq_accounts(token_program_left: Pubkey, token_program_right: Pubkey) -> AlphaQAccounts {
    AlphaQAccounts {
        pool: unique(1),
        market_stats: unique(2),
        vault_left: unique(3),
        vault_right: unique(4),
        mint_left: unique(5),
        mint_right: unique(6),
        token_program_left,
        token_program_right,
    }
}

fn raydium_clmm_accounts(token_program_0: Pubkey, token_program_1: Pubkey) -> RaydiumClmmAccounts {
    RaydiumClmmAccounts {
        pool_state: unique(1),
        amm_config: unique(2),
        token_vault_0: unique(3),
        token_vault_1: unique(4),
        observation_state: unique(5),
        token_mint_0: unique(6),
        token_mint_1: unique(7),
        token_program_0,
        token_program_1,
        tick_array_bitmap_ext: unique(8),
        tick_array_prev: unique(9),
        tick_array_cur: unique(10),
        tick_array_next: unique(11),
    }
}

fn orca_accounts(token_program_a: Pubkey, token_program_b: Pubkey) -> OrcaWhirlpoolAccounts {
    OrcaWhirlpoolAccounts {
        token_program_a,
        token_program_b,
        whirlpool: unique(1),
        token_mint_a: unique(2),
        token_mint_b: unique(3),
        vault_a: unique(4),
        vault_b: unique(5),
        tick_array_prev: unique(6),
        tick_array_cur: unique(7),
        tick_array_next: unique(8),
        oracle: unique(9),
    }
}

fn dlmm_accounts(token_x_program: Pubkey, token_y_program: Pubkey) -> MeteoraDlmmAccounts {
    MeteoraDlmmAccounts {
        lb_pair: unique(1),
        bin_array_bitmap_ext: unique(2),
        reserve_x: unique(3),
        reserve_y: unique(4),
        token_x_mint: unique(5),
        token_y_mint: unique(6),
        oracle: unique(7),
        host_fee_in: None,
        token_x_program,
        token_y_program,
        bin_array_prev: unique(8),
        bin_array_cur: unique(9),
        bin_array_next: unique(10),
    }
}

fn pancakeswap_accounts(token_program_0: Pubkey, token_program_1: Pubkey) -> PancakeswapAccounts {
    PancakeswapAccounts {
        pool_state: unique(1),
        amm_config: unique(2),
        token_vault_0: unique(3),
        token_vault_1: unique(4),
        observation_state: unique(5),
        token_mint_0: unique(6),
        token_mint_1: unique(7),
        token_program_0,
        token_program_1,
        tick_array_bitmap_ext: unique(8),
        tick_array_prev: unique(9),
        tick_array_cur: unique(10),
        tick_array_next: unique(11),
    }
}

fn byreal_accounts(token_program_0: Pubkey, token_program_1: Pubkey) -> ByrealClmmAccounts {
    ByrealClmmAccounts {
        pool_state: unique(1),
        amm_config: unique(2),
        token_vault_0: unique(3),
        token_vault_1: unique(4),
        observation_state: unique(5),
        token_mint_0: unique(6),
        token_mint_1: unique(7),
        token_program_0,
        token_program_1,
        tick_array_bitmap_ext: unique(8),
        tick_array_prev: unique(9),
        tick_array_cur: unique(10),
        tick_array_next: unique(11),
    }
}

fn goonfi_accounts(token_program_a: Pubkey, token_program_b: Pubkey) -> GoonfiV2Accounts {
    GoonfiV2Accounts {
        pair: unique(1),
        vault_a: unique(2),
        vault_b: unique(3),
        mint_a: unique(4),
        mint_b: unique(5),
        token_program_a,
        token_program_b,
        side_price: unique(6),
        global_state: unique(7),
    }
}

#[test]
fn builds_find_arb_v2_header_and_prefix() {
    let ix = build_find_arb_v2_instruction(params(MarketAccounts::RaydiumV4(RaydiumV4Accounts {
        pool_state: unique(1),
        coin_vault: unique(2),
        pc_vault: unique(3),
    })))
    .unwrap();

    let mut expected_data = vec![7, 0b0000_0011, 24, 1, 1];
    expected_data.extend_from_slice(&12_345u64.to_le_bytes());
    expected_data.push(MarketId::RaydiumV4.as_u8());
    assert_eq!(ix.program_id, PROGRAM_ID);
    assert_eq!(ix.data, expected_data);
    assert_eq!(ix.accounts[0], AccountMeta::new(unique(200), true));
    assert_eq!(ix.accounts[1], AccountMeta::new(unique(201), false));
    assert_eq!(ix.accounts[2], AccountMeta::new_readonly(WSOL_MINT, false));
    assert_eq!(ix.accounts[3], AccountMeta::new_readonly(spl_token_interface::ID, false));
    assert_eq!(ix.accounts[4], AccountMeta::new(VAULT_ATA_WSOL, false));
    assert_eq!(ix.accounts[5], AccountMeta::new_readonly(VAULT_AUTH, false));
    assert_eq!(ix.accounts[6], AccountMeta::new_readonly(spl_token_interface::ID, false));
    assert_eq!(ix.accounts[7], AccountMeta::new(unique(203), false));
    assert_eq!(ix.accounts[8], AccountMeta::new(unique(1), false));
    assert_eq!(ix.accounts[9], AccountMeta::new_readonly(RAYDIUM_V4_AUTHORITY, false));
    assert_eq!(ix.accounts[10], AccountMeta::new(unique(2), false));
    assert_eq!(ix.accounts[11], AccountMeta::new(unique(3), false));
    assert_eq!(ix.accounts[12], AccountMeta::new_readonly(RAYDIUM_V4, false));
}

#[test]
fn builder_automatically_selects_alphaq_token2022_wire_variant() {
    let ix = build_find_arb_v2_instruction(params(MarketAccounts::AlphaQ(alphaq_accounts(
        SPL_TOKEN_2022,
        SPL_TOKEN,
    ))))
    .unwrap();

    assert_eq!(ix.data[13], MarketId::AlphaQT22.as_u8());
    assert_eq!(ix.accounts.len(), 8 + 10);
}

#[test]
fn builder_rejects_unknown_market_token_program() {
    let token_program = unique(99);
    let err = build_find_arb_v2_instruction(params(MarketAccounts::AlphaQ(alphaq_accounts(
        token_program,
        SPL_TOKEN,
    ))))
    .unwrap_err();

    assert_eq!(
        err,
        BuildError::UnsupportedMarketTokenProgram {
            market: "AlphaQ",
            token_program,
        }
    );
}

#[test]
fn builder_rejects_alphaq_token2022_on_right_side() {
    let err = build_find_arb_v2_instruction(params(MarketAccounts::AlphaQ(alphaq_accounts(
        SPL_TOKEN,
        SPL_TOKEN_2022,
    ))))
    .unwrap_err();

    assert_eq!(
        err,
        BuildError::UnsupportedMarketTokenProgramPair {
            market: "AlphaQ",
            token_program_a: SPL_TOKEN,
            token_program_b: SPL_TOKEN_2022,
        }
    );
}

#[test]
fn public_builder_auto_selects_dlmm_t22_and_inserts_memo() {
    const FIRST_POOL_ACCOUNT: usize = 8;

    let spl_ix = build_find_arb_v2_instruction(params(MarketAccounts::MeteoraDlmm(
        dlmm_accounts(SPL_TOKEN, SPL_TOKEN),
    )))
    .unwrap();
    assert_eq!(spl_ix.data[13], MarketId::MeteoraDlmm.as_u8());
    assert_eq!(spl_ix.accounts.len(), FIRST_POOL_ACCOUNT + 15);
    assert!(!spl_ix.accounts[FIRST_POOL_ACCOUNT..]
        .iter()
        .any(|meta| meta.pubkey == SPL_MEMO));

    let t22_ix = build_find_arb_v2_instruction(params(MarketAccounts::MeteoraDlmm(
        dlmm_accounts(SPL_TOKEN_2022, SPL_TOKEN),
    )))
    .unwrap();
    assert_eq!(t22_ix.data[13], MarketId::MeteoraDlmmT22.as_u8());
    assert_eq!(t22_ix.accounts.len(), FIRST_POOL_ACCOUNT + 16);
    assert_eq!(
        t22_ix.accounts[FIRST_POOL_ACCOUNT + 10],
        AccountMeta::new_readonly(SPL_MEMO, false),
    );
}

#[test]
fn builder_resolves_unified_market_ids_counts_and_program_order() {
    let cases = vec![
        (
            "RaydiumClmm SPL",
            MarketAccounts::RaydiumClmm(raydium_clmm_accounts(SPL_TOKEN, SPL_TOKEN)),
            MarketId::RaydiumClmm,
            11,
            vec![(5, SPL_TOKEN)],
        ),
        (
            "RaydiumClmm T22",
            MarketAccounts::RaydiumClmm(raydium_clmm_accounts(SPL_TOKEN_2022, SPL_TOKEN)),
            MarketId::RaydiumClmmT22,
            15,
            vec![(5, SPL_TOKEN), (6, SPL_TOKEN_2022), (7, SPL_MEMO)],
        ),
        (
            "OrcaWhirlpool SPL",
            MarketAccounts::OrcaWhirlpool(orca_accounts(SPL_TOKEN, SPL_TOKEN)),
            MarketId::OrcaWhirlpool,
            9,
            vec![(0, SPL_TOKEN)],
        ),
        (
            "OrcaWhirlpool T22",
            MarketAccounts::OrcaWhirlpool(orca_accounts(SPL_TOKEN, SPL_TOKEN_2022)),
            MarketId::OrcaWhirlpoolT22,
            13,
            vec![(0, SPL_TOKEN), (1, SPL_TOKEN_2022), (2, SPL_MEMO)],
        ),
        (
            "MeteoraDlmm SPL",
            MarketAccounts::MeteoraDlmm(dlmm_accounts(SPL_TOKEN, SPL_TOKEN)),
            MarketId::MeteoraDlmm,
            15,
            vec![(8, SPL_TOKEN), (9, SPL_TOKEN)],
        ),
        (
            "MeteoraDlmm T22",
            MarketAccounts::MeteoraDlmm(dlmm_accounts(SPL_TOKEN_2022, SPL_TOKEN)),
            MarketId::MeteoraDlmmT22,
            16,
            vec![(8, SPL_TOKEN_2022), (9, SPL_TOKEN), (10, SPL_MEMO)],
        ),
        (
            "Pancakeswap SPL",
            MarketAccounts::Pancakeswap(pancakeswap_accounts(SPL_TOKEN, SPL_TOKEN)),
            MarketId::Pancakeswap,
            11,
            vec![(5, SPL_TOKEN)],
        ),
        (
            "Pancakeswap T22",
            MarketAccounts::Pancakeswap(pancakeswap_accounts(SPL_TOKEN, SPL_TOKEN_2022)),
            MarketId::PancakeswapT22,
            15,
            vec![(5, SPL_TOKEN), (6, SPL_TOKEN_2022), (7, SPL_MEMO)],
        ),
        (
            "ByrealClmm SPL",
            MarketAccounts::ByrealClmm(byreal_accounts(SPL_TOKEN, SPL_TOKEN)),
            MarketId::ByrealClmm,
            11,
            vec![(5, SPL_TOKEN)],
        ),
        (
            "ByrealClmm T22",
            MarketAccounts::ByrealClmm(byreal_accounts(SPL_TOKEN_2022, SPL_TOKEN)),
            MarketId::ByrealClmmT22,
            15,
            vec![(5, SPL_TOKEN), (6, SPL_TOKEN_2022), (7, SPL_MEMO)],
        ),
        (
            "AlphaQ SPL",
            MarketAccounts::AlphaQ(alphaq_accounts(SPL_TOKEN, SPL_TOKEN)),
            MarketId::AlphaQ,
            9,
            vec![(5, SPL_TOKEN)],
        ),
        (
            "AlphaQ T22",
            MarketAccounts::AlphaQ(alphaq_accounts(SPL_TOKEN_2022, SPL_TOKEN)),
            MarketId::AlphaQT22,
            10,
            vec![(5, SPL_TOKEN), (9, SPL_TOKEN_2022)],
        ),
        (
            "GoonfiV2 SPL",
            MarketAccounts::GoonfiV2(goonfi_accounts(SPL_TOKEN, SPL_TOKEN)),
            MarketId::GoonfiV2,
            10,
            vec![(8, SPL_TOKEN)],
        ),
        (
            "GoonfiV2 T22",
            MarketAccounts::GoonfiV2(goonfi_accounts(SPL_TOKEN_2022, SPL_TOKEN)),
            MarketId::GoonfiV2T22,
            11,
            vec![(8, SPL_TOKEN_2022), (9, SPL_TOKEN)],
        ),
    ];

    for (name, market, expected_id, expected_count, expected_programs) in cases {
        assert_eq!(market.try_market_id(), Ok(expected_id), "{name}");
        assert_eq!(market.try_account_count(), Ok(expected_count), "{name}");
        let ix = build_find_arb_v2_instruction(params(market)).unwrap();
        assert_eq!(ix.data[13], expected_id.as_u8(), "{name}");
        let pool_slice = &ix.accounts[8..];
        assert_eq!(pool_slice.len(), expected_count, "{name}");
        for (index, program) in expected_programs {
            assert_eq!(pool_slice[index], AccountMeta::new_readonly(program, false), "{name} index {index}");
        }
    }
}

#[test]
fn builder_rejects_unknown_programs_for_every_unified_market_family() {
    let unknown = unique(99);
    let cases = vec![
        MarketAccounts::RaydiumClmm(raydium_clmm_accounts(unknown, SPL_TOKEN)),
        MarketAccounts::OrcaWhirlpool(orca_accounts(unknown, SPL_TOKEN)),
        MarketAccounts::MeteoraDlmm(dlmm_accounts(unknown, SPL_TOKEN)),
        MarketAccounts::Pancakeswap(pancakeswap_accounts(unknown, SPL_TOKEN)),
        MarketAccounts::ByrealClmm(byreal_accounts(unknown, SPL_TOKEN)),
        MarketAccounts::AlphaQ(alphaq_accounts(unknown, SPL_TOKEN)),
        MarketAccounts::GoonfiV2(goonfi_accounts(unknown, SPL_TOKEN)),
    ];

    for market in cases {
        assert!(matches!(
            build_find_arb_v2_instruction(params(market)),
            Err(BuildError::UnsupportedMarketTokenProgram { token_program, .. }) if token_program == unknown
        ));
        assert!(market.try_market_id().is_err());
        assert!(market.try_account_count().is_err());
    }
}

#[test]
fn non_flashloan_uses_fee_ata_without_vault_auth() {
    let mut params = params(futarchy(1));
    params.flashloan = false;
    params.fail_if_no_profit = false;
    params.base.mint = USDT_MINT;

    let ix = build_find_arb_v2_instruction(params).unwrap();
    assert_eq!(ix.data[1], 0);
    assert_eq!(ix.accounts[4], AccountMeta::new(FEE_ATA_USDT, false));
    assert_eq!(ix.accounts[5], AccountMeta::new_readonly(spl_token_interface::ID, false));
}

#[test]
fn usdc_flashloan_uses_shared_vault_authority() {
    let mut params = params(futarchy(1));
    params.base.mint = USDC_MINT;

    let ix = build_find_arb_v2_instruction(params).unwrap();
    assert_eq!(ix.accounts[4], AccountMeta::new(VAULT_ATA_USDC, false));
    assert_eq!(ix.accounts[5], AccountMeta::new_readonly(VAULT_AUTH, false));
}

#[test]
fn derived_settlement_addresses_match_constants() {
    assert_eq!(
        associated_token_address(VAULT_AUTH, WSOL_MINT, SPL_TOKEN),
        VAULT_ATA_WSOL
    );
    assert_eq!(
        associated_token_address(VAULT_AUTH, USDC_MINT, SPL_TOKEN),
        VAULT_ATA_USDC
    );
    assert_eq!(
        associated_token_address(VAULT_AUTH, USDT_MINT, SPL_TOKEN),
        FEE_ATA_USDT
    );
    assert_eq!(
        associated_token_address(VAULT_AUTH, USD1_MINT, SPL_TOKEN),
        FEE_ATA_USD1
    );
    assert_eq!(prism_fee_recipient_ata(USDC_MINT, SPL_TOKEN), Ok(VAULT_ATA_USDC));
    assert_eq!(prism_fee_recipient_ata(USDT_MINT, SPL_TOKEN), Ok(FEE_ATA_USDT));
    assert_eq!(prism_fee_recipient_ata(USD1_MINT, SPL_TOKEN), Ok(FEE_ATA_USD1));
}

#[test]
fn rejects_wire_count_overflow() {
    let mut too_many_mints = params(futarchy(1));
    too_many_mints.route_mints = (0..=u8::MAX)
        .map(|seed| MintAccount {
            mint: unique(seed),
            token_program: spl_token_interface::ID,
            user_ata: unique(seed.wrapping_add(1)),
        })
        .collect();
    assert!(matches!(
        build_find_arb_v2_instruction(too_many_mints),
        Err(BuildError::RouteMintCountOverflow(256))
    ));

    let mut too_many_pools = params(futarchy(1));
    too_many_pools.pools = vec![futarchy(1); usize::from(u8::MAX) + 1];
    assert!(matches!(
        build_find_arb_v2_instruction(too_many_pools),
        Err(BuildError::PoolCountOverflow(256))
    ));
}

#[test]
fn validates_base_mint() {
    let mut unsupported_base = params(futarchy(1));
    unsupported_base.flashloan = false;
    unsupported_base.base.mint = unique(77);
    assert!(matches!(
        build_find_arb_v2_instruction(unsupported_base),
        Err(BuildError::UnsupportedBaseMint(_))
    ));

    let mut unsupported_flashloan_base = params(futarchy(1));
    unsupported_flashloan_base.base.mint = USDT_MINT;
    assert!(matches!(
        build_find_arb_v2_instruction(unsupported_flashloan_base),
        Err(BuildError::UnsupportedFlashloanBaseMint(_))
    ));
}

#[test]
fn supported_base_mint_list_has_four_entries() {
    assert!(prism_base_mint_supported(WSOL_MINT));
    assert!(prism_base_mint_supported(USDC_MINT));
    assert!(prism_base_mint_supported(USDT_MINT));
    assert!(prism_base_mint_supported(USD1_MINT));
    assert!(!prism_base_mint_supported(unique(77)));
}

#[test]
fn validates_route_mint_headers() {
    let mut missing = params(futarchy(1));
    missing.route_mints.clear();
    assert_eq!(build_find_arb_v2_instruction(missing), Err(BuildError::MissingRouteMints));

    let mut includes_base = params(futarchy(1));
    includes_base.route_mints[0].mint = includes_base.base.mint;
    assert_eq!(
        build_find_arb_v2_instruction(includes_base),
        Err(BuildError::BaseMintInRouteMints(WSOL_MINT))
    );

    let mut duplicate = params(futarchy(1));
    duplicate.route_mints.push(duplicate.route_mints[0]);
    assert_eq!(
        build_find_arb_v2_instruction(duplicate),
        Err(BuildError::DuplicateRouteMint(unique(202)))
    );
}

#[test]
fn typed_market_variant_sets_market_id_without_runtime_pairing() {
    let mut params = params(futarchy(1));
    params.pools.push(MarketAccounts::RaydiumCp(RaydiumCpAccounts {
        pool_state: unique(10),
        amm_config: unique(11),
        token_0_vault: unique(12),
        token_1_vault: unique(13),
        token_0_mint: unique(14),
        token_1_mint: unique(15),
        observation_state: unique(16),
    }));
    let ix = build_find_arb_v2_instruction(params).unwrap();
    assert_eq!(ix.data[13], MarketId::FutarchySpot.as_u8());
    assert_eq!(ix.data[14], MarketId::RaydiumCp.as_u8());
}

#[test]
fn tessera_emits_public_swap_account_slice() {
    let accounts = TesseraAccounts {
        global_authority: unique(1),
        pool: unique(2),
        mint_a_vault: unique(3),
        mint_b_vault: unique(4),
        mint_a: unique(5),
        mint_b: unique(6),
    };
    let market = MarketAccounts::Tessera(accounts);
    let mut metas = Vec::new();
    market.try_append_account_metas(&mut metas).unwrap();

    assert_eq!(market.try_market_id().unwrap(), MarketId::Tessera);
    assert_eq!(market.try_account_count().unwrap(), 8);
    assert_eq!(
        metas,
        vec![
            AccountMeta::new_readonly(accounts.global_authority, false),
            AccountMeta::new(accounts.pool, false),
            AccountMeta::new(accounts.mint_a_vault, false),
            AccountMeta::new(accounts.mint_b_vault, false),
            AccountMeta::new_readonly(accounts.mint_a, false),
            AccountMeta::new_readonly(accounts.mint_b, false),
            AccountMeta::new_readonly(SYSVAR_INSTRUCTIONS, false),
            AccountMeta::new_readonly(TESSERA, false),
        ]
    );
}

#[test]
fn bisonfi_emits_forward_swap_account_slice() {
    let accounts = BisonFiAccounts {
        pool: unique(1),
        vault_a: unique(2),
        vault_b: unique(3),
    };
    let market = MarketAccounts::BisonFi(accounts);
    let mut metas = Vec::new();
    market.try_append_account_metas(&mut metas).unwrap();

    assert_eq!(market.try_market_id().unwrap(), MarketId::BisonFi);
    assert_eq!(market.try_account_count().unwrap(), 7);
    assert_eq!(
        metas,
        vec![
            AccountMeta::new(accounts.pool, false),
            AccountMeta::new(accounts.vault_a, false),
            AccountMeta::new(accounts.vault_b, false),
            AccountMeta::new_readonly(SYSVAR_INSTRUCTIONS, false),
            AccountMeta::new_readonly(BISONFI_TRAILING_ACCOUNT, false),
            AccountMeta::new_readonly(BISONFI, false),
            AccountMeta::new_readonly(PROGRAM_ID, false),
        ]
    );
}

#[test]
fn zerofi_emits_swap_v4_account_slice() {
    let accounts = ZeroFiAccounts {
        pool: unique(1),
        oracle: unique(2),
        side_a: unique(3),
        vault_a: unique(4),
        side_b: unique(5),
        vault_b: unique(6),
        mint_a: unique(7),
        mint_b: unique(8),
    };
    let market = MarketAccounts::ZeroFi(accounts);
    let mut metas = Vec::new();
    market.try_append_account_metas(&mut metas).unwrap();

    assert_eq!(market.try_market_id().unwrap(), MarketId::ZeroFi);
    assert_eq!(market.try_account_count().unwrap(), 10);
    assert_eq!(
        metas,
        vec![
            AccountMeta::new(accounts.pool, false),
            AccountMeta::new_readonly(accounts.oracle, false),
            AccountMeta::new(accounts.side_a, false),
            AccountMeta::new(accounts.vault_a, false),
            AccountMeta::new(accounts.side_b, false),
            AccountMeta::new(accounts.vault_b, false),
            AccountMeta::new_readonly(accounts.mint_a, false),
            AccountMeta::new_readonly(accounts.mint_b, false),
            AccountMeta::new_readonly(SYSVAR_INSTRUCTIONS, false),
            AccountMeta::new_readonly(ZEROFI, false),
        ]
    );
}

#[test]
fn goonfi_v2_automatically_emits_mixed_token_program_account_slice() {
    let accounts = GoonfiV2Accounts {
        pair: unique(1),
        vault_a: unique(2),
        vault_b: unique(3),
        mint_a: unique(4),
        mint_b: unique(5),
        token_program_a: SPL_TOKEN_2022,
        token_program_b: SPL_TOKEN,
        side_price: unique(6),
        global_state: unique(7),
    };
    let market = MarketAccounts::GoonfiV2(accounts);
    let mut metas = Vec::new();
    market.try_append_account_metas(&mut metas).unwrap();

    assert_eq!(market.try_market_id().unwrap(), MarketId::GoonfiV2T22);
    assert_eq!(market.try_account_count().unwrap(), 11);
    assert_eq!(
        metas,
        vec![
            AccountMeta::new(accounts.pair, false),
            AccountMeta::new(accounts.vault_a, false),
            AccountMeta::new(accounts.vault_b, false),
            AccountMeta::new_readonly(accounts.mint_a, false),
            AccountMeta::new_readonly(accounts.mint_b, false),
            AccountMeta::new_readonly(accounts.side_price, false),
            AccountMeta::new(accounts.global_state, false),
            AccountMeta::new_readonly(SYSVAR_INSTRUCTIONS, false),
            AccountMeta::new_readonly(SPL_TOKEN_2022, false),
            AccountMeta::new_readonly(SPL_TOKEN, false),
            AccountMeta::new_readonly(GOONFI_V2, false),
        ]
    );
}

#[test]
#[allow(deprecated)]
fn deprecated_goonfi_v2_t22_alias_preserves_wire_output() {
    let accounts = GoonfiV2Accounts {
        pair: unique(1),
        vault_a: unique(2),
        vault_b: unique(3),
        mint_a: unique(4),
        mint_b: unique(5),
        token_program_a: SPL_TOKEN_2022,
        token_program_b: SPL_TOKEN,
        side_price: unique(6),
        global_state: unique(7),
    };

    let automatic = build_find_arb_v2_instruction(params(MarketAccounts::GoonfiV2(accounts))).unwrap();
    let legacy = build_find_arb_v2_instruction(params(MarketAccounts::GoonfiV2T22(accounts))).unwrap();

    assert_eq!(legacy.data, automatic.data);
    assert_eq!(legacy.accounts, automatic.accounts);
}

#[test]
#[allow(deprecated)]
fn deprecated_t22_aliases_validate_programs_and_builder_metadata() {
    let cases = vec![
        (
            MarketAccounts::MeteoraDlmmT22(dlmm_accounts(SPL_TOKEN_2022, SPL_TOKEN)),
            MarketId::MeteoraDlmmT22,
            16,
        ),
        (
            MarketAccounts::OrcaWhirlpoolT22(OrcaWhirlpoolT22Accounts {
                token_program_a: SPL_TOKEN,
                token_program_b: SPL_TOKEN_2022,
                whirlpool: unique(1),
                token_mint_a: unique(2),
                token_mint_b: unique(3),
                vault_a: unique(4),
                vault_b: unique(5),
                tick_array_prev: unique(6),
                tick_array_cur: unique(7),
                tick_array_next: unique(8),
                oracle: unique(9),
            }),
            MarketId::OrcaWhirlpoolT22,
            13,
        ),
        (
            MarketAccounts::AlphaQT22(alphaq_accounts(SPL_TOKEN_2022, SPL_TOKEN)),
            MarketId::AlphaQT22,
            10,
        ),
        (
            MarketAccounts::GoonfiV2T22(goonfi_accounts(SPL_TOKEN_2022, SPL_TOKEN)),
            MarketId::GoonfiV2T22,
            11,
        ),
    ];

    for (market, expected_id, expected_count) in cases {
        assert_eq!(market.try_market_id(), Ok(expected_id));
        assert_eq!(market.try_account_count(), Ok(expected_count));
        let ix = build_find_arb_v2_instruction(params(market)).unwrap();
        assert_eq!(ix.data[13], expected_id.as_u8());
        assert_eq!(ix.accounts.len(), 8 + expected_count);
    }

    let invalid_pairs = vec![
        MarketAccounts::MeteoraDlmmT22(dlmm_accounts(SPL_TOKEN, SPL_TOKEN)),
        MarketAccounts::OrcaWhirlpoolT22(OrcaWhirlpoolT22Accounts {
            token_program_a: SPL_TOKEN,
            token_program_b: SPL_TOKEN,
            whirlpool: unique(1),
            token_mint_a: unique(2),
            token_mint_b: unique(3),
            vault_a: unique(4),
            vault_b: unique(5),
            tick_array_prev: unique(6),
            tick_array_cur: unique(7),
            tick_array_next: unique(8),
            oracle: unique(9),
        }),
        MarketAccounts::AlphaQT22(alphaq_accounts(SPL_TOKEN, SPL_TOKEN)),
        MarketAccounts::AlphaQT22(alphaq_accounts(SPL_TOKEN_2022, SPL_TOKEN_2022)),
        MarketAccounts::GoonfiV2T22(goonfi_accounts(SPL_TOKEN, SPL_TOKEN)),
    ];
    for market in invalid_pairs {
        assert!(matches!(
            build_find_arb_v2_instruction(params(market)),
            Err(BuildError::UnsupportedMarketTokenProgramPair { .. })
        ));
    }

    let unknown = unique(99);
    let invalid = vec![
        MarketAccounts::MeteoraDlmmT22(dlmm_accounts(unknown, SPL_TOKEN_2022)),
        MarketAccounts::OrcaWhirlpoolT22(OrcaWhirlpoolT22Accounts {
            token_program_a: unknown,
            token_program_b: SPL_TOKEN_2022,
            whirlpool: unique(1),
            token_mint_a: unique(2),
            token_mint_b: unique(3),
            vault_a: unique(4),
            vault_b: unique(5),
            tick_array_prev: unique(6),
            tick_array_cur: unique(7),
            tick_array_next: unique(8),
            oracle: unique(9),
        }),
        MarketAccounts::AlphaQT22(alphaq_accounts(unknown, SPL_TOKEN)),
        MarketAccounts::GoonfiV2T22(goonfi_accounts(unknown, SPL_TOKEN_2022)),
    ];
    for market in invalid {
        assert!(matches!(
            build_find_arb_v2_instruction(params(market)),
            Err(BuildError::UnsupportedMarketTokenProgram { token_program, .. }) if token_program == unknown
        ));
    }
}

#[test]
#[allow(deprecated)]
fn legacy_t22_aliases_without_program_fields_remain_forced_compatibility_paths() {
    let cases = vec![
        (
            MarketAccounts::RaydiumClmmT22(RaydiumClmmT22Accounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: unique(10),
                tick_array_next: unique(11),
            }),
            MarketId::RaydiumClmmT22,
        ),
        (
            MarketAccounts::PancakeswapT22(PancakeswapT22Accounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: unique(10),
                tick_array_next: unique(11),
            }),
            MarketId::PancakeswapT22,
        ),
        (
            MarketAccounts::ByrealClmmT22(ByrealClmmT22Accounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: unique(10),
                tick_array_next: unique(11),
            }),
            MarketId::ByrealClmmT22,
        ),
    ];

    for (market, expected_id) in cases {
        assert_eq!(market.try_market_id(), Ok(expected_id));
        let ix = build_find_arb_v2_instruction(params(market)).unwrap();
        assert_eq!(ix.data[13], expected_id.as_u8());
        assert_eq!(ix.accounts.len(), 8 + 15);
    }
}

#[test]
fn all_market_slices_have_expected_writable_orders() {
    let cases = vec![
        (
            MarketAccounts::RaydiumV4(RaydiumV4Accounts {
                pool_state: unique(1),
                coin_vault: unique(2),
                pc_vault: unique(3),
            }),
            vec![0, 2, 3],
        ),
        (
            MarketAccounts::RaydiumCp(RaydiumCpAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_0_vault: unique(3),
                token_1_vault: unique(4),
                token_0_mint: unique(5),
                token_1_mint: unique(6),
                observation_state: unique(7),
            }),
            vec![0, 3, 4, 7],
        ),
        (
            MarketAccounts::RaydiumClmm(RaydiumClmmAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(10),
                token_mint_1: unique(11),
                token_program_0: SPL_TOKEN,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(6),
                tick_array_prev: unique(7),
                tick_array_cur: unique(8),
                tick_array_next: unique(9),
            }),
            vec![1, 2, 3, 4, 7, 8, 9, 10],
        ),
        (
            MarketAccounts::OrcaWhirlpool(OrcaWhirlpoolAccounts {
                token_program_a: SPL_TOKEN,
                token_program_b: SPL_TOKEN,
                whirlpool: unique(1),
                token_mint_a: unique(8),
                token_mint_b: unique(9),
                vault_a: unique(2),
                vault_b: unique(3),
                tick_array_prev: unique(4),
                tick_array_cur: unique(5),
                tick_array_next: unique(6),
                oracle: unique(7),
            }),
            vec![1, 2, 3, 4, 5, 6, 7],
        ),
        (
            MarketAccounts::MeteoraDlmm(MeteoraDlmmAccounts {
                lb_pair: unique(1),
                bin_array_bitmap_ext: unique(2),
                reserve_x: unique(3),
                reserve_y: unique(4),
                token_x_mint: unique(5),
                token_y_mint: unique(6),
                oracle: unique(7),
                host_fee_in: Some(unique(8)),
                token_x_program: SPL_TOKEN,
                token_y_program: SPL_TOKEN,
                bin_array_prev: unique(11),
                bin_array_cur: unique(12),
                bin_array_next: unique(13),
            }),
            vec![0, 1, 2, 3, 6, 7, 12, 13, 14],
        ),
        (
            MarketAccounts::MeteoraDammV2(MeteoraDammV2Accounts {
                pool: unique(1),
                token_a_vault: unique(2),
                token_b_vault: unique(3),
                token_a_mint: unique(4),
                token_b_mint: unique(5),
                token_a_program: unique(6),
                token_b_program: unique(7),
                instructions_sysvar_or_program_sentinel: Some(unique(8)),
            }),
            vec![1, 2, 3],
        ),
        (
            MarketAccounts::MeteoraPools(MeteoraPoolsAccounts {
                pool: unique(1),
                a_vault: unique(2),
                b_vault: unique(3),
                a_token_vault: unique(4),
                b_token_vault: unique(5),
                a_vault_lp_mint: unique(6),
                b_vault_lp_mint: unique(7),
                a_vault_lp: unique(8),
                b_vault_lp: unique(9),
                protocol_fee_a: unique(10),
                protocol_fee_b: unique(11),
                vault_program: unique(12),
            }),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        ),
        (
            MarketAccounts::PumpfunAmm(PumpfunAmmAccounts {
                pool: unique(1),
                base_mint: unique(2),
                quote_mint: unique(3),
                pool_base_vault: unique(4),
                pool_quote_vault: unique(5),
                fee_recipient: unique(6),
                fee_recipient_ata: unique(7),
                base_token_program: unique(8),
                quote_token_program: unique(9),
                creator_vault_ata: unique(10),
                creator_vault_authority: unique(11),
                user_volume_accumulator: unique(12),
                user_volume_accumulator_wsol_ata: Some(unique(13)),
                pool_v2: Some(unique(14)),
                swap_fee_recipient: unique(15),
                swap_fee_ata: unique(16),
            }),
            vec![0, 4, 5, 7, 14, 17, 20, 23],
        ),
        (
            MarketAccounts::Pancakeswap(PancakeswapAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(10),
                token_mint_1: unique(11),
                token_program_0: SPL_TOKEN,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(6),
                tick_array_prev: unique(7),
                tick_array_cur: unique(8),
                tick_array_next: unique(9),
            }),
            vec![1, 2, 3, 4, 7, 8, 9, 10],
        ),
        (
            MarketAccounts::MeteoraDlmm(MeteoraDlmmAccounts {
                lb_pair: unique(1),
                bin_array_bitmap_ext: unique(2),
                reserve_x: unique(3),
                reserve_y: unique(4),
                token_x_mint: unique(5),
                token_y_mint: unique(6),
                oracle: unique(7),
                host_fee_in: Some(unique(8)),
                token_x_program: SPL_TOKEN_2022,
                token_y_program: SPL_TOKEN,
                bin_array_prev: unique(11),
                bin_array_cur: unique(12),
                bin_array_next: unique(13),
            }),
            vec![0, 1, 2, 3, 6, 7, 13, 14, 15],
        ),
        (
            MarketAccounts::RaydiumClmm(RaydiumClmmAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                token_program_0: SPL_TOKEN_2022,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: unique(10),
                tick_array_next: unique(11),
            }),
            vec![1, 2, 3, 4, 11, 12, 13, 14],
        ),
        (
            MarketAccounts::Pancakeswap(PancakeswapAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                token_program_0: SPL_TOKEN,
                token_program_1: SPL_TOKEN_2022,
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: unique(10),
                tick_array_next: unique(11),
            }),
            vec![1, 2, 3, 4, 11, 12, 13, 14],
        ),
        (
            MarketAccounts::OrcaWhirlpool(OrcaWhirlpoolAccounts {
                token_program_a: SPL_TOKEN_2022,
                token_program_b: SPL_TOKEN,
                whirlpool: unique(3),
                token_mint_a: unique(4),
                token_mint_b: unique(5),
                vault_a: unique(6),
                vault_b: unique(7),
                tick_array_prev: unique(8),
                tick_array_cur: unique(9),
                tick_array_next: unique(10),
                oracle: unique(11),
            }),
            vec![3, 6, 7, 8, 9, 10, 11],
        ),
        (
            MarketAccounts::ByrealClmm(ByrealClmmAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(10),
                token_mint_1: unique(11),
                token_program_0: SPL_TOKEN,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(6),
                tick_array_prev: unique(7),
                tick_array_cur: unique(8),
                tick_array_next: unique(9),
            }),
            vec![1, 2, 3, 4, 7, 8, 9, 10],
        ),
        (
            MarketAccounts::ByrealClmm(ByrealClmmAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                token_program_0: SPL_TOKEN_2022,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: unique(10),
                tick_array_next: unique(11),
            }),
            vec![1, 2, 3, 4, 11, 12, 13, 14],
        ),
        (
            MarketAccounts::HumidifiSwapV2(HumidifiSwapV2Accounts {
                pool: unique(1),
                base_vault: unique(2),
                quote_vault: unique(3),
                base_mint: unique(4),
                quote_mint: unique(5),
                side_table: unique(6),
                jito_vote: unique(7),
            }),
            vec![0, 1, 2, 7],
        ),
        (
            MarketAccounts::HumidifiSwap(HumidifiSwapAccounts {
                pool: unique(1),
                base_vault: unique(2),
                quote_vault: unique(3),
                jito_vote: unique(4),
            }),
            vec![0, 1, 2],
        ),
        (
            MarketAccounts::Manifest(ManifestAccounts {
                market: unique(1),
                base_vault: unique(2),
                quote_vault: unique(3),
                base_mint: unique(4),
                quote_mint: unique(5),
            }),
            vec![0, 1, 2],
        ),
        (
            MarketAccounts::AlphaQ(AlphaQAccounts {
                pool: unique(1),
                market_stats: unique(2),
                vault_left: unique(3),
                vault_right: unique(4),
                mint_left: unique(5),
                mint_right: unique(6),
                token_program_left: SPL_TOKEN,
                token_program_right: SPL_TOKEN,
            }),
            vec![0, 1, 2, 3],
        ),
        (
            MarketAccounts::AlphaQ(AlphaQAccounts {
                pool: unique(1),
                market_stats: unique(2),
                vault_left: unique(3),
                vault_right: unique(4),
                mint_left: unique(5),
                mint_right: unique(6),
                token_program_left: SPL_TOKEN_2022,
                token_program_right: SPL_TOKEN,
            }),
            vec![0, 1, 2, 3],
        ),
        (
            MarketAccounts::GoonfiV2(GoonfiV2Accounts {
                pair: unique(1),
                vault_a: unique(2),
                vault_b: unique(3),
                mint_a: unique(4),
                mint_b: unique(5),
                token_program_a: SPL_TOKEN,
                token_program_b: SPL_TOKEN,
                side_price: unique(6),
                global_state: unique(7),
            }),
            vec![0, 1, 2, 6],
        ),
        (
            MarketAccounts::GoonfiV2(GoonfiV2Accounts {
                pair: unique(1),
                vault_a: unique(2),
                vault_b: unique(3),
                mint_a: unique(4),
                mint_b: unique(5),
                token_program_a: SPL_TOKEN_2022,
                token_program_b: SPL_TOKEN,
                side_price: unique(6),
                global_state: unique(7),
            }),
            vec![0, 1, 2, 6],
        ),
        (
            MarketAccounts::SolfiV2(SolfiV2Accounts {
                pair: unique(1),
                oracle: unique(2),
                global_config: unique(3),
                base_vault: unique(4),
                quote_vault: unique(5),
                base_mint: unique(6),
                quote_mint: unique(7),
                base_token_program: unique(8),
                quote_token_program: unique(9),
            }),
            vec![0, 3, 4],
        ),
        (futarchy(1), vec![0, 1, 2]),
        (
            MarketAccounts::Fusion(FusionAccounts {
                token_program_a: unique(1),
                token_program_b: unique(2),
                pool: unique(3),
                token_mint_a: unique(4),
                token_mint_b: unique(5),
                token_vault_a: unique(6),
                token_vault_b: unique(7),
                tick_array_cur: unique(8),
                tick_array_above_1: unique(9),
                tick_array_above_2: unique(10),
                tick_array_below_1: unique(11),
                tick_array_below_2: unique(12),
            }),
            vec![3, 6, 7, 8, 9, 10, 11, 12],
        ),
        (
            MarketAccounts::BisonFi(BisonFiAccounts {
                pool: unique(1),
                vault_a: unique(2),
                vault_b: unique(3),
            }),
            vec![0, 1, 2],
        ),
        (
            MarketAccounts::Tessera(TesseraAccounts {
                global_authority: unique(1),
                pool: unique(2),
                mint_a_vault: unique(3),
                mint_b_vault: unique(4),
                mint_a: unique(5),
                mint_b: unique(6),
            }),
            vec![1, 2, 3],
        ),
        (
            MarketAccounts::ZeroFi(ZeroFiAccounts {
                pool: unique(1),
                oracle: unique(2),
                side_a: unique(3),
                vault_a: unique(4),
                side_b: unique(5),
                vault_b: unique(6),
                mint_a: unique(7),
                mint_b: unique(8),
            }),
            vec![0, 2, 3, 4, 5],
        ),
    ];

    for (accounts, writable_indexes) in cases {
        let mut metas = Vec::new();
        accounts.try_append_account_metas(&mut metas).unwrap();
        assert_eq!(metas.len(), accounts.try_account_count().unwrap(), "{accounts:?}");
        for (index, meta) in metas.iter().enumerate() {
            assert_eq!(
                meta.is_writable,
                writable_indexes.contains(&index),
                "{accounts:?} index {}",
                index
            );
            assert!(!meta.is_signer);
        }
    }
}

#[test]
fn documented_placeholders_use_market_program_sentinels() {
    let dlmm = MarketAccounts::MeteoraDlmm(MeteoraDlmmAccounts {
        lb_pair: unique(1),
        bin_array_bitmap_ext: unique(2),
        reserve_x: unique(3),
        reserve_y: unique(4),
        token_x_mint: unique(5),
        token_y_mint: unique(6),
        oracle: unique(7),
        host_fee_in: None,
        token_x_program: SPL_TOKEN,
        token_y_program: SPL_TOKEN,
        bin_array_prev: unique(10),
        bin_array_cur: unique(11),
        bin_array_next: unique(12),
    });
    let mut metas = Vec::new();
    dlmm.try_append_account_metas(&mut metas).unwrap();
    assert_eq!(metas[7], AccountMeta::new(METEORA_DLMM, false));

    let damm = MarketAccounts::MeteoraDammV2(MeteoraDammV2Accounts {
        pool: unique(1),
        token_a_vault: unique(2),
        token_b_vault: unique(3),
        token_a_mint: unique(4),
        token_b_mint: unique(5),
        token_a_program: unique(6),
        token_b_program: unique(7),
        instructions_sysvar_or_program_sentinel: None,
    });
    metas.clear();
    damm.try_append_account_metas(&mut metas).unwrap();
    assert_eq!(metas[10], AccountMeta::new_readonly(METEORA_DAMM_V2, false));

    let pump = MarketAccounts::PumpfunAmm(PumpfunAmmAccounts {
        pool: unique(1),
        base_mint: unique(2),
        quote_mint: unique(3),
        pool_base_vault: unique(4),
        pool_quote_vault: unique(5),
        fee_recipient: unique(6),
        fee_recipient_ata: unique(7),
        base_token_program: unique(8),
        quote_token_program: unique(9),
        creator_vault_ata: unique(10),
        creator_vault_authority: unique(11),
        user_volume_accumulator: unique(12),
        user_volume_accumulator_wsol_ata: None,
        pool_v2: None,
        swap_fee_recipient: unique(13),
        swap_fee_ata: unique(14),
    });
    metas.clear();
    pump.try_append_account_metas(&mut metas).unwrap();
    assert_eq!(metas[20], AccountMeta::new_readonly(PUMPFUN_AMM, false));
    assert_eq!(metas[21], AccountMeta::new_readonly(PUMPFUN_AMM, false));
}

#[test]
fn clmm_current_tick_array_accepts_program_id_placeholder() {
    let cases = [
        (
            MarketAccounts::RaydiumClmm(RaydiumClmmAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(9),
                token_mint_1: unique(10),
                token_program_0: SPL_TOKEN,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(6),
                tick_array_prev: unique(7),
                tick_array_cur: RAYDIUM_CLMM,
                tick_array_next: unique(8),
            }),
            9usize,
            RAYDIUM_CLMM,
        ),
        (
            MarketAccounts::RaydiumClmm(RaydiumClmmAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                token_program_0: SPL_TOKEN_2022,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: RAYDIUM_CLMM,
                tick_array_next: unique(10),
            }),
            13usize,
            RAYDIUM_CLMM,
        ),
        (
            MarketAccounts::Pancakeswap(PancakeswapAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(9),
                token_mint_1: unique(10),
                token_program_0: SPL_TOKEN,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(6),
                tick_array_prev: unique(7),
                tick_array_cur: PANCAKESWAP,
                tick_array_next: unique(8),
            }),
            9usize,
            PANCAKESWAP,
        ),
        (
            MarketAccounts::Pancakeswap(PancakeswapAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                token_program_0: SPL_TOKEN_2022,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: PANCAKESWAP,
                tick_array_next: unique(10),
            }),
            13usize,
            PANCAKESWAP,
        ),
        (
            MarketAccounts::ByrealClmm(ByrealClmmAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(9),
                token_mint_1: unique(10),
                token_program_0: SPL_TOKEN,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(6),
                tick_array_prev: unique(7),
                tick_array_cur: BYREAL_CLMM,
                tick_array_next: unique(8),
            }),
            9usize,
            BYREAL_CLMM,
        ),
        (
            MarketAccounts::ByrealClmm(ByrealClmmAccounts {
                pool_state: unique(1),
                amm_config: unique(2),
                token_vault_0: unique(3),
                token_vault_1: unique(4),
                observation_state: unique(5),
                token_mint_0: unique(6),
                token_mint_1: unique(7),
                token_program_0: SPL_TOKEN_2022,
                token_program_1: SPL_TOKEN,
                tick_array_bitmap_ext: unique(8),
                tick_array_prev: unique(9),
                tick_array_cur: BYREAL_CLMM,
                tick_array_next: unique(10),
            }),
            13usize,
            BYREAL_CLMM,
        ),
    ];

    for (accounts, cur_index, program_id) in cases {
        let mut metas = Vec::new();
        accounts.try_append_account_metas(&mut metas).unwrap();
        assert_eq!(metas[cur_index], AccountMeta::new(program_id, false));
    }
}

#[test]
fn market_id_try_from_covers_current_range() {
    for id in 0..=26 {
        assert_eq!(MarketId::try_from(id).unwrap().as_u8(), id);
    }
    assert_eq!(MarketId::try_from(27), Err(BuildError::UnsupportedMarketId(27)));
    assert_eq!(MarketId::try_from(28).unwrap(), MarketId::GoonfiV2T22);
    assert_eq!(MarketId::try_from(29), Err(BuildError::UnsupportedMarketId(29)));
}
