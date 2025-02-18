use crate::{instruction::test_data::*, transaction::process_transaction};

use solana_sdk::transaction::TransactionError;
use solana_transaction_status::{
    option_serializer::OptionSerializer,
    parse_accounts::{ParsedAccount, ParsedAccountSource},
    EncodedTransaction, EncodedTransactionWithStatusMeta, UiInnerInstructions, UiMessage,
    UiParsedMessage, UiTransaction, UiTransactionStatusMeta,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn get_signatures() -> Vec<String> {
    vec![
        "32bsAScN7dH7DW3tjP3AnD1RBn8UsDsE4BwVFfGLUKRda4ConrSwbGhAdpUU5p1Jw6k1jvAZD1bHhHdJ6Xvuphjh"
            .to_string(),
        "4XD9ZNwzrcnrGrAsQZaw1TKg521aGbggvmiYhjHokQjKQo2SqbQXNsuNjQEnPMeiU5uEtKPtqJfTVsEpUHc4cNzb"
            .to_string(),
    ]
}

fn get_account_keys() -> Vec<ParsedAccount> {
    vec![
        ParsedAccount {
            pubkey: "E1wid5KyTfkzxWDUmhxhJTKZqVRR4N6kyuTLXB5bPiU".to_string(),
            signer: true,
            source: Some(ParsedAccountSource::Transaction),
            writable: true,
        },
        ParsedAccount {
            pubkey: "4yV1DkPtGameUwtM3CzwM23inphbosNbmX4d3MRLaPBc".to_string(),
            signer: false,
            source: Some(ParsedAccountSource::Transaction),
            writable: true,
        },
        ParsedAccount {
            pubkey: "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo".to_string(),
            signer: false,
            source: Some(ParsedAccountSource::Transaction),
            writable: true,
        },
    ]
}

fn get_ui_txn_status_meta(with_error: bool) -> UiTransactionStatusMeta {
    UiTransactionStatusMeta {
        err: if with_error {
            Some(TransactionError::MissingSignatureForFee)
        } else {
            None
        },
        status: Ok(()),
        fee: 12345,
        pre_balances: vec![],
        post_balances: vec![],
        inner_instructions: OptionSerializer::from(Some(vec![UiInnerInstructions {
            index: 0,
            instructions: vec![],
        }])),
        log_messages: OptionSerializer::None,
        pre_token_balances: OptionSerializer::None,
        post_token_balances: OptionSerializer::None,
        rewards: OptionSerializer::None,
        loaded_addresses: OptionSerializer::None,
        return_data: OptionSerializer::None,
        compute_units_consumed: OptionSerializer::None,
    }
}

pub fn get_ui_transaction(with_signatures: bool) -> UiTransaction {
    UiTransaction {
        signatures: if with_signatures {
            get_signatures()
        } else {
            vec![]
        },
        message: UiMessage::Parsed(UiParsedMessage {
            account_keys: get_account_keys(),
            recent_blockhash: "".to_string(),
            instructions: vec![get_parsed_ui_instruction_for_test(1, false)],
            address_table_lookups: None,
        }),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_01_should_parse_encoded_txn_without_status_meta() -> Result<(), String> {
    let usdc_txns = process_transaction(&EncodedTransactionWithStatusMeta {
        transaction: EncodedTransaction::Json(get_ui_transaction(true)),
        meta: None,
        version: None,
    });

    if usdc_txns.len() > 0 {
        Ok(())
    } else {
        Err("Failed to parse encoded transaction".to_string())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_02_should_parse_encoded_txn_with_non_error_status_meta() -> Result<(), String> {
    let usdc_txns = process_transaction(&EncodedTransactionWithStatusMeta {
        transaction: EncodedTransaction::Json(get_ui_transaction(true)),
        meta: Some(get_ui_txn_status_meta(false)),
        version: None,
    });

    if usdc_txns.len() > 0 {
        Ok(())
    } else {
        Err("Failed to parse encoded transaction".to_string())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_03_should_not_parse_unsigned_encoded_txn() -> Result<(), String> {
    let usdc_txns = process_transaction(&EncodedTransactionWithStatusMeta {
        transaction: EncodedTransaction::Json(get_ui_transaction(false)),
        meta: Some(get_ui_txn_status_meta(false)),
        version: None,
    });

    if usdc_txns.len() > 0 {
        Err("Should not have parsed unsigned encoded transaction".to_string())
    } else {
        Ok(())
    }
}
