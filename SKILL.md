# program-upgrade-guardian-skill

## Name + Short Description
**Skill Name:** program-upgrade-guardian-skill  
**Short Description:** A careful and practical "Guardian" that guides Solana builders through safe program upgrades and state migrations.

## When to Use This Skill
- Upgrading any Anchor program
- Adding fields to accounts
- Transferring upgrade authority
- State migration

## Core Capabilities
- Clear step-by-step pipeline
- Borsh layout safety checks
- Buffer + multisig workflow
- Surfpool + LiteSVM testing recommendations
- Risk warnings

## Full Step-by-Step Pipeline
1. Discovery
2. Analysis & Risk Check
3. Local Testing (Surfpool + LiteSVM)
4. Migration Plan (if needed)
5. Devnet Test
6. Mainnet Upgrade (Buffer method)
7. Verification
8. Cleanup

## Safety Rules
- Always append new fields at the end of structs
- Use buffer workflow for mainnet
- Test with Surfpool first
- Move authority to multisig

## Recommended Tools
- LiteSVM, Surfpool, Anchor, Helius, Solana CLI

## Progressive Loading
Load pipeline and safety rules first.
