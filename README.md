# program-upgrade-guardian-skill

## Name + Short Description
**Skill Name:** program-upgrade-guardian-skill  
**Short Description:** A meticulous "Guardian" AI skill that helps Solana builders safely upgrade live programs and perform state migrations with zero data loss and maximum security.

## When to Use This Skill
- Adding/changing Anchor account structs on live programs
- Upgrading high-TVL or production programs
- Transferring upgrade authority (to multisig/DAO)
- Performing state migrations or account reallocations
- Pre-flight risk assessment before mainnet changes

## Core Capabilities
- Automatic Borsh layout drift detection
- Safe buffer + multisig upgrade workflow
- Realistic mainnet forking with Surfpool + LiteSVM
- Smart state migration blueprint generation
- Risk scoring + guardian-style warnings
- Rollback plans and post-upgrade cleanup

## Full Step-by-Step Guardian Pipeline
1. **Discovery** – Pull program metadata and authority via Helius
2. **Analysis** – Detect breaking changes and layout risks
3. **Local Testing** – Fork mainnet state with Surfpool + fast testing with LiteSVM
4. **Migration Blueprint** – Generate safe realloc + migration code
5. **Devnet Rehearsal** – Full test deployment
6. **Mainnet Upgrade** – Execute via secure buffer workflow
7. **Verification** – IDL, PDA, canary transaction checks
8. **Post-Upgrade** – Buffer cleanup and rent reclamation

## Recommended Tools (2026 Stack)
- **LiteSVM** — Ultra-fast in-process testing
- **Surfpool** — Fork real mainnet state locally
- **Anchor** — Program build, IDL, testing
- **Helius** — RPC, priority fees, data queries
- **Solana CLI** — Buffer and authority management

## Safety Rules & Red Flags (Guardian Mode)
- Never run `solana program deploy` directly on mainnet
- Always use buffer + multisig workflow for production
- Append new fields **only at the end** of structs
- Use `#[account(realloc)]` for account resizing
- Test with real mainnet fork before upgrading

## Progressive Loading
Load core safety rules first, then tool commands and examples only when needed.

## Example Interaction
**User:** "Help me safely upgrade my program and add a new field to User account"  
**Guardian:** Performs risk analysis → gives migration plan → buffer commands → multisig recommendation.
