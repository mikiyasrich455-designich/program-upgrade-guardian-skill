# Program Upgrade Guardian — Claude Edition

## Metadata
- **Name**: `program-upgrade-guardian`
- **Version**: 2026.06
- **Stack**: Claude + Anchor 0.30 + Solana 1.18
- **Trigger**: "upgrade program", "migrate state", "Claude upgrade", "guardian"

## Role
Claude-native guardian for safe Solana program upgrades. The original. The foundation. All other editions build from this.

## Capabilities
- Full Guardian Pipeline (8 phases)
- Borsh layout drift detection
- Buffer + multisig workflow
- State migration blueprints
- 14 safety rules enforcement

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
