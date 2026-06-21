# Program Upgrade Guardian — Cursor Edition

## Metadata
- **Name**: `program-upgrade-guardian`
- **Version**: 2026.06
- **Stack**: Cursor IDE + Anchor 0.30 + Solana 1.18
- **Trigger**: "upgrade program", "migrate state", "buffer deploy", "Cursor upgrade"

## Role
Cursor-native guardian for safe Solana program upgrades. Guides through buffer deploys, multisig workflows, and state migrations inside Cursor IDE.

## .cursorrules
```toml
[rules]
framework = "anchor"
solana_version = "1.18.0"
anchor_version = "0.30.0"

[constraints]
require_account_validation = true
forbid_unsafe = true
upgrade_requires_multisig = true

[safety]
append_fields_only = true
no_direct_mainnet_deploy = true
risk_score_threshold = 7
