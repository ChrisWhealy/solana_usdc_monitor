use crate::{instruction::process_instruction, solana::SignedUsdcTransaction};

use log::info;
use solana_transaction_status::{
    EncodedTransaction, EncodedTransactionWithStatusMeta, UiMessage, UiParsedMessage, UiTransaction,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub fn process_transaction(
    slot: u64,
    txn: &EncodedTransactionWithStatusMeta,
) -> Vec<SignedUsdcTransaction> {
    let mut signed_txns: Vec<SignedUsdcTransaction> = Vec::new();

    match &txn.transaction {
        // If present, extract instructions from parsed JSON transaction
        EncodedTransaction::Json(
            UiTransaction {
                message: UiMessage::Parsed(UiParsedMessage { instructions, .. }),
                signatures,
                ..
            },
            ..,
        ) => instructions.iter().for_each(|i| {
            if let Some(txn) = process_instruction(i) {
                info!("{}", txn);
                signed_txns.push(SignedUsdcTransaction {
                    slot,
                    signatures: signatures.clone(),
                    txn,
                });
            }
        }),
        _ => info!("     Parsed JSON message not found"),
    }

    signed_txns
}
