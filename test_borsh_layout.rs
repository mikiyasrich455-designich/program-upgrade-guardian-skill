# Incident Response Playbooks

> **SEVERITY LEVELS**
> - SEV-1: Program bricked / funds locked / exploit active
> - SEV-2: Upgrade failed, program partially broken
> - SEV-3: Data corruption detected, program functional
> - SEV-4: Monitoring alert, potential issue

---

## SEV-1: Program Bricked / Funds Locked

### Immediate Actions (0-5 minutes)

```bash
# 1. STOP ALL OPERATIONS
# Do NOT attempt another upgrade without analysis

# 2. Assess scope
solana program show <PROGRAM_ID> | grep -E "Authority|Last Deployed|Data Length"

# 3. Check if authority is still accessible
# If multisig: gather signers for emergency rollback
# If single key: pray you have the keypair

# 4. Alert team
# - Post in #incidents channel
# - Page on-call engineer
# - Notify community if user funds at risk
```

### Rollback Procedure (5-30 minutes)

```bash
# PRE-REQUISITE: You must have the previous .so file
# If you don't, you're in deep trouble - build from previous git tag

# Step 1: Build/prepare rollback binary
git checkout <PREVIOUS_TAG>
anchor build

# Step 2: Create rollback buffer
solana-keygen new -o rollback-buffer.json --no-bip39-passphrase
solana program write-buffer target/deploy/<program>.so --buffer rollback-buffer.json

# Step 3: Set buffer authority to multisig
solana program set-buffer-authority $(solana-keygen pubkey rollback-buffer.json) \
  --new-buffer-authority <MULTISIG_PUBKEY>

# Step 4: Create emergency rollback proposal via Squads
sqd program-upgrade create \
  --multisig <MULTISIG_PUBKEY> \
  --program <PROGRAM_ID> \
  --buffer $(solana-keygen pubkey rollback-buffer.json) \
  --spill <EMERGENCY_WALLET> \
  --name "EMERGENCY ROLLBACK - SEV-1"

# Step 5: Expedite votes (bypass normal review if necessary)
sqd proposal vote --multisig <MULTISIG_PUBKEY> --proposal <PROPOSAL> --action Approve

# Step 6: Execute immediately
sqd proposal execute --multisig <MULTISIG_PUBKEY> --proposal <PROPOSAL>

# Step 7: Verify rollback
solana program dump <PROGRAM_ID> /tmp/verify-rollback.so
sha256sum /tmp/verify-rollback.so target/deploy/<program>.so
# MUST MATCH
```

### Post-Rollback (30-60 minutes)

```bash
# 1. Verify program is functional
anchor test --provider.cluster mainnet --skip-build

# 2. Check all user accounts
# Run account health check script

# 3. If state was corrupted during bad upgrade:
#   - Identify corrupted accounts
#   - Prepare state recovery migration
#   - Test recovery on devnet first

# 4. Document incident
#   - Timeline
#   - Root cause
#   - Impact assessment
#   - Lessons learned
```

---

## SEV-2: Upgrade Failed, Program Partially Broken

### Symptoms
- Program deploys but some instructions fail
- IDL mismatch between on-chain and client
- New features work but old features broken

### Response

```bash
# 1. Identify which instructions are broken
# Run comprehensive test suite
anchor test --provider.cluster mainnet

# 2. If fix is simple (e.g., missing account validation):
#   - Prepare hotfix
#   - Fast-track through devnet
#   - Deploy via normal multisig flow

# 3. If fix is complex:
#   - Consider SEV-1 rollback
#   - Or deploy patch upgrade

# Hotfix deployment (if safe)
git checkout -b hotfix/sev2-<description>
# Fix the bug
anchor build
# Normal buffer + multisig flow
# BUT add "HOTFIX" to proposal name for visibility
```

---

## SEV-3: Data Corruption Detected

### Symptoms
- User accounts deserialize incorrectly
- Missing fields or wrong values
- Migration didn't complete for all accounts

### Response

```bash
# 1. Identify corrupted accounts
# Query all accounts of affected type
solana program accounts <PROGRAM_ID> --output json | jq '.[] | select(...)'

# 2. If migration was incomplete:
#   - Re-run migration instruction for affected accounts
#   - Verify each account after migration

# 3. If data is truly corrupted (wrong values):
#   - Determine correct values from logs/events
#   - Write recovery migration
#   - Test on forked mainnet
#   - Deploy via multisig

# Recovery migration example:
# (See templates/migration-template.rs Pattern 2)
```

---

## SEV-4: Monitoring Alert

### Common Alerts
- High error rate on specific instruction
- Unusual authority changes
- Unexpected account size changes
- Buffer accounts created without proposal

### Response

```bash
# 1. Investigate
solana logs <PROGRAM_ID> --url mainnet | grep -i error

# 2. Check recent transactions
solana transaction-history <PROGRAM_ID> --limit 50

# 3. If false alarm: document and adjust alert threshold
# 4. If real issue: escalate to SEV-3 or SEV-2
```

---

## Emergency Contacts & Resources

| Role | Contact | Escalation Time |
|------|---------|----------------|
| On-call Engineer | #incidents Slack | Immediate |
| Team Lead | @lead-handle | 15 min if no response |
| Security | security@company.com | 30 min for SEV-1/2 |
| Squads Support | https://docs.squads.so | For multisig issues |
| Solana Foundation | security@solana.com | For network-level issues |

## Prevention Checklist

- [ ] Previous .so binary stored in secure location
- [ ] Previous IDL version tagged in git
- [ ] Multisig has enough active signers
- [ ] Emergency wallet funded for transaction fees
- [ ] Rollback procedure tested on devnet quarterly
- [ ] Monitoring alerts configured for all SEV levels
- [ ] Incident response runbook reviewed monthly
