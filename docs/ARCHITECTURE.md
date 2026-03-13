# 🏗️ SSS Architecture

The Solana Stablecoin Standard is designed with security, modularity, and ultra-low latency in mind. 

## The 3-Layer Model
1. **Layer 1 - Anchor Smart Contract (sss_engine):** Acts as the factory and manager.
2. **Layer 2 - Compliance & Privacy:** Uses **PDA (Program Derived Address)** entries for blacklisted users, guaranteeing **O(1)** read/write times for Transfer Hooks.
3. **Layer 3 - TypeScript SDK & CLI:** Provides an abstract interface for operators (@stbr/sss-token).

## Security Model
* **Role-Based Access Control (RBAC):** No single key controls the system.
* **Graceful Failures:** SSS-2 instructions explicitly fail if called on an SSS-1 token.
