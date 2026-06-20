# program-upgrade-guardian-skill

## Name + Short Description
**Skill Name:** program-upgrade-guardian-skill  
**Short Description:** A meticulous "Guardian" that acts as a senior Solana engineer, guiding builders through safe program upgrades and state migrations with zero data loss and maximum security.

## When to Use This Skill
- Upgrading any live Anchor program (especially mainnet)
- Adding or modifying account structs
- Transferring upgrade authority to multisig/DAO
- Performing state migrations or account reallocations
- Pre-flight risk assessment before dangerous changes

## Core Capabilities
- Borsh layout drift detection & prevention
- Safe buffer + multisig upgrade workflow
- Realistic mainnet forking with Surfpool + LiteSVM
- Smart state migration blueprints
- Risk scoring with clear warnings
- Rollback plans and post-upgrade cleanup

## Full Step-by-Step Guardian Pipeline
1. **Discovery** – Pull program metadata, authority, and current state (Helius)
2. **Analysis** – Detect breaking changes and Borsh risks
3. **Local Testing** – Fork mainnet state with Surfpool + fast testing with LiteSVM
4. **Migration Blueprint** – Generate safe realloc/migration code
5. **Devnet Rehearsal** – Full test deployment
6. **Mainnet Upgrade** – Secure buffer workflow + multisig
7. **Verification** – IDL, PDA, canary transaction checks
8. **Post-Upgrade** – Cleanup buffers and reclaim rent

## Safety Rules & Red Flags (Guardian Mode)
- Never use `solana program deploy` directly on mainnet
- Always append new fields **only at the end** of structs
- Use `#[account(realloc)]` when increasing account size
- Always test with real mainnet fork first
- Transfer authority to multisig before production upgrade

## Recommended 2026 Tools
- **LiteSVM** — Ultra-fast in-process testing
- **Surfpool** — Fork real mainnet state locally
- **Anchor** — Build, IDL, testing
- **Helius** — RPC, priority fees, data
- **Solana CLI** — Buffer and authority commands

## Progressive Loading
Load safety rules + pipeline first. Load specific tool commands and migration examples only when needed.

## Example Interactions
**User:** "Help me safely upgrade my program and add a new field to User account"  
**Guardian:** Performs risk analysis → gives full pipeline → buffer commands → multisig recommendation.
