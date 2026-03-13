# ⚙️ Operator Runbook (CLI)

The `sss-token` CLI allows rapid incident response and daily operations.

## Initialization
    sss-token init --preset sss-1 --name "DAO USD" --symbol "DUSD"
    sss-token init --preset sss-2 --name "Regulated USD" --symbol "RUSD"

## Compliance (SSS-2 Only)
    sss-token blacklist add <address> --reason "OFAC match"
    sss-token blacklist remove <address>
