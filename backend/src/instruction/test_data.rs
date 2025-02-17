use crate::*;
use serde_json::{json, Value};
use solana_transaction_status::{parse_instruction::ParsedInstruction, UiInstruction, UiParsedInstruction};
use std::collections::HashMap;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
pub const AUTHORITY: &str = "Because I said so...";
pub const SOURCE: &str = "The sender";
pub const DESTINATION: &str = "The receiver";
pub const AMOUNT_STR: &str = "1470000";
pub const AMOUNT_F64: f64 = 1.470;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// Info contains amount and decimals
fn test_data_amount_and_decimals() -> Value {
    json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "mint": USDC_MINT,
            "amount": "14700000",
            "decimals": 7,
        },
        "type": "transferChecked"
    })
}

// Info contains only amount
fn test_data_amount() -> Value {
    json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "mint": USDC_MINT,
            "amount": AMOUNT_STR,
        },
        "type": "transferChecked"
    })
}

// Complete instruction
fn test_data_complete_instruction() -> Value {
    json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "mint": USDC_MINT,
            "tokenAmount": {
                "amount": "1470000000",
                "decimals": 9,
                "uiAmount": AMOUNT_F64,
                "uiAmountString": "1.47"
            }
        },
        "type": "transferChecked"
    })
}

// Empty info 
fn test_data_empty_info() -> Value {
    json!({
        "info": {},
        "type": "transferChecked"
    })
}

// Missing source 
fn test_data_missing_source() -> Value {
    json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "mint": USDC_MINT,
            "tokenAmount": {
                "amount": "1470000000",
                "decimals": 9,
                "uiAmount": AMOUNT_F64,
                "uiAmountString": "1.47"
            }
        },
        "type": "transferChecked"
    })
}

// Missing destination
fn test_data_missing_destination() -> Value {
    json!({
        "info": {
            "authority": AUTHORITY,
            "source": SOURCE,
            "mint": USDC_MINT,
            "tokenAmount": {
                "amount": "1470000000",
                "decimals": 9,
                "uiAmount": AMOUNT_F64,
                "uiAmountString": "1.47"
            }
        },
        "type": "transferChecked"
    })
}

// Missing mint
fn test_data_missing_mint() -> Value {
    json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "tokenAmount": {
                "amount": "1470000000",
                "decimals": 9,
                "uiAmount": AMOUNT_F64,
                "uiAmountString": "1.47"
            }
        },
        "type": "transferChecked"
    })
}

// Missing tokenAmount property
fn test_data_missing_token_amount() -> Value {
    json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "mint": USDC_MINT,
        },
        "type": "transferChecked"
    })
}

// Missing tokenAmount.uiAmount property
fn test_data_missing_token_amount_ui_amount() -> Value {
    json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "mint": USDC_MINT,
            "tokenAmount": {
                "amount": "1470000000",
                "decimals": 9,
                "uiAmountString": "1.47"
            }
        },
        "type": "transferChecked"
    })
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn get_test_map() -> HashMap<u8, Value> {
    let mut test_map: HashMap<u8, Value> = HashMap::new();

    test_map.insert(1, test_data_amount_and_decimals());
    test_map.insert(2, test_data_amount());
    test_map.insert(3, test_data_complete_instruction());
    test_map.insert(4, test_data_complete_instruction());
    test_map.insert(5, test_data_empty_info());
    test_map.insert(6, test_data_missing_source());
    test_map.insert(7, test_data_missing_destination());
    test_map.insert(8, test_data_missing_mint());
    test_map.insert(9, test_data_missing_token_amount());
    test_map.insert(10, test_data_missing_token_amount_ui_amount());

    test_map
}

fn get_parsed_value_for_test(test_num: u8) -> Value {
    let tests = get_test_map();
    
    if let Some(v) = tests.get(&test_num) {
        v.clone()
    } else {
        panic!("Invalid test number {}.  Expected a number between 1 and {}", test_num, tests.len());
    }
}

pub fn get_parsed_ui_instruction_for_test(test_num: u8, bad_program_id: bool) -> UiInstruction {
    UiInstruction::Parsed(UiParsedInstruction::Parsed(ParsedInstruction {
        parsed: get_parsed_value_for_test(test_num),
        program: "some value".to_string(),
        program_id: (if bad_program_id { "bad program id" } else { TOKEN_PROGRAM_ID}).to_string(),
        stack_height: None,
    }))
}
