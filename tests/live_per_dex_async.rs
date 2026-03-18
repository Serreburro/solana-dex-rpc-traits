use std::str::FromStr;

use solana_client::nonblocking::rpc_client::RpcClient as AsyncRpcClient;
use solana_dex_rpc_traits::async_extensions::DexRpcClientAsyncExt;
use solana_pubkey::Pubkey;

const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const WSOL: &str = "So11111111111111111111111111111111111111112";
const random: &str = "F2XEZAd1862DMjhpSUeP51R2BMpzLyRYuv8Kk4DQ3Vwb";

fn client() -> AsyncRpcClient {
    AsyncRpcClient::new(

        std::env::var("SOLANA_RPC_URL").unwrap_or("http://lon.corvus-labs.io:8899/".to_string()),
    )
}

fn pubkey(input: &str) -> Pubkey {
    Pubkey::from_str(input).unwrap()
}

#[tokio::test]
#[ignore]
async fn orca_bidirectional_live() {
    let pools = client()
        .get_all_orca_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("orca bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn orca_single_token_live() {
    let pools = client()
        .get_all_orca_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("orca single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn meteora_dlmm_bidirectional_live() {
    let pools = client()
        .get_all_meteora_dlmm_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("meteora_dlmm bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn meteora_dlmm_single_token_live() {
    let pools = client()
        .get_all_meteora_dlmm_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("meteora_dlmm single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn meteora_damm_v2_bidirectional_live() {
    let pools = client()
        .get_all_meteora_damm_v2_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("meteora_damm_v2 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn meteora_damm_v2_single_token_live() {
    let pools = client()
        .get_all_meteora_damm_v2_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("meteora_damm_v2 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn meteora_damm_v1_bidirectional_live() {
    let pools = client()
        .get_all_meteora_damm_v1_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("meteora_damm_v1 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn meteora_damm_v1_single_token_live() {
    let pools = client()
        .get_all_meteora_damm_v1_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("meteora_damm_v1 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn raydium_clmm_bidirectional_live() {
    let pools = client()
        .get_all_raydium_clmm_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("raydium_clmm bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn raydium_clmm_single_token_live() {
    let pools = client()
        .get_all_raydium_clmm_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("raydium_clmm single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn pancake_swap_bidirectional_live() {
    let pools = client()
        .get_all_pancake_swap_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("pancake_swap bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn pancake_swap_single_token_live() {
    let pools = client()
        .get_all_pancake_swap_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("pancake_swap single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn byreal_bidirectional_live() {
    let pools = client()
        .get_all_byreal_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("byreal bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn byreal_single_token_live() {
    let pools = client()
        .get_all_byreal_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("byreal single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn pumpfun_bidirectional_live() {
    let pools = client()
        .get_all_pump_fun_amm_pools_by_tokens_bidirectional(&pubkey(random), &pubkey(WSOL))
        .await
        .unwrap();
    println!("pumpfun bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn pumpfun_single_token_live() {
    let pools = client()
        .get_all_pump_fun_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("pumpfun single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn raydium_cpmm_bidirectional_live() {
    let pools = client()
        .get_all_raydium_cpmm_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("raydium_cpmm bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn raydium_cpmm_single_token_live() {
    let pools = client()
        .get_all_raydium_cpmm_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("raydium_cpmm single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn raydium_amm_bidirectional_live() {
    let pools = client()
        .get_all_raydium_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("raydium_amm bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn raydium_amm_single_token_live() {
    let pools = client()
        .get_all_raydium_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("raydium_amm single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn saros_bidirectional_live() {
    let pools = client()
        .get_all_saros_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("saros bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn saros_single_token_live() {
    let pools = client()
        .get_all_saros_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("saros single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn orca_v2_bidirectional_live() {
    let pools = client()
        .get_all_orca_v2_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("orca_v2 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn orca_v2_single_token_live() {
    let pools = client()
        .get_all_orca_v2_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("orca_v2 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn orca_v1_bidirectional_live() {
    let pools = client()
        .get_all_orca_v1_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("orca_v1 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn orca_v1_single_token_live() {
    let pools = client()
        .get_all_orca_v1_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("orca_v1 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn stepn_bidirectional_live() {
    let pools = client()
        .get_all_stepn_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("stepn bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn stepn_single_token_live() {
    let pools = client()
        .get_all_stepn_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("stepn single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn token_swap_bidirectional_live() {
    let pools = client()
        .get_all_token_swap_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("token_swap bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn token_swap_single_token_live() {
    let pools = client()
        .get_all_token_swap_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("token_swap single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn penguin_bidirectional_live() {
    let pools = client()
        .get_all_penguin_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("penguin bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn penguin_single_token_live() {
    let pools = client()
        .get_all_penguin_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("penguin single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn fusion_bidirectional_live() {
    let pools = client()
        .get_all_fusion_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("fusion bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn fusion_single_token_live() {
    let pools = client()
        .get_all_fusion_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("fusion single token: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn obric_v2_bidirectional_live() {
    let pools = client()
        .get_all_obric_v2_amm_pools_by_tokens_bidirectional(&pubkey(USDC), &pubkey(WSOL))
        .await
        .unwrap();
    println!("obric_v2 bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn obric_v2_single_token_live() {
    let pools = client()
        .get_all_obric_v2_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("obric_v2 single token: {}", pools.len());
    assert!(!pools.is_empty());
}

/// Doesn't have WSOL/USDC.
#[tokio::test]
#[ignore]
async fn futarchy_bidirectional_live() {
    let solo_token = "SoLo9oxzLDpcq1dpqAgMwgce5WqkRDtNXK7EPnbmeta";
    let pools = client()
        .get_all_futarchy_amm_pools_by_tokens(&pubkey(USDC), &pubkey(solo_token))
        .await
        .unwrap();
    println!("futarchy bidirectional: {}", pools.len());
    assert!(!pools.is_empty());
}

#[tokio::test]
#[ignore]
async fn futarchy_single_token_live() {
    let pools = client()
        .get_all_futarchy_amm_pools_by_token(&pubkey(USDC))
        .await
        .unwrap();
    println!("futarchy single token: {}", pools.len());
    assert!(!pools.is_empty());
}
