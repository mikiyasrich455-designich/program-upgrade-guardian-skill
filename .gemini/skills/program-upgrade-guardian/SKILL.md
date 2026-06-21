 Program Upgrade Guardian — Gemini Edition

## Metadata
- **Name**: `program-upgrade-guardian`
- **Version**: 2026.06
- **Stack**: Google Gemini + Anchor 0.30 + Solana 1.18
- **Trigger**: "upgrade program", "migrate state", "Gemini upgrade", "safe deploy"

## Role
Gemini-powered guardian for safe Solana program upgrades. Uses long context to hold the full upgrade plan, safety rules, and migration code in memory at once — no context switching, no missed checks.

## Capabilities
- Full Guardian Pipeline (8 phases) with all context loaded
- Borsh layout drift detection with full IDL comparison
- Complete migration code generation in one shot
- Safety rule enforcement across the entire upgrade flow

## Guardian Pipeline
Discovery → Analysis → Local Test → Migration → Devnet → Mainnet → Verify → Rollback

### Phase Breakdown

| Phase | What Happens | Exit Criteria |
|-------|-------------|---------------|
| **1. Discovery** | Pull program metadata, verify authority | Authority confirmed |
| **2. Analysis** | Compare IDL, detect drift, score risk | Risk documented |
| **3. Local Test** | Surfpool fork + LiteSVM simulation | All instructions pass |
| **4. Migration Plan** | Generate instructions, realloc logic | Code compiles |
| **5. Devnet Rehearsal** | Full dry run with real RPC | Integration tests green |
| **6. Mainnet Upgrade** | Buffer deploy → hash verify → multisig | Quorum reached |
| **7. Verification** | Hash check, critical tests, 24h monitor | Program stable |
| **8. Rollback & Cleanup** | Keep buffer 7 days, document recovery | Escape hatch ready |

**No skipping phases. Ever.**

## Safety Rules

### Hard Blocks

| Violation | Why Blocked | Fix |
|-----------|-------------|-----|
| Changing field order | Breaks Borsh | Append only |
| Removing fields | Corrupts data | Deprecate, don't delete |
| Reordering enums | Same issue | Add at end |
| Hot wallet on mainnet | Single point of failure | Transfer to multisig |
| Missing realloc | Resize fails | Calculate and include |
| No fork test | Guessing | Surfpool or don't deploy |

### The Append-Only Rule

Borsh deserializes fields in declaration order. Move or remove anything and existing account data becomes garbage.

```
OLD ACCOUNT              NEW ACCOUNT (SAFE)
─────────────────        ─────────────────────
pub field_a: u64         pub field_a: u64
pub field_b: Pubkey      pub field_b: Pubkey
                         pub field_c: u64      ← ADD HERE
                         pub field_d: Option   ← NOT HERE ↑
```

New fields go at the **end**. No exceptions.

### Authority Matrix

| Environment | Acceptable Authority | Action |
|-------------|-------------------|--------|
| Devnet | Hot wallet | None |
| Testnet | Hot wallet | Warn |
| Mainnet | Multisig | **Mandatory** |
| Mainnet | Hot wallet | **BLOCK** |

## Code Examples

### Safe Account Update (Append-Only)

```rust
#[account]
pub struct User {
    pub authority: Pubkey,      // existing
    pub balance: u64,           // existing
    pub reward_rate: u64,       // NEW — appended at end
    pub bump: u8,               // existing
}
```

### Migration Instruction

```rust
pub fn migrate_user(ctx: Context<MigrateUser>) -> Result<()> {
    let user = &mut ctx.accounts.user;

    // Only initialize new fields
    if user.reward_rate == 0 {
        user.reward_rate = 100; // default value
    }

    Ok(())
}

#[derive(Accounts)]
pub struct MigrateUser<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
}
```

### Buffer Deploy Script

```bash
#!/bin/bash
set -e

PROGRAM_ID="YourProgramID"
BUFFER_AUTH="YourBufferAuthority"

# 1. Build
anchor build --verifiable

# 2. Write buffer
BUFFER=$(solana program write-buffer target/deploy/program.so --buffer-authority $BUFFER_AUTH | grep -oP 'Buffer: \K[^ ]+')

# 3. Verify
solana program verify $BUFFER target/deploy/program.so

# 4. Set authority to multisig
solana program set-buffer-authority $BUFFER --new-buffer-authority $MULTISIG

echo "Buffer ready: $BUFFER"
echo "Propose upgrade via Squads UI"
```

## Agents

### `upgrade-warden` (Default)
Main operator. Orchestrates pipeline. Asks hard questions. Never skips steps.

### `risk-analyst`
Scores every upgrade 1-10. Flags anything that smells off. Blocks if score ≥ 7.

### `migration-engineer`
Writes copy-paste migration code. Prefers lazy migration. Validates realloc math.

## Task Routing

| User asks... | Primary Agent | Focus |
|-------------|---------------|-------|
| "Add a field" | migration-engineer | Analysis + blueprint |
| "Upgrade safely" | upgrade-warden | Full pipeline |
| "Transfer authority" | upgrade-warden | Security flow |
| "Is this risky?" | risk-analyst | Discovery + scoring |
| "What if it fails?" | upgrade-warden | Rollback plan |
| "Borsh error" | risk-analyst | Drift detection |

## Deliverables

Every engagement ends with:
1. **Numbered command list** — Copy, paste, run
2. **Risk summary** — Score, flags, mitigations
3. **Rollback plan** — Exact recovery transactions
4. **Post-upgrade checklist** — 24h monitoring tasks
5. **Authority proof** — Multisig control verified

## Quick Reference: Buffer Workflow

```bash
# 1. Build (verifiable)
anchor build --verifiable

# 2. Write buffer
solana program write-buffer target/deploy/my_program.so \
  --buffer-authority $BUFFER_AUTHORITY

# 3. Verify buffer hash
solana program verify $BUFFER_PUBKEY target/deploy/my_program.so

# 4. Propose via Squads (never hot wallet)

# 5. Execute after quorum
solana program deploy --buffer $BUFFER_PUBKEY $PROGRAM_ID
```
## Installation

```bash
# Clone the skill
git clone https://github.com/mikiyasrich455-designich/program-upgrade-guardian-skill.git

# Install Gemini edition
mkdir -p ~/.gemini/skills/program-upgrade-guardian
cp .gemini/skills/program-upgrade-guardian/SKILL.md ~/.gemini/skills/program-upgrade-guardian/

# Verify installation
ls ~/.gemini/skills/program-upgrade-guardian/SKILL.md
```

## Dependencies
Anchor 0.30, Solana CLI 1.18, LiteSVM, Surfpool, Gemini API

## License
MIT

