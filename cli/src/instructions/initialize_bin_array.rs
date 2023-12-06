use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::{solana_sdk::pubkey::Pubkey, solana_sdk::signer::Signer, Program};
use anyhow::*;
use dlmm_program_interface::accounts;
use dlmm_program_interface::instruction;
use dlmm_program_interface::utils::pda::*;
use std::ops::Deref;

#[derive(Debug)]
pub struct InitBinArrayParameters {
    pub lb_pair: Pubkey,
    pub bin_array_index: i64,
}

pub fn initialize_bin_array<C: Deref<Target = impl Signer> + Clone>(
    params: InitBinArrayParameters,
    program: &Program<C>,
    transaction_config: RpcSendTransactionConfig,
) -> Result<Pubkey> {
    let InitBinArrayParameters {
        lb_pair,
        bin_array_index,
    } = params;

    let (bin_array, _bump) = derive_bin_array_pda(lb_pair, bin_array_index);

    let accounts = accounts::InitializeBinArray {
        bin_array,
        funder: program.payer(),
        lb_pair,
        system_program: anchor_client::solana_sdk::system_program::ID,
    };

    let ix = instruction::InitializeBinArray {
        index: bin_array_index,
    };

    let request_builder = program.request();
    let signature = request_builder
        .accounts(accounts)
        .args(ix)
        .send_with_spinner_and_config(transaction_config);

    println!("Initialize Bin Array {bin_array}. Signature: {signature:#?}");

    signature?;

    Ok(bin_array)
}
