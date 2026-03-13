# 🏗️ SSS Architecture

The Solana Stablecoin Standard is designed with security, modularity, and ultra-low latency in mind. 

## The 3-Layer Model
1. **Layer 1 - Anchor Smart Contract (sss_engine):** Acts as the factory and manager. It initializes the Token-2022 mint via CPI (Cross-Program Invocation) and dynamically calculates account space based on enabled extensions.
2. **Layer 2 - Compliance & Privacy:**
   Instead of storing blacklists in a massive array (which hits compute limits), SSS uses **PDA (Program Derived Address)** entries for each blacklisted user ([b"blacklist", stablecoin, target]). This guarantees **O(1)** read/write times for Transfer Hooks.
3. **Layer 3 - TypeScript SDK & CLI:**
   Provides an abstract, ergonomic interface for operators (@stbr/sss-token), hiding the complexity of Token-2022 CPIs.

## Security Model
* **Role-Based Access Control (RBAC):** No single key controls the system.
* **Graceful Failures:** SSS-2 instructions (like seize) will explicitly fail with ErrorCode::ComplianceModuleDisabled if called on an SSS-1 token.
