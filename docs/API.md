# 🔌 Backend API Reference

The Dockerized Node.js backend acts as the bridge between fiat banking APIs, sanctions screening, and the Solana blockchain.

## Endpoints
* `GET /health` - System status.
* `POST /api/v1/fiat/mint-request` - Initiates fiat-to-stablecoin minting process.
* `POST /api/v1/compliance/screen` - Checks target address against sanctions lists.
* `POST /api/v1/webhooks/onchain-events` - Listens to on-chain events.
