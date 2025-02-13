import { useEffect, useState } from "react";

interface SignedUsdcTransaction {
    slot: number;
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
    const [transactions, setTransactions] = useState<SignedUsdcTransaction[]>([]);
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
                throw new Error("Fetch failed");
            }

            const data: SignedUsdcTransaction[] = await response.json();

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
            <table border={1} cellPadding={8} style={{ width: "100%", marginTop: "10px" }}>
                <thead>
                <tr>
                    <th>Slot</th>
                    <th>Signatures</th>
                    <th>From</th>
                    <th>To</th>
                    <th>Amount (USDC)</th>
                </tr>
                </thead>
                <tbody>
                {transactions.length === 0 ? (
                    <tr><td colSpan={5} style={{ textAlign: "center" }}>No transactions found</td></tr>
                ) : (
                    transactions.map((tx, index) => (
                        <tr key={index}>
                            <td>{tx.slot}</td>
                            <td>
                                {tx.signatures.map((sig, i) => (
                                    <div key={i}>{sig}</div>
                                ))}
                            </td>
                            <td>{tx.txn.from}</td>
                            <td>{tx.txn.to}</td>
                            <td>{tx.txn.amount.toFixed(6)}</td>
                        </tr>
                    ))
                )}
                </tbody>
            </table>
        </div>
    );
}

export default App;
