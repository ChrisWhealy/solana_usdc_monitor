use crate::{instruction::process_instruction, solana::SignedUsdcTransaction};

use log::info;
use solana_transaction_status::{
    EncodedTransaction, EncodedTransactionWithStatusMeta, UiMessage, UiParsedMessage, UiTransaction,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn process_transaction(txn: &EncodedTransactionWithStatusMeta) -> Vec<SignedUsdcTransaction> {
    let mut signed_txns: Vec<SignedUsdcTransaction> = Vec::new();

    match &txn.transaction {
        // If present, extract instructions from signed, parsed JSON transaction
        EncodedTransaction::Json(
            UiTransaction {
                message: UiMessage::Parsed(UiParsedMessage { instructions, .. }),
                signatures,
                ..
            },
            ..,
        ) => instructions.iter().for_each(|i| {
            if signatures.is_empty() {
                // info!("Skipping unsigned transaction");
            } else if let Some(txn) = process_instruction(i) {
                info!("{}", txn);
                signed_txns.push(SignedUsdcTransaction {
                    signatures: signatures.clone(),
                    txn,
                });
            }
        }),
        _ => info!("Transaction does not contain a JSON message"),
    }

    signed_txns
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
pub mod unit_tests;
