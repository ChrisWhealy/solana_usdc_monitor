use crate::{
    solana::{SignedUsdcTransaction, SignedUsdcTransactionsBySlot},
    transaction::process_transaction,
};

use log::{error, info};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::UiTransactionEncoding;
use tokio::time::Instant;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn process_slot_txns(rpc_client: &RpcClient, slot: u64) -> SignedUsdcTransactionsBySlot {
    let slot_start_time = Instant::now();
    let rpc_client_config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        max_supported_transaction_version: Some(0),
        ..Default::default()
    };
    let mut usdc_txns: Vec<SignedUsdcTransaction> = Vec::new();

    info!("---> Slot {}", slot);

    match rpc_client.get_block_with_config(slot, rpc_client_config) {
        Ok(confirmed_block) => {
            info!(
                "     get_block request took {:.3?}",
                slot_start_time.elapsed()
            );
            let mut txn_count = 0;

            // Process only confirmed transactions
            for txns in confirmed_block.transactions.iter() {
                // Process inner transactions
                for inner_txn in txns.iter() {
                    txn_count += 1;

                    // Exclude any transactions whose meta.err property is populated
                    if &inner_txn.meta.is_some() == &true {
                        if inner_txn.meta.clone().unwrap().err.is_some() {
                            continue;
                        }
                    }

                    usdc_txns.append(&mut process_transaction(inner_txn));
                }
            }

            info!(
                "<--- Slot {}: Processed {} transactions in {:.3?}",
                slot,
                txn_count,
                slot_start_time.elapsed()
            );
        }
        Err(e) => error!("<--- Slot {}: {}", slot, e),
    };

    SignedUsdcTransactionsBySlot {
        slot,
        txns: usdc_txns,
    }
}
