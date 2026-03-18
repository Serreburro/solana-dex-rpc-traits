use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_dex_rpc_traits::sync_extensions::DexRpcClientExt;
use solana_pubkey::Pubkey;

const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const WSOL: &str = "So11111111111111111111111111111111111111112";

fn client() -> RpcClient {
    RpcClient::new(
        std::env::var("SOLANA_RPC_URL").unwrap_or("http://lon.corvus-labs.io:8899/".to_string()),
    )
}

fn pubkey(input: &str) -> Pubkey {
    Pubkey::from_str(input).unwrap()
}

#[test]
#[ignore]
fn orca_bidirectional_live() {
    let pools = client()
        .get_all_orca_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("orca bidirectional: {}", pools.len());

    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn orca_single_token_live() {
    let pools = client()
        .get_all_orca_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("orca single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn meteora_dlmm_bidirectional_live() {
    let pools = client()
        .get_all_meteora_dlmm_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("meteora_dlmm bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn meteora_dlmm_single_token_live() {
    let pools = client()
        .get_all_meteora_dlmm_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("meteora_dlmm single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn meteora_damm_v2_bidirectional_live() {
    let pools = client()
        .get_all_meteora_damm_v2_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("meteora_damm_v2 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn meteora_damm_v2_single_token_live() {
    let pools = client()
        .get_all_meteora_damm_v2_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("meteora_damm_v2 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn meteora_damm_v1_bidirectional_live() {
    let pools = client()
        .get_all_meteora_damm_v1_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("meteora_damm_v1 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn meteora_damm_v1_single_token_live() {
    let pools = client()
        .get_all_meteora_damm_v1_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("meteora_damm_v1 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn raydium_clmm_bidirectional_live() {
    let pools = client()
        .get_all_raydium_clmm_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("raydium_clmm bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn raydium_clmm_single_token_live() {
    let pools = client()
        .get_all_raydium_clmm_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("raydium_clmm single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn pancake_swap_bidirectional_live() {
    let pools = client()
        .get_all_pancake_swap_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("pancake_swap bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn pancake_swap_single_token_live() {
    let pools = client()
        .get_all_pancake_swap_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("pancake_swap single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn byreal_bidirectional_live() {
    let pools = client()
        .get_all_byreal_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("byreal bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn byreal_single_token_live() {
    let pools = client()
        .get_all_byreal_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("byreal single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn pumpfun_bidirectional_live() {
    let pools = client()
        .get_all_pump_fun_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("pumpfun bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn pumpfun_single_token_live() {
    let pools = client()
        .get_all_pump_fun_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("pumpfun single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn raydium_cpmm_bidirectional_live() {
    let pools = client()
        .get_all_raydium_cpmm_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("raydium_cpmm bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn raydium_cpmm_single_token_live() {
    let pools = client()
        .get_all_raydium_cpmm_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("raydium_cpmm single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn raydium_amm_bidirectional_live() {
    let pools = client()
        .get_all_raydium_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("raydium_amm bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn raydium_amm_single_token_live() {
    let pools = client()
        .get_all_raydium_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("raydium_amm single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn saros_bidirectional_live() {
    let pools = client()
        .get_all_saros_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("saros bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn saros_single_token_live() {
    let pools = client()
        .get_all_saros_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("saros single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn orca_v2_bidirectional_live() {
    let pools = client()
        .get_all_orca_v2_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("orca_v2 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn orca_v2_single_token_live() {
    let pools = client()
        .get_all_orca_v2_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("orca_v2 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn orca_v1_bidirectional_live() {
    let pools = client()
        .get_all_orca_v1_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("orca_v1 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn orca_v1_single_token_live() {
    let pools = client()
        .get_all_orca_v1_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("orca_v1 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn stepn_bidirectional_live() {
    let pools = client()
        .get_all_stepn_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("stepn bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn stepn_single_token_live() {
    let pools = client()
        .get_all_stepn_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("stepn single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn token_swap_bidirectional_live() {
    let pools = client()
        .get_all_token_swap_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("token_swap bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn token_swap_single_token_live() {
    let pools = client()
        .get_all_token_swap_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("token_swap single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn penguin_bidirectional_live() {
    let pools = client()
        .get_all_penguin_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("penguin bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn penguin_single_token_live() {
    let pools = client()
        .get_all_penguin_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("penguin single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn fusion_bidirectional_live() {
    let pools = client()
        .get_all_fusion_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("fusion bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn fusion_single_token_live() {
    let pools = client()
        .get_all_fusion_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("fusion single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn obric_v2_bidirectional_live() {
    let pools = client()
        .get_all_obric_v2_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL), false)
        .unwrap();
    println!("obric_v2 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn obric_v2_single_token_live() {
    let pools = client()
        .get_all_obric_v2_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("obric_v2 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

/// Doesn't have WSOL/USDC.
#[test]
#[ignore]
fn futarchy_bidirectional_live() {
    let solo_token = "SoLo9oxzLDpcq1dpqAgMwgce5WqkRDtNXK7EPnbmeta";
    let pools = client()
        .get_all_futarchy_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(solo_token), false)
        .unwrap();
    println!("futarchy bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[test]
#[ignore]
fn futarchy_single_token_live() {
    let pools = client()
        .get_all_futarchy_amm_pools_by_token(&pubkey(USDC), false)
        .unwrap();
    println!("futarchy single token: {}", pools.len());
    assert!(!pools.is_empty());
}
