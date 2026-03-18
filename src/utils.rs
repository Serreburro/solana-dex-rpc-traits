use solana_account::Account;
use solana_client::rpc_config::{
    RpcAccountInfoConfig, RpcProgramAccountsConfig, UiAccountEncoding,
};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_client::rpc_request::Address;
use solana_client::rpc_response::UiAccount;
use solana_pubkey::Pubkey;
use std::collections::HashSet;
use std::pin::Pin;

pub type ProgramAccount = (Pubkey, Account);
pub type ProgramAccounts = Vec<(Address, UiAccount)>;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub fn memcmp_b58(discriminator_b58: &str) -> RpcFilterType {
    RpcFilterType::Memcmp(Memcmp::new(
        0,
        MemcmpEncodedBytes::Base58(discriminator_b58.to_string()),
    ))
}
pub fn memcmp_bytes(discriminator: &[u8]) -> RpcFilterType {
    RpcFilterType::Memcmp(Memcmp::new_base58_encoded(0, discriminator))
}
pub fn memcmp_pubkey(offset: usize, mint: &Pubkey) -> RpcFilterType {
    RpcFilterType::Memcmp(Memcmp::new(
        offset,
        MemcmpEncodedBytes::Base58(mint.to_string()),
    ))
}

pub fn with_single_token_filter(
    mut base_filters: Vec<RpcFilterType>,
    mint_offset: usize,
    mint: &Pubkey,
) -> Vec<RpcFilterType> {
    base_filters.push(memcmp_pubkey(mint_offset, mint));
    base_filters
}

pub fn with_token_pair_filters(
    mut base_filters: Vec<RpcFilterType>,
    mint_a_offset: usize,
    mint_b_offset: usize,
    mint_a: &Pubkey,
    mint_b: &Pubkey,
) -> Vec<RpcFilterType> {
    base_filters.push(memcmp_pubkey(mint_a_offset, mint_a));
    base_filters.push(memcmp_pubkey(mint_b_offset, mint_b));
    base_filters
}

pub fn merge_program_accounts(
    mut first: ProgramAccounts,
    second: ProgramAccounts,
) -> ProgramAccounts {
    first.extend(second);
    first
}

pub fn intersect_program_accounts(
    first: ProgramAccounts,
    second: ProgramAccounts,
) -> ProgramAccounts {
    let second_keys: HashSet<_> = second.into_iter().map(|(address, _)| address).collect();
    first
        .into_iter()
        .filter(|(address, _)| second_keys.contains(address))
        .collect()
}

pub fn pool_fetch_config(filters: Vec<RpcFilterType>, zero_slice: bool) -> RpcProgramAccountsConfig {
    RpcProgramAccountsConfig {
        filters: Some(filters),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            data_slice: zero_slice.then_some(solana_client::rpc_config::UiDataSliceConfig { offset: 0, length: 0 }),
            commitment: None,
            min_context_slot: None,
        },
        with_context: None,
        sort_results: None,
    }
}
