import { useEffect, useState, Fragment } from "react";
import { SignedUsdcTransactionsBySlot } from "./Solana";

const API_URL = "http://localhost:3000/transactions";
const REFRESH_INTERVAL = 2000;
const COL_COUNT = 6;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
function App() {
  const [currentTxns, setCurrentTxns] = useState<SignedUsdcTransactionsBySlot[]>([]);
  const [filteredTransactions, setFilteredTransactions] = useState<SignedUsdcTransactionsBySlot[]>([]);
  const [signatureFilterValue, setSignatureFilterValue] = useState<string>("");
  const [showSignatures, setShowSignatures] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  const fetchTransactions = async () => {
    try {
      const response = await fetch(API_URL);
      if (!response.ok) throw new Error("Fetch failed");

      const newTxns: SignedUsdcTransactionsBySlot[] = await response.json();

      // Only update if there are actual changes
      if (JSON.stringify(newTxns) !== JSON.stringify(currentTxns)) {
        setCurrentTxns(prevTxns => JSON.stringify(prevTxns) !== JSON.stringify(newTxns) ? newTxns : prevTxns);
      }

      setError(null);
    } catch (err: any) {
        setError(err.message);
    } finally {
        setLoading(false);
    }
  };

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Repeatedly poll for updates
  useEffect(() => {
    let isMounted = true;

    const poll = async () => {
      if (isMounted) {
        await fetchTransactions();
      }
    };

    poll();
    const interval = setInterval(poll, REFRESH_INTERVAL);

    return () => {
      isMounted = false;
      clearInterval(interval);
    };
  }, []);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Filter Transactions by Signature
  useEffect(() => {
    const filterTransactions = () => {
      if (!signatureFilterValue.trim()) {
        setFilteredTransactions(currentTxns);
        return;
      }

      const filtered = currentTxns
        .map(txnsBySlot => ({
          ...txnsBySlot,
          txns: txnsBySlot.txns.filter(tx => tx.signatures.some(sig => sig.startsWith(signatureFilterValue))),
        }))
        .filter(txnsBySlot => txnsBySlot.txns.length > 0);

      setFilteredTransactions(filtered);
    };

    filterTransactions();
  }, [signatureFilterValue, currentTxns]);

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  // Memoize the transaction table to prevent screen flicker
  const renderTableContent = () => {
    if (filteredTransactions.length === 0) {
      return <tr><td colSpan={COL_COUNT}>No transactions found</td></tr>;
    }

    return filteredTransactions.map(txnsBySlot => (
      <Fragment key={`slot-${txnsBySlot.slot}`}>
        <tr>
          <td className="slot-hdr" colSpan={COL_COUNT} style={{ fontWeight: "700"}}>
            Latest block: {txnsBySlot.slot}
          </td>
        </tr>
        {txnsBySlot.txns.map((tx) => (
          <Fragment key={tx.signatures[0]}>
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
              <td colSpan={COL_COUNT-1}>{tx.signatures.map(sig => (<p key={sig}>{sig}</p>))}</td>
            </tr>
            )}
          </Fragment>
        ))}
      </Fragment>
    ));
  };

  // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  return (
    <div className="div-top">
      {loading && <p>Loading transactions...</p>}
      {error && <p style={{ color: "red" }}>Error: {error}</p>}
      <div>
        <input
          type="text"
          className="search-term"
          placeholder="Filter by start of transaction hash..."
          value={signatureFilterValue}
          onChange={e => setSignatureFilterValue(e.target.value)}
        />
        <label className="sig-label">
            Show Signatures&nbsp;&nbsp;
            <input
              type="checkbox"
              checked={showSignatures}
              onChange={e => setShowSignatures(e.target.checked)}
            />
        </label>
      </div>
      <table className="txn-table">
        <thead>
          <tr><th colSpan={COL_COUNT}>Solana USDC Transactions</th></tr>
        </thead>
        <tbody>
          {renderTableContent()}
        </tbody>
      </table>
    </div>
  );
}

export default App;
