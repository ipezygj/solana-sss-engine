# 🛡️ Compliance & Audit Trails

## On-Chain Compliance
SSS-2 enforces compliance at the protocol level. A flagged address cannot bypass the transfer hook.

## Off-Chain Audit Trails (Backend)
The SSS backend service uses structured JSON logging to maintain an immutable-style record.

Example Log:
    {"timestamp":"2026-03-13T12:00:00Z","level":"warn","message":"Address flagged","action":"FLAGGED","address":"8x..."}
