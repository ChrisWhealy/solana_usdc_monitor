use serde::{Deserialize, Serialize};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedUsdcTransactionsBySlot {
    pub slot: u64,
    pub txns: Vec<SignedUsdcTransaction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedUsdcTransaction {
    pub signatures: Vec<String>,
    pub txn: UsdcTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsdcTransaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl std::fmt::Display for UsdcTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "TX detected: {} sent {} USDC to {}",
            self.from, self.amount, self.to
        )
    }
}
