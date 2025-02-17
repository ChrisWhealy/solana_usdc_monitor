use crate::instruction::*;
use serde_json::json;
use solana_transaction_status::parse_instruction::ParsedInstruction;

const AUTHORITY: &str = "Because I said so...";
const SOURCE: &str = "The sender";
const DESTINATION: &str = "The receiver";
const AMOUNT_STR: &str = "1470000";
const AMOUNT_F64: f64 = 1.470;
#[test]
fn should_parse_complete_instruction_info_amount_decimals() -> Result<(), String> {
    let parsed = json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "mint": USDC_MINT,
            "amount": "14700000",
            "decimals": 7,
        },
        "type": "transferChecked"
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(usdc_inst) => {
            if usdc_inst.amount == AMOUNT_F64 {
                Ok(())
            } else {
                Err(format!("Wrong amount in parsed instruction.  Expected {}, got {}", AMOUNT_F64, usdc_inst.amount))
            }
        }
        None => Err(String::from("Failed to parse instruction with info.amount")),
    }
}

#[test]
fn should_parse_complete_instruction_info_amount_no_decimals() -> Result<(), String> {
    let parsed = json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "mint": USDC_MINT,
            "amount": AMOUNT_STR,
        },
        "type": "transferChecked"
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(usdc_inst) => {
            if usdc_inst.amount == AMOUNT_F64 {
                Ok(())
            } else {
                Err(format!("Wrong amount in parsed instruction.  Expected {}, got {}", AMOUNT_F64, usdc_inst.amount))
            }
        }
        None => Err(String::from("Failed to parse instruction with info.amount")),
    }
}

#[test]
fn should_parse_complete_instruction_token_ui_amount() -> Result<(), String> {
    let parsed = json!({
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
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(_) => Ok(()),
        None => Err(String::from(
            "Failed to parse instruction with tokenAmount.uiAmount",
        )),
    }
}

#[test]
fn should_not_parse_wrong_program_id() -> Result<(), String> {
    let parsed = json!({
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
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: "some program_id".to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Wrong program_id",
        )),
        None => Ok(()),
    }
}

#[test]
fn should_not_parse_empty_info() -> Result<(), String> {
    let parsed = json!({
        "info": {},
        "type": "transferChecked"
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Info property is empty",
        )),
        None => Ok(()),
    }
}

#[test]
fn should_not_parse_missing_source() -> Result<(), String> {
    let parsed = json!({
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
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Source field is missing",
        )),
        None => Ok(()),
    }
}

#[test]
fn should_not_parse_missing_destination() -> Result<(), String> {
    let parsed = json!({
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
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Destination field is missing",
        )),
        None => Ok(()),
    }
}

#[test]
fn should_not_parse_missing_mint() -> Result<(), String> {
    let parsed = json!({
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
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Mint field is missing",
        )),
        None => Ok(()),
    }
}

#[test]
fn should_not_parse_missing_token_amount() -> Result<(), String> {
    let parsed = json!({
        "info": {
            "authority": AUTHORITY,
            "destination": DESTINATION,
            "source": SOURCE,
            "mint": USDC_MINT,
        },
        "type": "transferChecked"
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. tokenAmount property is missing",
        )),
        None => Ok(()),
    }
}

#[test]
fn should_not_parse_missing_ui_amount() -> Result<(), String> {
    let parsed = json!({
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
    });
    let inst: ParsedInstruction = ParsedInstruction {
        parsed,
        program: "some value".to_string(),
        program_id: TOKEN_PROGRAM_ID.to_string(),
        stack_height: None,
    };

    match process_instruction(&UiInstruction::Parsed(UiParsedInstruction::Parsed(inst))) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. tokenAmount.uiAmount field is missing",
        )),
        None => Ok(()),
    }
}
