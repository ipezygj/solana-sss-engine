**🔥 DEVNET DEPLOYMENT PROOF:** ``

# Solana Stablecoin Standard (SSS) 🌐

An open-source, production-ready standard and modular SDK for issuing stablecoins on Solana. Built on top of **Token-2022 (Token Extensions)**, SSS provides configurable presets for both decentralized minimal stablecoins and highly regulated, compliant stablecoins.

## 📦 The Layers
1. **Core SDK:** TypeScript/CLI toolkit for initialization and operations.
2. **Compliance Modules:** On-chain Blacklist (PDA-based) and Transfer Hooks.
3. **Standard Presets:** Opinionated, ready-to-deploy configurations (SSS-1 & SSS-2).

## ⚖️ Preset Comparison

| Feature | SSS-1 (Minimal) | SSS-2 (Compliant) |
| :--- | :---: | :---: |
| **Use Case** | DAOs, Internal tokens | Regulated Fiat-backed (USDC/USDT) |
| **Mint & Freeze Auth** | ✅ | ✅ |
| **Permanent Delegate** | ❌ | ✅ (For asset seizure) |
| **Transfer Hook** | ❌ | ✅ (For blacklist enforcement) |
| **Default Account Frozen** | ❌ | ❌ |

## 🚀 Quick Start

    npm install -g @stbr/sss-token
    sss-token init --preset sss-2 --name "Brazil USD" --symbol "BRUSD"

*For detailed instructions, see the docs/ folder.*
