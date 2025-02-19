mod instruction;
mod slot;
mod solana;
mod transaction;

use crate::{
    slot::process_slot_txns, solana::SignedUsdcTransaction, solana::SignedUsdcTransactionsBySlot,
};

use axum::{routing::get, Json, Router};
use env_logger;
use log::{error, info};
use solana_client::rpc_client::RpcClient;
use std::{env, net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time::sleep};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
const DEFAULT_SOLANA_RPC_URL: &str = "https://api.mainnet-beta.solana.com";
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const LOCAL_ADDR: &str = "0.0.0.0:3000";
const SLEEP_TIME_MILLIS: Duration = Duration::from_millis(1000);

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
        .route("/api/health", get(|| async { "API is running!" }))
        .route("/transactions", get(get_transactions))
        .fallback_service(ServeDir::new("./frontend/dist"))
        .layer(permissive_cors)
        .with_state(txns);
    let listener = tokio::net::TcpListener::bind(SocketAddr::from_str(LOCAL_ADDR).unwrap())
        .await
        .unwrap();

    info!("Server running on http://{}", LOCAL_ADDR);
    axum::serve(listener, app).await.unwrap();
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
async fn monitor_solana_txns(transactions: Arc<Mutex<Vec<SignedUsdcTransactionsBySlot>>>) {
    let mut next_slot: u64 = 0;
    let solana_rpc_url = env::var("SOLANA_RPC_URL").unwrap_or_else(|_| DEFAULT_SOLANA_RPC_URL.to_string());
    let rpc_client = RpcClient::new(&solana_rpc_url);

    info!("Monitoring: {}", solana_rpc_url);

    loop {
        // Fetch latest slot, or slot range
        let slots: Vec<u64> = if next_slot == 0 {
            match rpc_client.get_slot() {
                Ok(slot) => vec![slot],
                Err(e) => {
                    error!("{}", e);
                    vec![]
                }
            }
        } else {
            match rpc_client.get_blocks(next_slot, None) {
                Ok(s) => s,
                Err(e) => {
                    error!("{}", e);
                    vec![]
                }
            }
        };

        // Process all transactions per slot
        for slot in slots.iter() {
            let signed_usdc_txns: Vec<SignedUsdcTransaction> =
                process_slot_txns(&rpc_client, *slot)
                    .txns
                    .into_iter()
                    .collect();

            if !signed_usdc_txns.is_empty() {
                transactions
                    .lock()
                    .await
                    .push(SignedUsdcTransactionsBySlot {
                        slot: *slot,
                        txns: signed_usdc_txns,
                    });
            }
        }

        // Bump slot number for next iteration
        next_slot = if let Some(last_slot) = slots.last() {
            *last_slot + 1
        } else {
            0
        };

        info!("Sleeping for {:?}", SLEEP_TIME_MILLIS);
        sleep(SLEEP_TIME_MILLIS).await;
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
async fn get_transactions(
    state: axum::extract::State<Arc<Mutex<Vec<SignedUsdcTransactionsBySlot>>>>,
) -> Json<Vec<SignedUsdcTransactionsBySlot>> {
    Json(state.lock().await.clone())
}
