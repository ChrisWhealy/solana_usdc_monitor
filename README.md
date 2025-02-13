# Solana Blockchain Monitor

## Backend

Rust application that exposes USDC transaction information via http://localhost:3000/transactions

1) Ensure Rust is installed

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2) Clone repo

   ```bash
   % git clone https://github.com/ChrisWhealy/solana_usdc_monitor.git
   % cd solana_usdc_monitor
   ```
 
3) Start Web server

   ```bash
   % cd backend
   % cargo run
       Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.71s
        Running `target/debug/solana_usdc_backend`
   [2025-02-13T16:19:23Z INFO  solana_usdc_backend] Server running on http://127.0.0.1:3000
   [2025-02-13T16:19:23Z INFO  solana_usdc_backend::slot] ---> Slot 320414738
   [2025-02-13T16:19:25Z INFO  solana_usdc_backend::slot]      get_block request took 1.701s
   [2025-02-13T16:19:25Z INFO  solana_usdc_backend::transactions] TX detected: 5zsgeDG984Nd3Q65JPFYSaKMu5fzETLL56Dyj4e87sNd sent 528.572454 USDC to 2i3TLNNdStHXorusiDcquAVe6BmXQYMc3qfzyKbX1c7V
   [2025-02-13T16:19:25Z INFO  solana_usdc_backend::transactions] TX detected: 6Vt8Xd65nTNzah9oFkoLXUxpR6mdPMqY5Bimz1MAQb9M sent 0.948717 USDC to 2am4n8dNv4HrVUVpL4SFqmdzfRznyxrBx3SXwp8WZ9pR
   [2025-02-13T16:19:25Z INFO  solana_usdc_backend::slot] <--- Slot 320414738: Processed 2086 transactions in 1.712s
   [2025-02-13T16:19:25Z INFO  solana_usdc_backend] Sleeping for 1 second(s)
   ```
   
## Frontend

```bash
% cd frontend/solana_usdc_ui
% npm run dev

> solana-usdc-ui@0.0.0 dev
> vite


VITE v6.1.0  ready in 232 ms

➜  Local:   http://localhost:5173/
➜  Network: use --host to expose
➜  press h + enter to show help
```
   
