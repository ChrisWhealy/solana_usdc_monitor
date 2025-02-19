#-------------------------------------------------------------------------------
FROM node:18-alpine AS build-frontend
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

#-------------------------------------------------------------------------------
FROM rust:1.84-slim AS build-backend
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
COPY backend/ .
RUN cargo build --release

#-------------------------------------------------------------------------------
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build-backend /app/target/release/solana_usdc_backend /app/solana_usdc_backend
COPY --from=build-frontend /app/frontend/dist /app/frontend/dist

ENV RUST_LOG=info
ENV STATIC_FILES_PATH=/app/frontend/dist
ENV SOLANA_RPC_URL=https://api.mainnet-beta.solana.com

EXPOSE 3000
CMD ["/app/solana_usdc_backend"]
