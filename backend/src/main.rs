mod instruction;
mod slot;
mod solana;
mod transactions;

use crate::{slot::process_slot_txns, solana::SignedUsdcTransactionsBySlot};

use axum::{routing::get, Json, Router};
use env_logger;
use log::{error, info};
use solana_client::rpc_client::RpcClient;
use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time::sleep};
use tower_http::cors::{Any, CorsLayer};
use crate::solana::SignedUsdcTransaction;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const SOLANA_RPC_URL: &str = "https://api.mainnet-beta.solana.com";
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const LOCAL_ADDR: &str = "127.0.0.1:3000";
const SLEEP_TIME: Duration = Duration::from_secs(1);

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[tokio::main]
async fn main() {
    env_logger::init();

    let txns: Arc<Mutex<Vec<SignedUsdcTransactionsBySlot>>> = Arc::new(Mutex::new(Vec::new()));
    let txns_clone = Arc::clone(&txns);

    task::spawn(async move {
        monitor_solana_txns(txns_clone).await;
    });

    let permissive_cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/transactions", get(get_transactions))
        .layer(permissive_cors)
        .with_state(txns);

    let addr = SocketAddr::from_str(LOCAL_ADDR).unwrap();
    info!("Server running on http://{}", LOCAL_ADDR);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
async fn monitor_solana_txns(transactions: Arc<Mutex<Vec<SignedUsdcTransactionsBySlot>>>) {
    let mut start_slot: u64 = 0;
    let rpc_client = RpcClient::new(SOLANA_RPC_URL.to_string());

    loop {
        // Assume that no slots are available
        let mut slots: Vec<u64> = vec![];

        // Fetch latest slot, or slot range from the last one processed up till now
        if start_slot == 0 {
            match rpc_client.get_slot() {
                Ok(slot) => slots = vec![slot],
                Err(e) => error!("{}", e),
            }
        } else {
            match rpc_client.get_blocks(start_slot, None) {
                Ok(s) => slots = s,
                Err(e) => error!("{}", e),
            }
        };

        // Process all transactions per slot
        for slot in slots.iter() {
            let mut signed_usdc_txns: Vec<SignedUsdcTransaction> = Vec::new();
            for txn in process_slot_txns(&rpc_client, *slot).txns {
                signed_usdc_txns.push(txn);
            }

            if signed_usdc_txns.len() > 0 {
                transactions.lock().await.push(SignedUsdcTransactionsBySlot {
                    slot: *slot,
                    txns: signed_usdc_txns
                });
            }
        }

        // Bump start slot for next iteration
        start_slot = if let Some(last_slot) = slots.last() {
            *last_slot + 1
        } else {
            0
        };

        info!("Sleeping for {} second(s)", SLEEP_TIME.as_secs());
        sleep(SLEEP_TIME).await;
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
async fn get_transactions(
    state: axum::extract::State<Arc<Mutex<Vec<SignedUsdcTransactionsBySlot>>>>,
) -> Json<Vec<SignedUsdcTransactionsBySlot>> {
    Json(state.lock().await.clone())
}
