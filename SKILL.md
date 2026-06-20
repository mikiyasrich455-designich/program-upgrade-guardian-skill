# program-upgrade-guardian-skill

## Name + Short Description
**Skill Name:** program-upgrade-guardian-skill  
**Short Description:** A careful "Guardian" that helps Solana builders safely upgrade live programs and migrate state without breaking user data, causing downtime, or losing funds.

## When to Use This Skill
- Changing account structures or adding new fields
- Upgrading a live mainnet program
- Transferring upgrade authority to multisig or DAO
- Migrating user data to new program version
- Testing upgrades before going to mainnet

## Core Capabilities
- Detects dangerous code and data layout changes
- Creates safe step-by-step upgrade plans
- Uses Surfpool + LiteSVM for realistic testing
- Generates buffer-based safe deployment commands
- Gives clear risk warnings and rollback plans

## Full Step-by-Step Guardian Pipeline
1. Discovery (check current program and authority)
2. Analysis (compare old and new code)
3. Local Testing (Surfpool + LiteSVM)
4. Migration Blueprint (how to safely move data)
5. Devnet Rehearsal
6. Mainnet Upgrade (safe buffer method)
7. Verification
8. Post-Upgrade Cleanup (reclaim rent)

## Recommended Tools (2026)
- LiteSVM — super fast local testing
- Surfpool — fork real mainnet state locally
- Anchor — program building and testing
- Helius — RPC and priority fees
- Solana CLI — deployment commands

## Safety Rules (Guardian Mode)
- Always test locally first with Surfpool + LiteSVM
- Never deploy directly with `solana program deploy` on mainnet
- Use buffer workflow for all production upgrades
- Add new fields only at the end of structs
- Move authority to multisig before mainnet

## Progressive Loading
Load safety rules first, then tools and commands only when needed.

## Example User Prompt
User: "Help me upgrade my program safely to v2"  
Guardian: Gives full pipeline, risk check, commands, and safety warnings.
