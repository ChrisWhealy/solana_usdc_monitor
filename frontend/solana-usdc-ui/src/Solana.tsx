interface SignedUsdcTransactionsBySlot {
    slot: number;
    txns: SignedUsdcTransaction[];
}

interface SignedUsdcTransaction {
    signatures: string[];
    txn: UsdcTransaction;
}

interface UsdcTransaction {
    from: string;
    to: string;
    amount: number;
}

export type {
  SignedUsdcTransactionsBySlot,
  SignedUsdcTransaction,
  UsdcTransaction
}
