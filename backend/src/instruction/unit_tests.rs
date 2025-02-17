use crate::instruction::{process_instruction, test_data::*};

fn test_for_false_positive(test_num: u8, prop_name: &str) -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(test_num, false)) {
        Some(_) => Err(format!(
            "Should not have parsed instruction. Property '{}' is missing",
            prop_name
        )),
        None => Ok(()),
    }
}

fn false_negative_msg(prop_name: &str) -> String {
    format!(
        "Failed to parse instruction containing a valid value for '{}'",
        prop_name
    )
}

fn test_for_false_negative(test_num: u8, prop_name: &str) -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(test_num, false)) {
        Some(_) => Ok(()),
        None => Err(false_negative_msg(prop_name)),
    }
}

fn check_parsed_amount(got: f64, expected: f64) -> Result<(), String> {
    if got == expected {
        Ok(())
    } else {
        Err(format!(
            "Wrong amount in parsed instruction.  Expected {}, got {}",
            expected, got
        ))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_01_should_parse_complete_instruction_info_amount_decimals() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(1, false)) {
        Some(usdc_inst) => check_parsed_amount(usdc_inst.amount, AMOUNT_F64),
        None => Err(false_negative_msg("info.amount")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_02_should_parse_complete_instruction_info_amount_no_decimals() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(2, false)) {
        Some(usdc_inst) => check_parsed_amount(usdc_inst.amount, AMOUNT_F64),
        None => Err(false_negative_msg("info.amount")),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_03_should_parse_complete_instruction_token_ui_amount() -> Result<(), String> {
    test_for_false_negative(3, "tokenAmount.uiAmount")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_04_should_not_parse_wrong_program_id() -> Result<(), String> {
    match process_instruction(&get_parsed_ui_instruction_for_test(4, true)) {
        Some(_) => Err("Should not have parsed instruction. Wrong program_id".to_string()),
        None => Ok(()),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_05_should_not_parse_empty_info() -> Result<(), String> {
    test_for_false_positive(5, "info")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_06_should_not_parse_missing_source() -> Result<(), String> {
    test_for_false_positive(6, "source")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_07_should_not_parse_missing_destination() -> Result<(), String> {
    test_for_false_positive(7, "destination")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_08_should_not_parse_missing_mint() -> Result<(), String> {
    test_for_false_positive(8, "mint")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_09_should_not_parse_missing_token_amount() -> Result<(), String> {
    test_for_false_positive(9, "tokenAmount")
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[test]
fn test_10_should_not_parse_missing_ui_amount() -> Result<(), String> {
    test_for_false_positive(10, "tokenAmount.uiAmount")
}
