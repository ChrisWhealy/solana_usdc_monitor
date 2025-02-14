import { useEffect, useState, Fragment } from "react";

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

const API_URL = "http://localhost:3000/transactions";
const REFRESH_INTERVAL = 2000;

function App() {
    const [transactions, setTransactions] = useState<SignedUsdcTransactionsBySlot[]>([]);
    const [filteredTransactions, setFilteredTransactions] = useState<SignedUsdcTransactionsBySlot[]>([]);
    const [signatureFilter, setSignatureFilter] = useState<string>("");
    const [showSignatures, setShowSignatures] = useState<boolean>(true);

    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    const fetchTransactions = async () => {
        try {
            setLoading(true);
            const response = await fetch(API_URL);

            if (!response.ok) throw new Error("Fetch failed");

            const data: SignedUsdcTransactionsBySlot[] = await response.json();

            setTransactions(data);
            setFilteredTransactions(data);
            setError(null);
        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    // Repeatedly poll for updates
    useEffect(() => {
      fetchTransactions();
      const interval = setInterval(fetchTransactions, REFRESH_INTERVAL);
      return () => clearInterval(interval);
    }, []);

    // Filter Transactions by Signature
    useEffect(() => {
      if (!signatureFilter.trim()) {
          setFilteredTransactions(transactions);
          return;
      }

      const filtered = transactions
          .map(slotGroup => ({
              ...slotGroup,
              txns: slotGroup.txns.filter(tx => tx.signatures.some(sig => sig.startsWith(signatureFilter))),
          }))
          .filter(slotGroup => slotGroup.txns.length > 0);

      setFilteredTransactions(filtered);
    }, [signatureFilter, transactions]);

  return (
      <div style={{ padding: "20px", fontFamily: "Arial, sans-serif" }}>
          {loading && <p>Loading transactions...</p>}
          {error && <p style={{ color: "red" }}>Error: {error}</p>}
          <>
            <div style={{ marginBottom: "20px", display: "flex", gap: "10px", alignItems: "center" }}>
              <input
                type="text"
                className="search-term"
                placeholder="Filter by start of transaction hash..."
                value={signatureFilter}
                onChange={e => setSignatureFilter(e.target.value)}
              />
              <label className="sig-label">
                Show Signatures
                <input
                  type="checkbox"
                  checked={showSignatures}
                  onChange={e => setShowSignatures(e.target.checked)}
                />
              </label>
            </div>
          </>
          <>
            <table border={0} cellPadding={3} cellSpacing={0} className="txn-table">
              <thead>
                <tr><th colSpan={6}>Solana USDC Transactions</th></tr>
              </thead>
              <tbody>
              {filteredTransactions.length === 0
                ? (<tr><td>No transactions found</td></tr>)
                : (filteredTransactions.map((txnsBySlot, index) => (
                    <Fragment key={index}>
                      <tr>
                        <td
                          className="slot-hdr"
                          colSpan={6}
                          style={{ fontWeight: "700"}}>
                            Latest block: {txnsBySlot.slot}
                        </td>
                      </tr>
                      {txnsBySlot.txns.map((tx, txnIndex) => (
                        <Fragment key={`${index}-${txnIndex}`}>
                          <tr>
                            <td>TX detected:</td>
                            <td>{tx.txn.from}</td>
                            <td>sent</td>
                            <td style={{textAlign: "right"}}>{tx.txn.amount}</td>
                            <td>USDC to</td>
                            <td>{tx.txn.to}</td>
                          </tr>
                          {showSignatures && (
                            <tr>
                              <td>Signatures</td>
                              <td colSpan={5}>{tx.signatures.map(sig => <p key={sig}>{sig}</p>)}</td>
                            </tr>
                          )}
                        </Fragment>
                      ))}
                    </Fragment>
                  ))
                )}
              </tbody>
            </table>
          </>
      </div>
    );
}

export default App;
