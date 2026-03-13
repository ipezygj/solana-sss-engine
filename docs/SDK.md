# 💻 TypeScript SDK

The `@stbr/sss-token` SDK is designed to be as intuitive as `ethers.js` or `spl-token`.

## Installation
    npm install @stbr/sss-token

## Initializing a Compliant Stablecoin (SSS-2)
    import { SolanaStablecoin, Presets } from "@stbr/sss-token";

    const stable = await SolanaStablecoin.create(connection, program, adminKeypair, {
      preset: Presets.SSS_2,
      name: "Institutional USD",
      symbol: "IUSD",
      decimals: 6,
    });

    // Blacklist an OFAC-sanctioned address
    await stable.compliance.blacklistAdd(suspectAddress, adminKeypair);
