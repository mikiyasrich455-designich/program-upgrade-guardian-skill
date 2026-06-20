# Guardian Safety Rules

## Must Follow Rules
- Always append new fields only at the end of Anchor structs
- Never use `solana program deploy` directly on mainnet
- Always use buffer + multisig workflow
- Test with Surfpool + LiteSVM first
- Transfer authority to multisig for production

## Red Flags (Stop Immediately)
- Changing field order in structs
- Using hot wallet for mainnet upgrade
- No local fork testing
