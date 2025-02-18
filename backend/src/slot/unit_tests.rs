use crate::{process_slot_txns, transaction::unit_tests::get_ui_transaction};

use async_trait::async_trait;
use env_logger;
use serde_json::{json, Value};
use solana_client::{
    client_error,
    rpc_client::{RpcClient, RpcClientConfig},
    rpc_request::RpcRequest,
    rpc_sender::{RpcSender, RpcTransportStats},
};
use solana_commitment_config::CommitmentConfig;
use std::collections::HashMap;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn get_mock_response() -> Value {
    json!({
        "blockHeight": 298412657,
        "blockTime": 1739353792,
        "blockhash": "4wf7zh1rycq61WQ6bgrtmwctNKVjiw2yMKXHmSWXE4cT",
        "parentSlot": 320137774,
        "previousBlockhash": "2suEkuPqvWnEaQFNsuhKy7cmafraseALmhjo1k6jRu61",
        "rewards": [{
            "commission": null,
            "lamports": 26251371,
            "postBalance": 11190784600i64,
            "pubkey": "A4hyMd3FyvUJSRafDUSwtLLaQcxRP4r1BRC9w2AJ1to2",
            "reward_type": "Fee"
        }],
        "transactions": [{
            "transaction": json!(get_ui_transaction(true)),
            "meta": null,
            "version": null
        }],
        "numPartitions": null
    })
}

struct MockRpcSender {
    responses: HashMap<RpcRequest, Value>,
}

#[async_trait]
impl RpcSender for MockRpcSender {
    async fn send(&self, request: RpcRequest, _params: Value) -> client_error::Result<Value> {
        if let Some(response) = self.responses.get(&request) {
            Ok(response.clone())
        } else {
            Err(client_error::ClientError::from(
                client_error::ClientErrorKind::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("MockRpcSender has not been configured to respond to request: {:#?}", request),
                )),
            ))
        }
    }

    fn get_transport_stats(&self) -> RpcTransportStats {
        RpcTransportStats::default()
    }

    fn url(&self) -> String {
        "mock_url".to_string()
    }
}

fn create_mock_client(responses: HashMap<RpcRequest, Value>) -> RpcClient {
    RpcClient::new_sender(
        MockRpcSender { responses },
        RpcClientConfig {
            commitment_config: CommitmentConfig::finalized(),
            confirm_transaction_initial_timeout: None,
        },
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_01_should_process_slot_with_valid_txn() -> Result<(), String> {
    env_logger::builder()
        .is_test(true)
        .format_timestamp_millis()
        .init();

    let test_slot = 12345;
    let mut responses = HashMap::new();
    responses.insert(RpcRequest::GetBlock, get_mock_response());

    let mock_client = create_mock_client(responses.clone());
    let result = process_slot_txns(&mock_client, test_slot);

    if result.txns.len() > 0 {
        Ok(())
    } else {
        Err("Failed to process valid transaction".to_string())
    }
}
