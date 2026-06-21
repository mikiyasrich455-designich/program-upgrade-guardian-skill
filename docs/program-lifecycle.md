# Program Lifecycle

> Complete guide: Deploy → Operate → Upgrade → Sunset

---

## Phase 1: Deploy

### Initial Deployment Checklist

```bash
# 1. Build and verify
anchor build
sha256sum target/deploy/*.so

# 2. Deploy to devnet first
anchor deploy --provider.cluster devnet

# 3. Run full test suite
anchor test --provider.cluster devnet

# 4. Verify program
solana program show <PROGRAM_ID> --url devnet

# 5. Deploy to mainnet
anchor deploy --provider.cluster mainnet

# 6. IMMEDIATELY transfer authority to multisig
# (See commands/upgrade-commands.md Phase 3)

# 7. Verify authority transfer
solana program show <PROGRAM_ID> | grep Authority
# MUST show multisig address

# 8. Tag release
git tag -a v1.0.0 -m "Initial mainnet deployment"
git push origin v1.0.0
```

### Post-Deployment

- [ ] Program ID documented
- [ ] IDL published
- [ ] Authority is multisig
- [ ] Monitoring configured
- [ ] Emergency contacts updated

---

## Phase 2: Operate

### Daily Operations

```bash
# Monitor program health
solana program show <PROGRAM_ID>

# Check for unusual activity
solana logs <PROGRAM_ID> --url mainnet

# Verify authority hasn't changed
solana program show <PROGRAM_ID> | grep Authority
```

### Weekly Operations

- [ ] Review error rates
- [ ] Check account growth
- [ ] Verify multisig members are active
- [ ] Review pending proposals

### Monthly Operations

- [ ] Security audit dependencies (`cargo audit`)
- [ ] Review access logs
- [ ] Test backup/rollback procedure on devnet
- [ ] Update incident response contacts

---

## Phase 3: Upgrade

### See commands/upgrade-commands.md for full details

### Quick Reference

```
1. Discovery: Identify what needs to change
2. Analysis: Risk score, Borsh layout check
3. Local Test: LiteSVM + Surfpool
4. Migration Plan: Write migration code if needed
5. Devnet Rehearsal: Full deployment test
6. Mainnet Upgrade: Buffer + multisig proposal
7. Post-Upgrade: 24h monitoring
8. Cleanup: Reclaim buffer rent
```

### Version Numbering

| Version | Meaning | Example |
|---------|---------|---------|
| Major | Breaking change, migration required | v2.0.0 |
| Minor | New features, backward compatible | v1.1.0 |
| Patch | Bug fix, no state changes | v1.0.1 |

---

## Phase 4: Sunset

### When to Sunset
- Program no longer needed
- Migrated to new architecture
- Security vulnerability can't be patched
- Community has moved to fork

### Sunset Procedure

```bash
# 1. Announce sunset plan
#   - 30-day notice minimum
#   - Document migration path for users
#   - Provide support channels

# 2. Disable new deposits/operations
#   - Add sunset flag to program
#   - Reject new account creation
#   - Allow withdrawals only

# 3. Final upgrade: withdrawal-only mode
#   - Deploy via normal multisig flow
#   - Verify no new state can be created

# 4. Wait for all users to exit
#   - Monitor account count
#   - Provide support for stuck users

# 5. Close program account
#   - Requires multisig approval
#   - Returns rent to spill address

solana program close <PROGRAM_ID> \
  --recipient <SPILL_ADDRESS> \
  --authority <MULTISIG_PUBKEY>
# (Must be executed via Squads proposal)

# 6. Archive everything
#   - Final IDL
#   - Last known good state
#   - User exit records
#   - Incident history
```

### Emergency Sunset (Critical Vulnerability)

```bash
# If exploit is active:
# 1. Immediately set program to reject all instructions
# 2. Announce emergency
# 3. Guide users to exit
# 4. Close program when safe
```

---

## Authority Transfer Checklist

### Transfer TO Multisig (Recommended)

```bash
# 1. Create Squads multisig
sqd multisig-create --members <PUBKEY1> <PUBKEY2> <PUBKEY3> --threshold 2

# 2. Get vault address
export MULTISIG_VAULT=<VAULT_ADDRESS>

# 3. Transfer authority
solana program set-upgrade-authority <PROGRAM_ID> \
  --new-upgrade-authority $MULTISIG_VAULT

# 4. Verify
solana program show <PROGRAM_ID> | grep Authority
```

### Transfer FROM Multisig (Emergency Only)

```bash
# This requires multisig votes
sqd config-transaction-create \
  --multisig <MULTISIG_PUBKEY> \
  --action "ChangeUpgradeAuthority <NEW_AUTHORITY>"

# Then vote and execute
sqd proposal-vote --multisig <MULTISIG_PUBKEY> --transaction-index <INDEX> --action Approve
sqd config-transaction-execute --multisig <MULTISIG_PUBKEY> --transaction-index <INDEX>
```

---

## State Migration Decision Tree

```
Adding a field?
  ├── Append to end of struct?
  │   ├── Same total size? → No migration needed (backward compatible)
  │   └── Larger size? → Migration required (Pattern 2)
  ├── Insert in middle? → BLOCKED (breaking change)
  └── Remove field? → BLOCKED (breaking change)

Changing field type?
  ├── Same size? → Maybe safe (test thoroughly)
  └── Different size? → Migration required

Adding instruction?
  └── Always safe (no state impact)

Removing instruction?
  └── Check if any clients depend on it

Adding account type?
  └── Always safe (new PDA)
```
