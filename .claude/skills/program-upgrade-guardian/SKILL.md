# Program Upgrade Guardian

&gt; Production-Ready Skill for Solana AI Kit — Safe program upgrades, state migrations, and authority transfers.

## Capabilities
- Full Guardian Pipeline (Discovery → Analysis → Testing → Upgrade → Rollback)
- Automatic Borsh layout drift detection
- Safe buffer + multisig workflow
- State migration blueprints + rollback plans
- 14 strict safety rules & red flags

## Agent Personas
| Agent | Role |
|-------|------|
| upgrade-warden | Main Guardian (orchestrates pipeline) |
| risk-analyst | Quantifies danger (scores 1-10) |
| migration-engineer | Writes bulletproof migration code |

## Safety Rules
1. NEVER deploy directly to mainnet
2. Authority must be multisig on mainnet
3. All tests must pass before mainnet
4. Risk score ≤ 7 without second review
5. Rollback plan must be documented

## MCP Tools
- check_borsh_layout
- simulate_upgrade
- calculate_risk_score
- generate_migration_code
- verify_multisig
- create_upgrade_proposal

## Quick Start
```bash
python3 install.py
