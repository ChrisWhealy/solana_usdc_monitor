use crate::{solana::UsdcTransaction, TOKEN_PROGRAM_ID, USDC_MINT};
use serde_json::{Map, Value};
use solana_transaction_status::{
    parse_instruction::ParsedInstruction, UiInstruction, UiParsedInstruction,
};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
fn get_prop_as_str<'a>(obj: &'a Map<String, Value>, property: &'a str) -> Option<&'a str> {
    obj.get(property).and_then(|v| v.as_str())
}
fn get_prop_as_f64(obj: &Map<String, Value>, property: &str) -> Option<f64> {
    obj.get(property).and_then(|v| v.as_f64())
}
fn get_prop_as_str_then_f64(obj: &Map<String, Value>, property: &str) -> Option<f64> {
    get_prop_as_str(obj, property).map(|s| s.parse::<f64>().unwrap())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// This function will bail out at the earliest opportunity
pub fn process_instruction(instruction: &UiInstruction) -> Option<UsdcTransaction> {
    let no_mint = "mint missing";

    match instruction {
        UiInstruction::Parsed(UiParsedInstruction::Parsed(ParsedInstruction {
            program_id,
            parsed,
            ..
        })) => {
            if !program_id.eq(TOKEN_PROGRAM_ID) {
                return None;
            }

            if let Some(info) = parsed
                .as_object()
                .and_then(|parsed_obj| parsed_obj.get("info"))
                .and_then(|info| info.as_object())
            {
                if !get_prop_as_str(info, "mint")
                    .unwrap_or(&no_mint)
                    .eq(USDC_MINT)
                {
                    return None;
                }

                let source = if let Some(src) = get_prop_as_str(info, "source") {
                    src
                } else {
                    return None;
                };
                let destination = if let Some(dest) = get_prop_as_str(info, "destination") {
                    dest
                } else {
                    return None;
                };

                // If info.amount is missing, then look for tokenAmount.uiAmount
                let amount = if let Some(amt) = get_prop_as_str_then_f64(info,"amount") {
                    let exp = if let Some(decimals) = get_prop_as_f64(info,"decimals") {
                        decimals
                    } else {
                        // If decimal places are not specified, then assume 6
                        6_f64
                    };
                    
                    amt / 10_f64.powf(exp)
                } else {
                    if let Some(mut token_amount) = info
                        .get("tokenAmount")
                        .and_then(|tkn_amt| tkn_amt.as_object())
                    {
                        if let Some(ui_amount) = get_prop_as_f64(&mut token_amount,"uiAmount") {
                            ui_amount
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                };

                return Some(UsdcTransaction {
                    from: source.to_string(),
                    to: destination.to_string(),
                    amount,
                });
            }
        }
        _ => {}
    }

    None
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[cfg(test)]
mod unit_tests;