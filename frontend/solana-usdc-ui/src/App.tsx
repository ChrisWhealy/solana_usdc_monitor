import { useEffect, useState, Fragment } from "react";

interface SignedUsdcTransactionBySlot {
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
    const [transactions, setTransactions] = useState<SignedUsdcTransactionBySlot[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetchTransactions();
        const interval = setInterval(fetchTransactions, REFRESH_INTERVAL);
        return () => clearInterval(interval);
    }, []);

    const fetchTransactions = async () => {
        try {
            setLoading(true);
            const response = await fetch(API_URL);

            if (!response.ok) {
                throw new Error(`Fetch failed: ${response.text}`);
            }

            const data: SignedUsdcTransactionBySlot[] = await response.json();

            setTransactions(data);
            setError(null);
        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    return (
      <div style={{ padding: "20px", fontFamily: "Arial, sans-serif" }}>
          <h1>Solana USDC Transactions</h1>
          {loading && <p>Loading transactions...</p>}
          {error && <p style={{ color: "red" }}>Error: {error}</p>}
          {transactions.length === 0 ? (
              <p>No transactions found</p>
          ) : (
              <table border={0} cellPadding={3} cellSpacing={0} style={{ width: "100%", marginTop: "10px" }}>
                  {transactions.map((txnsBySlot, index) => (
                      <Fragment key={index}>
                          <tr><td colSpan={6} style={{ fontWeight: "700"}}>Latest block: {txnsBySlot.slot}</td></tr>
                          {txnsBySlot.txns.map((tx, txnIndex) => (
                              <tr key={`${index}-${txnIndex}`}>
                                  <td>TX detected:</td>
                                  <td>{tx.txn.from}</td>
                                  <td>sent</td>
                                  <td style={{textAlign: "right"}}>{tx.txn.amount}</td>
                                  <td>USDC to</td>
                                  <td>{tx.txn.to}</td>
                              </tr>
                          ))}
                      </Fragment>
                  ))}
              </table>
          )}
      </div>
  );
}

export default App;
