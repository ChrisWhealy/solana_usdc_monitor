# Solana Blockchain Monitor

This is a small PoC application to monitor the Solana blockchain for USDC transactions.

## Execution

1. Ensure [Docker Desktop](https://docs.docker.com/get-started/introduction/get-docker-desktop/) has been installed
1. Clone this repo into some local directory then change into that directory
1. Build the Docker container:

    `docker build -t solana-usdc-monitor .`
1. Run the Docker container

    `docker run -p 3000:3000 solana-usdc-monitor`

   ```bash
   % docker run -p 3000:3000 solana-usdc-monitor
   [2025-02-19T09:26:49Z INFO  solana_usdc_backend] Server running on http://0.0.0.0:3000
   [2025-02-19T09:26:49Z INFO  solana_usdc_backend] Monitoring: https://api.mainnet-beta.solana.com
   [2025-02-19T09:26:51Z INFO  solana_usdc_backend::slot] ---> Slot 321659973
   [2025-02-19T09:26:52Z INFO  solana_usdc_backend::slot]      get_block request took 649.188ms
   [2025-02-19T09:26:52Z INFO  solana_usdc_backend::transaction] TX detected: 7nQFY83oXFTZduD3oQpwxC4AVMwGdwRooL7WhtkMBkkg sent 8.867259 USDC to 2cidtavnfrCcjbAHyWkcUfH9zWC6LmajrL4GWUFBKeje
   [2025-02-19T09:26:52Z INFO  solana_usdc_backend::slot] <--- Slot 321659973: Processed 1605 transactions in 652.987ms
   [2025-02-19T09:26:52Z INFO  solana_usdc_backend::slot] ---> Slot 321659974
   [2025-02-19T09:26:52Z INFO  solana_usdc_backend::slot]      get_block request took 629.171ms
   [2025-02-19T09:26:52Z INFO  solana_usdc_backend::slot] <--- Slot 321659974: Processed 1570 transactions in 632.451ms
   [2025-02-19T09:26:53Z INFO  solana_usdc_backend] Sleeping for 1s
   [2025-02-19T09:26:54Z INFO  solana_usdc_backend::slot] ---> Slot 321659975
   [2025-02-19T09:26:54Z INFO  solana_usdc_backend::slot]      get_block request took 628.327ms
   [2025-02-19T09:26:54Z INFO  solana_usdc_backend::slot] <--- Slot 321659975: Processed 1524 transactions in 631.708ms
   [2025-02-19T09:26:54Z INFO  solana_usdc_backend::slot] ---> Slot 321659976
   [2025-02-19T09:27:05Z INFO  solana_usdc_backend::slot]      get_block request took 10.850s
   [2025-02-19T09:27:05Z INFO  solana_usdc_backend::transaction] TX detected: 5ha8tFqUGA36x3Xd2CsUg4pa4pUEBLQgbxfq1zFwEyCA sent 54.239793 USDC to 6q7Tj6RjpMdfMPY4wzt3PjSpebzE93GXdWGnH9XNPLCD
   [2025-02-19T09:27:05Z INFO  solana_usdc_backend::slot] <--- Slot 321659976: Processed 1669 transactions in 10.854s
   ...
   ```
1. Visit <http://localhost:3000> to view USDC transactions by slot number
