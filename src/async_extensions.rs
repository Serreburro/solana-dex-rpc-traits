use crate::globals::*;
use crate::utils::{
    BoxFuture, ProgramAccounts, memcmp_b58, memcmp_bytes, merge_program_accounts,
    pool_fetch_config, with_single_token_filter, with_token_pair_filters,
};
use solana_client::client_error::Result as ClientResult;
use solana_client::rpc_filter::RpcFilterType;
use solana_pubkey::Pubkey;

pub trait DexRpcClientAsyncExt {
    fn get_all_pools_with_filters(
        &self,
        program_id: Pubkey,
        filters: Vec<RpcFilterType>,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>>;

    fn get_pools_by_token_offsets(
        &self,
        program_id: Pubkey,
        base_filters: Vec<RpcFilterType>,
        mint_a_offset: usize,
        mint_b_offset: usize,
        mint_a: &Pubkey,
        mint_b: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        let filters =
            with_token_pair_filters(base_filters, mint_a_offset, mint_b_offset, mint_a, mint_b);
        self.get_all_pools_with_filters(program_id, filters)
    }

    fn get_pools_by_single_token_offset(
        &self,
        program_id: Pubkey,
        base_filters: Vec<RpcFilterType>,
        mint_offset: usize,
        mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        let filters = with_single_token_filter(base_filters, mint_offset, mint);
        self.get_all_pools_with_filters(program_id, filters)
    }

    fn get_pools_by_token_offsets_bidirectional(
        &self,
        program_id: Pubkey,
        base_filters: Vec<RpcFilterType>,
        mint_a_offset: usize,
        mint_b_offset: usize,
        mint_a: &Pubkey,
        mint_b: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        if mint_a == mint_b {
            return self.get_pools_by_token_offsets(
                program_id,
                base_filters,
                mint_a_offset,
                mint_b_offset,
                mint_a,
                mint_b,
            );
        }

        let forward = self.get_pools_by_token_offsets(
            program_id,
            base_filters.clone(),
            mint_a_offset,
            mint_b_offset,
            mint_a,
            mint_b,
        );
        let reverse = self.get_pools_by_token_offsets(
            program_id,
            base_filters,
            mint_a_offset,
            mint_b_offset,
            mint_b,
            mint_a,
        );
        Box::pin(async move {
            let forward = forward.await?;
            let reverse = reverse.await?;
            Ok(merge_program_accounts(forward, reverse))
        })
    }

    fn get_pools_by_single_token_offsets(
        &self,
        program_id: Pubkey,
        base_filters: Vec<RpcFilterType>,
        mint_a_offset: usize,
        mint_b_offset: usize,
        mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        if mint_a_offset == mint_b_offset {
            return self.get_pools_by_single_token_offset(
                program_id,
                base_filters,
                mint_a_offset,
                mint,
            );
        }

        let first = self.get_pools_by_single_token_offset(
            program_id,
            base_filters.clone(),
            mint_a_offset,
            mint,
        );
        let second =
            self.get_pools_by_single_token_offset(program_id, base_filters, mint_b_offset, mint);
        Box::pin(async move {
            let first = first.await?;
            let second = second.await?;
            Ok(merge_program_accounts(first, second))
        })
    }

    fn get_all_orca_amm_pools_by_tokens(
        &self,
        token_mint_a: &Pubkey,
        token_mint_b: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            ORCA_PROGRAM_ID,
            vec![memcmp_b58(DISC_ORCA_B58)],
            ORCA_TOKEN_MINT_A_OFFSET,
            ORCA_TOKEN_MINT_B_OFFSET,
            token_mint_a,
            token_mint_b,
        )
    }

    fn get_all_orca_amm_pools_by_tokens_bidirectional(
        &self,
        token_mint_a: &Pubkey,
        token_mint_b: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            ORCA_PROGRAM_ID,
            vec![memcmp_b58(DISC_ORCA_B58)],
            ORCA_TOKEN_MINT_A_OFFSET,
            ORCA_TOKEN_MINT_B_OFFSET,
            token_mint_a,
            token_mint_b,
        )
    }

    fn get_all_orca_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            ORCA_PROGRAM_ID,
            vec![memcmp_b58(DISC_ORCA_B58)],
            ORCA_TOKEN_MINT_A_OFFSET,
            ORCA_TOKEN_MINT_B_OFFSET,
            token_mint,
        )
    }

    fn get_all_meteora_dlmm_amm_pools_by_tokens(
        &self,
        token_x_mint: &Pubkey,
        token_y_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            METEORA_DLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DLMM_B58)],
            METEORA_DLMM_TOKEN_X_MINT_OFFSET,
            METEORA_DLMM_TOKEN_Y_MINT_OFFSET,
            token_x_mint,
            token_y_mint,
        )
    }

    fn get_all_meteora_dlmm_amm_pools_by_tokens_bidirectional(
        &self,
        token_x_mint: &Pubkey,
        token_y_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            METEORA_DLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DLMM_B58)],
            METEORA_DLMM_TOKEN_X_MINT_OFFSET,
            METEORA_DLMM_TOKEN_Y_MINT_OFFSET,
            token_x_mint,
            token_y_mint,
        )
    }

    fn get_all_meteora_dlmm_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            METEORA_DLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DLMM_B58)],
            METEORA_DLMM_TOKEN_X_MINT_OFFSET,
            METEORA_DLMM_TOKEN_Y_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_meteora_damm_v2_amm_pools_by_tokens(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            METEORA_DAMM_V2_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DAMM_V2_B58)],
            METEORA_DAMM_V2_TOKEN_A_MINT_OFFSET,
            METEORA_DAMM_V2_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_meteora_damm_v2_amm_pools_by_tokens_bidirectional(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            METEORA_DAMM_V2_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DAMM_V2_B58)],
            METEORA_DAMM_V2_TOKEN_A_MINT_OFFSET,
            METEORA_DAMM_V2_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_meteora_damm_v2_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            METEORA_DAMM_V2_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DAMM_V2_B58)],
            METEORA_DAMM_V2_TOKEN_A_MINT_OFFSET,
            METEORA_DAMM_V2_TOKEN_B_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_meteora_damm_v1_amm_pools_by_tokens(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            METEORA_DAMM_V1_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_METEORA_DAMM_V1_BYTES)],
            METEORA_DAMM_V1_TOKEN_A_MINT_OFFSET,
            METEORA_DAMM_V1_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_meteora_damm_v1_amm_pools_by_tokens_bidirectional(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            METEORA_DAMM_V1_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_METEORA_DAMM_V1_BYTES)],
            METEORA_DAMM_V1_TOKEN_A_MINT_OFFSET,
            METEORA_DAMM_V1_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_meteora_damm_v1_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            METEORA_DAMM_V1_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_METEORA_DAMM_V1_BYTES)],
            METEORA_DAMM_V1_TOKEN_A_MINT_OFFSET,
            METEORA_DAMM_V1_TOKEN_B_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_raydium_clmm_amm_pools_by_tokens(
        &self,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            RAYDIUM_CLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint_0,
            token_mint_1,
        )
    }

    fn get_all_raydium_clmm_amm_pools_by_tokens_bidirectional(
        &self,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            RAYDIUM_CLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint_0,
            token_mint_1,
        )
    }

    fn get_all_raydium_clmm_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            RAYDIUM_CLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint,
        )
    }

    fn get_all_pancake_swap_amm_pools_by_tokens(
        &self,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            PANCAKE_SWAP_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint_0,
            token_mint_1,
        )
    }

    fn get_all_pancake_swap_amm_pools_by_tokens_bidirectional(
        &self,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            PANCAKE_SWAP_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint_0,
            token_mint_1,
        )
    }

    fn get_all_pancake_swap_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            PANCAKE_SWAP_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint,
        )
    }

    fn get_all_byreal_amm_pools_by_tokens(
        &self,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            BYREAL_CLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint_0,
            token_mint_1,
        )
    }

    fn get_all_byreal_amm_pools_by_tokens_bidirectional(
        &self,
        token_mint_0: &Pubkey,
        token_mint_1: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            BYREAL_CLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint_0,
            token_mint_1,
        )
    }

    fn get_all_byreal_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            BYREAL_CLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CLMM_TOKEN_MINT_0_OFFSET,
            RAYDIUM_CLMM_TOKEN_MINT_1_OFFSET,
            token_mint,
        )
    }

    fn get_all_pump_fun_amm_pools_by_tokens(
        &self,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            PUMPFUN_AMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DAMM_V2_B58)],
            PUMPFUN_AMM_BASE_MINT_OFFSET,
            PUMPFUN_AMM_QUOTE_MINT_OFFSET,
            base_mint,
            quote_mint,
        )
    }

    fn get_all_pump_fun_amm_pools_by_tokens_bidirectional(
        &self,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            PUMPFUN_AMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DAMM_V2_B58)],
            PUMPFUN_AMM_BASE_MINT_OFFSET,
            PUMPFUN_AMM_QUOTE_MINT_OFFSET,
            base_mint,
            quote_mint,
        )
    }

    fn get_all_pump_fun_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            PUMPFUN_AMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DAMM_V2_B58)],
            PUMPFUN_AMM_BASE_MINT_OFFSET,
            PUMPFUN_AMM_QUOTE_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_raydium_cpmm_amm_pools_by_tokens(
        &self,
        token_0_mint: &Pubkey,
        token_1_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            RAYDIUM_CPMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CPMM_TOKEN_0_MINT_OFFSET,
            RAYDIUM_CPMM_TOKEN_1_MINT_OFFSET,
            token_0_mint,
            token_1_mint,
        )
    }

    fn get_all_raydium_cpmm_amm_pools_by_tokens_bidirectional(
        &self,
        token_0_mint: &Pubkey,
        token_1_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            RAYDIUM_CPMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CPMM_TOKEN_0_MINT_OFFSET,
            RAYDIUM_CPMM_TOKEN_1_MINT_OFFSET,
            token_0_mint,
            token_1_mint,
        )
    }

    fn get_all_raydium_cpmm_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            RAYDIUM_CPMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
            RAYDIUM_CPMM_TOKEN_0_MINT_OFFSET,
            RAYDIUM_CPMM_TOKEN_1_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_raydium_amm_pools_by_tokens(
        &self,
        coin_vault_mint: &Pubkey,
        pc_vault_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            RAYDIUM_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_RAYDIUM_AMM_BYTES)],
            RAYDIUM_AMM_COIN_VAULT_MINT_OFFSET,
            RAYDIUM_AMM_PC_VAULT_MINT_OFFSET,
            coin_vault_mint,
            pc_vault_mint,
        )
    }

    fn get_all_raydium_amm_pools_by_tokens_bidirectional(
        &self,
        coin_vault_mint: &Pubkey,
        pc_vault_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            RAYDIUM_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_RAYDIUM_AMM_BYTES)],
            RAYDIUM_AMM_COIN_VAULT_MINT_OFFSET,
            RAYDIUM_AMM_PC_VAULT_MINT_OFFSET,
            coin_vault_mint,
            pc_vault_mint,
        )
    }

    fn get_all_raydium_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            RAYDIUM_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_RAYDIUM_AMM_BYTES)],
            RAYDIUM_AMM_COIN_VAULT_MINT_OFFSET,
            RAYDIUM_AMM_PC_VAULT_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_saros_amm_pools_by_tokens(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            SAROS_SWAP_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_saros_amm_pools_by_tokens_bidirectional(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            SAROS_SWAP_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_saros_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            SAROS_SWAP_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_orca_v2_amm_pools_by_tokens(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            ORCA_V2_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_orca_v2_amm_pools_by_tokens_bidirectional(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            ORCA_V2_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_orca_v2_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            ORCA_V2_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_orca_v1_amm_pools_by_tokens(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            ORCA_V1_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_orca_v1_amm_pools_by_tokens_bidirectional(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            ORCA_V1_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_orca_v1_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            ORCA_V1_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_stepn_amm_pools_by_tokens(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            STEPN_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_stepn_amm_pools_by_tokens_bidirectional(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            STEPN_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_stepn_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            STEPN_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_token_swap_amm_pools_by_tokens(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            TOKEN_SWAP_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_token_swap_amm_pools_by_tokens_bidirectional(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            TOKEN_SWAP_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_token_swap_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            TOKEN_SWAP_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_penguin_amm_pools_by_tokens(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            PENGUIN_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_penguin_amm_pools_by_tokens_bidirectional(
        &self,
        token_a_mint: &Pubkey,
        token_b_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            PENGUIN_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_a_mint,
            token_b_mint,
        )
    }

    fn get_all_penguin_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            PENGUIN_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
            TOKEN_SWAP_STYLE_TOKEN_A_MINT_OFFSET,
            TOKEN_SWAP_STYLE_TOKEN_B_MINT_OFFSET,
            token_mint,
        )
    }

    fn get_all_fusion_amm_pools_by_tokens(
        &self,
        token_mint_a: &Pubkey,
        token_mint_b: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            FUSION_PROGRAM_ID,
            vec![memcmp_b58(DISC_FUSION_B58)],
            FUSION_TOKEN_MINT_A_OFFSET,
            FUSION_TOKEN_MINT_B_OFFSET,
            token_mint_a,
            token_mint_b,
        )
    }

    fn get_all_fusion_amm_pools_by_tokens_bidirectional(
        &self,
        token_mint_a: &Pubkey,
        token_mint_b: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            FUSION_PROGRAM_ID,
            vec![memcmp_b58(DISC_FUSION_B58)],
            FUSION_TOKEN_MINT_A_OFFSET,
            FUSION_TOKEN_MINT_B_OFFSET,
            token_mint_a,
            token_mint_b,
        )
    }

    fn get_all_fusion_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            FUSION_PROGRAM_ID,
            vec![memcmp_b58(DISC_FUSION_B58)],
            FUSION_TOKEN_MINT_A_OFFSET,
            FUSION_TOKEN_MINT_B_OFFSET,
            token_mint,
        )
    }

    fn get_all_obric_v2_amm_pools_by_tokens(
        &self,
        mint_x: &Pubkey,
        mint_y: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets(
            OBRIC_V2_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_OBRIC_V2_BYTES)],
            OBRIC_V2_MINT_X_OFFSET,
            OBRIC_V2_MINT_Y_OFFSET,
            mint_x,
            mint_y,
        )
    }

    fn get_all_obric_v2_amm_pools_by_tokens_bidirectional(
        &self,
        mint_x: &Pubkey,
        mint_y: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_token_offsets_bidirectional(
            OBRIC_V2_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_OBRIC_V2_BYTES)],
            OBRIC_V2_MINT_X_OFFSET,
            OBRIC_V2_MINT_Y_OFFSET,
            mint_x,
            mint_y,
        )
    }

    fn get_all_obric_v2_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_pools_by_single_token_offsets(
            OBRIC_V2_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_OBRIC_V2_BYTES)],
            OBRIC_V2_MINT_X_OFFSET,
            OBRIC_V2_MINT_Y_OFFSET,
            token_mint,
        )
    }

    fn get_all_futarchy_amm_pools_by_tokens(
        &self,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        let spot_only = self.get_pools_by_token_offsets(
            FUTARCHY_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_FUTARCHY_BYTES)],
            FUTARCHY_SPOT_BASE_MINT_OFFSET,
            FUTARCHY_SPOT_QUOTE_MINT_OFFSET,
            base_mint,
            quote_mint,
        );
        let spot_only_reverse = self.get_pools_by_token_offsets(
            FUTARCHY_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_FUTARCHY_BYTES)],
            FUTARCHY_SPOT_BASE_MINT_OFFSET,
            FUTARCHY_SPOT_QUOTE_MINT_OFFSET,
            quote_mint,
            base_mint,
        );
        let futarchy_layout = self.get_pools_by_token_offsets(
            FUTARCHY_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_FUTARCHY_BYTES)],
            FUTARCHY_LAYOUT_BASE_MINT_OFFSET,
            FUTARCHY_LAYOUT_QUOTE_MINT_OFFSET,
            base_mint,
            quote_mint,
        );
        let futarchy_layout_reverse = self.get_pools_by_token_offsets(
            FUTARCHY_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_FUTARCHY_BYTES)],
            FUTARCHY_LAYOUT_BASE_MINT_OFFSET,
            FUTARCHY_LAYOUT_QUOTE_MINT_OFFSET,
            quote_mint,
            base_mint,
        );
        Box::pin(async move {
            let mut pools = spot_only.await?;
            pools.extend(spot_only_reverse.await?);
            pools.extend(futarchy_layout.await?);
            pools.extend(futarchy_layout_reverse.await?);
            Ok(pools)
        })
    }

    fn get_all_futarchy_amm_pools_by_token(
        &self,
        token_mint: &Pubkey,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        let spot_only = self.get_pools_by_single_token_offsets(
            FUTARCHY_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_FUTARCHY_BYTES)],
            FUTARCHY_SPOT_BASE_MINT_OFFSET,
            FUTARCHY_SPOT_QUOTE_MINT_OFFSET,
            token_mint,
        );
        let futarchy_layout = self.get_pools_by_single_token_offsets(
            FUTARCHY_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_FUTARCHY_BYTES)],
            FUTARCHY_LAYOUT_BASE_MINT_OFFSET,
            FUTARCHY_LAYOUT_QUOTE_MINT_OFFSET,
            token_mint,
        );
        Box::pin(async move {
            let mut pools = spot_only.await?;
            pools.extend(futarchy_layout.await?);
            Ok(pools)
        })
    }

    fn get_all_orca_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(ORCA_PROGRAM_ID, vec![memcmp_b58(DISC_ORCA_B58)])
    }

    fn get_all_meteora_dlmm_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            METEORA_DLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DLMM_B58)],
        )
    }

    fn get_all_meteora_damm_v2_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            METEORA_DAMM_V2_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DAMM_V2_B58)],
        )
    }

    fn get_all_meteora_damm_v1_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            METEORA_DAMM_V1_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_METEORA_DAMM_V1_BYTES)],
        )
    }

    fn get_all_raydium_clmm_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            RAYDIUM_CLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
        )
    }

    fn get_all_pancake_swap_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            PANCAKE_SWAP_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
        )
    }

    fn get_all_pump_fun_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            PUMPFUN_AMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_METEORA_DAMM_V2_B58)],
        )
    }

    fn get_all_raydium_cpmm_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            RAYDIUM_CPMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
        )
    }

    fn get_all_raydium_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            RAYDIUM_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_RAYDIUM_AMM_BYTES)],
        )
    }

    fn get_all_saros_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            SAROS_SWAP_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
        )
    }

    fn get_all_orca_v2_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            ORCA_V2_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
        )
    }

    fn get_all_orca_v1_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            ORCA_V1_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
        )
    }

    fn get_all_stepn_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            STEPN_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
        )
    }

    fn get_all_token_swap_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            TOKEN_SWAP_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
        )
    }

    fn get_all_byreal_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            BYREAL_CLMM_PROGRAM_ID,
            vec![memcmp_b58(DISC_RAYDIUM_LIKE_B58)],
        )
    }

    fn get_all_fusion_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(FUSION_PROGRAM_ID, vec![memcmp_b58(DISC_FUSION_B58)])
    }

    fn get_all_obric_v2_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            OBRIC_V2_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_OBRIC_V2_BYTES)],
        )
    }

    fn get_all_futarchy_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            FUTARCHY_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_FUTARCHY_BYTES)],
        )
    }

    fn get_all_penguin_amm_pools(&self) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        self.get_all_pools_with_filters(
            PENGUIN_AMM_PROGRAM_ID,
            vec![memcmp_bytes(&DISC_TOKEN_SWAP_STYLE_BYTES)],
        )
    }
}

use solana_client::nonblocking::rpc_client::RpcClient as AsyncRpcClient;
impl DexRpcClientAsyncExt for AsyncRpcClient {
    fn get_all_pools_with_filters(
        &self,
        program_id: Pubkey,
        filters: Vec<RpcFilterType>,
    ) -> BoxFuture<'_, ClientResult<ProgramAccounts>> {
        let config = pool_fetch_config(filters);
        Box::pin(async move {
            self.get_program_ui_accounts_with_config(&program_id, config)
                .await
        })
    }
}
