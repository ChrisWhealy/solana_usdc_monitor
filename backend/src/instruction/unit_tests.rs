use crate::instruction::{process_instruction, test_data::*};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_01_should_parse_complete_instruction_info_amount_decimals() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(1, false)) {
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_02_should_parse_complete_instruction_info_amount_no_decimals() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(2, false)) {
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_03_should_parse_complete_instruction_token_ui_amount() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(3, false)) {
        Some(_) => Ok(()),
        None => Err(String::from(
            "Failed to parse instruction with tokenAmount.uiAmount",
        )),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_04_should_not_parse_wrong_program_id() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(4, true)) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Wrong program_id",
        )),
        None => Ok(()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_05_should_not_parse_empty_info() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(5, false)) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Info property is empty",
        )),
        None => Ok(()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_06_should_not_parse_missing_source() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(6, false)) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Source field is missing",
        )),
        None => Ok(()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_07_should_not_parse_missing_destination() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(7, false)) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Destination field is missing",
        )),
        None => Ok(()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_08_should_not_parse_missing_mint() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(8, false)) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. Mint field is missing",
        )),
        None => Ok(()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_09_should_not_parse_missing_token_amount() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(9, false)) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. tokenAmount property is missing",
        )),
        None => Ok(()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_10_should_not_parse_missing_ui_amount() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(10, false)) {
        Some(_) => Err(String::from(
            "Should not have parsed instruction. tokenAmount.uiAmount field is missing",
        )),
        None => Ok(()),
    }
}
