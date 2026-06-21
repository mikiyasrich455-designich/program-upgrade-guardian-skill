# Safety Rules - Non-Negotiable

> **VIOLATION OF ANY RULE = UPGRADE BLOCKED**

## Rule Matrix

| ID | Rule | Severity | Auto-Block | Check Command |
|----|------|----------|------------|---------------|
| R1 | Never deploy directly to mainnet | CRITICAL | Yes | `solana config get \| grep "RPC URL"` |
| R2 | Authority must be multisig on mainnet | CRITICAL | Yes | `solana program show <ID> \| grep Authority` |
| R3 | Buffer authority must match program authority | CRITICAL | Yes | `solana program show <BUFFER> \| grep Authority` |
| R4 | All tests must pass before mainnet | CRITICAL | Yes | `cargo test && anchor test` |
| R5 | IDL compatibility check must pass | CRITICAL | Yes | CI gate `.github/workflows/idl-check.yml` |
| R6 | Risk score must be <= 7 without second review | HIGH | Yes | Risk analyst evaluation |
| R7 | Rollback plan must be documented | HIGH | Yes | Check `docs/incident-response.md` |
| R8 | Devnet rehearsal must succeed | HIGH | Yes | Full test suite on devnet |
| R9 | Borsh layout drift must be detected | HIGH | Yes | `check_borsh_layout` tool |
| R10 | New fields must be appended at end | MEDIUM | No | Manual IDL review |
| R11 | Optional<T> preferred for new fields | MEDIUM | No | Code review |
| R12 | Account resize must include rent check | MEDIUM | No | Migration code review |
| R13 | Events must be emitted for migrations | LOW | No | Code review |
| R14 | 24h monitoring plan required | LOW | No | Post-upgrade checklist |

## Detailed Rules

### R1: No Direct Mainnet Deploy
```
BLOCKED: solana program deploy <program.so> --url mainnet
ALLOWED:  solana program write-buffer <program.so> + multisig proposal
```

**Why**: Direct deploy gives no review window, no rollback, no accountability.

### R2: Multisig Authority Required
```bash
# Check current authority
solana program show <PROGRAM_ID>

# MUST show a Squads multisig address, NOT a single keypair like:
# ❌ 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU (single wallet)
# ✅ 59eNrRrxrZMdqJxS7J3WGaV4MLLog2er14kePiWVjXtY (Squads vault)
```

**Why**: Single-key authority = single point of failure. One compromised key = program takeover.

### R3: Buffer Authority Match
```bash
# After creating buffer, verify:
solana program show <BUFFER_PUBKEY> | grep "Authority"

# Must match program authority exactly
```

**Why**: Mismatched authority means the multisig can't execute the upgrade.

### R4: All Tests Pass
```bash
# Minimum test suite
cargo test --features test-bpf
anchor test --skip-build

# Must include:
# - Unit tests (LiteSVM)
# - Integration tests (Surfpool fork)
# - Migration tests (if state changes)
# - IDL compatibility tests
```

### R5: IDL Compatibility Gate
```yaml
# .github/workflows/idl-check.yml
# Must pass before any PR merge
# Checks:
# - Account struct field additions only at end
# - No field removals
# - No type changes to existing fields
# - Instruction signature compatibility
```

### R6: Risk Score Threshold
```
Score 1-3:  Warden approves, proceed
Score 4-6:  Warden approves, extra monitoring
Score 7-8:  BLOCKED - Requires second senior engineer review
Score 9-10: BLOCKED - Requires team lead + security audit
```

### R7: Rollback Plan Required
Every upgrade must document:
1. Previous program binary hash (sha256)
2. Previous IDL version
3. Rollback buffer creation steps
4. State corruption detection method
5. Emergency contact for execution

### R8: Devnet Rehearsal
```bash
# Full rehearsal checklist:
# 1. Deploy to devnet
anchor deploy --provider.cluster devnet

# 2. Run full test suite
anchor test --provider.cluster devnet

# 3. Test migration (if applicable)
# 4. Verify authority is still multisig
# 5. Test rollback procedure
```

### R9: Borsh Layout Detection
```rust
// Automated check in CI:
// Compare old_account_size vs new_account_size
// If size changed without migration instruction = BLOCK

// Manual check:
// 1. Export old IDL
// 2. Export new IDL  
// 3. diff -u old_idl.json new_idl.json
// 4. Any breaking change = reject
```

### R10-R14: Best Practices
- **R10**: Append-only field additions preserve backward compatibility
- **R11**: `Option<T>` for new fields allows graceful handling of old accounts
- **R12**: `Rent::get()?.minimum_balance(new_size)` before any realloc
- **R13**: `emit!(MigrationEvent { ... })` for audit trail
- **R14**: Monitor program logs, account access patterns, error rates for 24h

## Red Flags - Immediate STOP

If you see ANY of these, HALT the upgrade:

1. `solana program deploy` on mainnet
2. Program authority is a single wallet
3. Buffer authority != program authority
4. Tests failing or skipped
5. IDL shows field removal or reorder
6. No rollback plan documented
7. Risk score >= 7 without second review
8. Devnet rehearsal not performed
9. New account size > old size without migration code
10. `unwrap()` or `expect()` in migration logic

## Approval Workflow

```
Developer submits upgrade
    ↓
CI runs: tests + IDL check + lint
    ↓
Risk Analyst scores the change
    ↓
[Score <= 6] → Warden approves → Proceed to devnet
[Score 7-8]  → BLOCKED → Second review required
[Score 9-10] → BLOCKED → Team lead + security audit
    ↓
Devnet rehearsal succeeds
    ↓
Multisig proposal created
    ↓
Multisig members vote
    ↓
Execute on mainnet
    ↓
24h monitoring
    ↓
Cleanup (reclaim buffer rent)
```
