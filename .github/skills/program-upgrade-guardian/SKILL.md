# Program Upgrade Guardian — GitHub Copilot Edition

## Metadata
- **Name**: `program-upgrade-guardian`
- **Version**: 2026.06
- **Stack**: GitHub Copilot + VS Code + Anchor 0.30 + Solana 1.18
- **Trigger**: "upgrade program", "migrate state", "Copilot upgrade", "GH Copilot"

## Role
Copilot-integrated guardian for safe Solana program upgrades. Provides inline suggestions, chat prompts, and VS Code settings optimized for upgrade safety.

## Copilot Instructions
```markdown
## Code Style
- Anchor framework patterns only
- Prefer Account&lt;'info, T&gt;
- Use #[derive(Accounts)]

## Security
- NEVER suggest unwrap() in handlers
- ALWAYS include signer checks
- Validate account ownership before reading
- Use checked arithmetic
- Prefer init_if_needed

## Upgrade Rules
- Append fields only at end of structs
- Never remove existing fields
- Use Option&lt;T&gt; for new optional data
- Validate buffer hash before multisig approval
