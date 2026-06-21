# Upgrade Commands - Copy-Paste Ready

> **WARNING**: Read the entire pipeline before executing any command. Never skip steps.

## Phase 0: Pre-Flight Checks

```bash
# 1. Verify you're NOT on mainnet
solana config get | grep "RPC URL"
# Should show devnet or localhost. If mainnet, STOP.

# 2. Run dependency checker
chmod +x install.sh && ./install.sh

# 3. Verify program authority
solana program show <PROGRAM_ID> | grep "Authority"
# Should show your multisig pubkey, NOT a single keypair
```

## Phase 1: Build & Verify

```bash
# Build the new program
anchor build

# Verify the build hash
sha256sum target/deploy/<program_name>.so
# Record this hash for verification later

# Generate new IDL
anchor idl init --filepath target/idl/<program_name>.json <PROGRAM_ID>
# OR if IDL exists:
anchor idl upgrade --filepath target/idl/<program_name>.json <PROGRAM_ID>
```

## Phase 2: Borsh Layout Check

```bash
# Compare old vs new IDL for breaking changes
# (This should be automated in CI, but run manually too)

# Install IDL diff tool if needed
npm install -g @coral-xyz/anchor-cli

# Extract account layouts
jq '.accounts' target/idl/<program_name>.json > new_accounts.json
git show HEAD:target/idl/<program_name>.json | jq '.accounts' > old_accounts.json

# Manual check: ensure fields are ONLY appended at the end
# Any field removal, reorder, or size reduction = BREAKING
```

## Phase 3: Buffer Creation (Multisig Required)

```bash
# Create a buffer keypair
solana-keygen new -o buffer-keypair.json --no-bip39-passphrase

# Write program to buffer (DO NOT deploy yet)
solana program write-buffer target/deploy/<program_name>.so --buffer buffer-keypair.json

# Record buffer address
BUFFER_PUBKEY=$(solana-keygen pubkey buffer-keypair.json)
echo "Buffer: $BUFFER_PUBKEY"

# Set buffer authority to multisig (CRITICAL)
solana program set-buffer-authority $BUFFER_PUBKEY --new-buffer-authority <MULTISIG_PUBKEY>

# Verify buffer authority changed
solana program show $BUFFER_PUBKEY | grep "Authority"
```

## Phase 4: Squads Multisig Upgrade Proposal

```bash
# Using Squads CLI (sqd)

# 1. Create upgrade proposal
sqd program-upgrade create \
  --multisig <MULTISIG_PUBKEY> \
  --program <PROGRAM_ID> \
  --buffer <BUFFER_PUBKEY> \
  --spill <YOUR_WALLET_PUBKEY> \
  --name "Upgrade <program_name> to v<X.Y.Z>"

# 2. Get proposal address
sqd program-upgrade list --multisig <MULTISIG_PUBKEY>

# 3. Have multisig members vote
sqd proposal vote --multisig <MULTISIG_PUBKEY> --proposal <PROPOSAL_PUBKEY>

# 4. Execute once threshold reached
sqd proposal execute --multisig <MULTISIG_PUBKEY> --proposal <PROPOSAL_PUBKEY>

# 5. Verify upgrade succeeded
solana program show <PROGRAM_ID> | grep "Last Deployed"
```

## Phase 5: Post-Upgrade Verification

```bash
# 1. Verify program hash matches build
solana program dump <PROGRAM_ID> /tmp/verify.so
sha256sum /tmp/verify.so target/deploy/<program_name>.so
# MUST match exactly

# 2. Verify authority is still multisig
solana program show <PROGRAM_ID> | grep "Authority"

# 3. Test critical instructions
anchor test --skip-build
# Or run specific integration tests against mainnet RPC

# 4. Monitor for 24h
# Check program logs for errors
# Verify all user accounts are accessible
```

## Phase 6: Cleanup

```bash
# Reclaim buffer rent (after confirming upgrade is stable)
solana program close --buffers
# This returns rent to the spill address

# Update documentation
# Tag release in git
git tag -a v<X.Y.Z> -m "Mainnet upgrade: <description>"
git push origin v<X.Y.Z>
```

## Emergency: Rollback

```bash
# If upgrade is broken, you need the PREVIOUS .so file

# 1. Build old version (or use cached .so)
git checkout <PREVIOUS_COMMIT>
anchor build

# 2. Create rollback buffer
solana-keygen new -o rollback-buffer.json --no-bip39-passphrase
solana program write-buffer target/deploy/<program_name>.so --buffer rollback-buffer.json

# 3. Set authority to multisig
solana program set-buffer-authority $(solana-keygen pubkey rollback-buffer.json) \
  --new-buffer-authority <MULTISIG_PUBKEY>

# 4. Create rollback proposal via Squads
sqd program-upgrade create \
  --multisig <MULTISIG_PUBKEY> \
  --program <PROGRAM_ID> \
  --buffer $(solana-keygen pubkey rollback-buffer.json) \
  --spill <YOUR_WALLET_PUBKEY> \
  --name "ROLLBACK <program_name> to v<PREVIOUS_VERSION>"

# 5. Execute after votes
sqd proposal execute --multisig <MULTISIG_PUBKEY> --proposal <ROLLBACK_PROPOSAL>

# NOTE: This ONLY rolls back the program binary.
# If state was corrupted during migration, you need the migration rollback plan.
```

## One-Liner Safety Check

```bash
# Run this before ANY mainnet operation
solana config get | grep -q "mainnet" && echo "STOP: You're on mainnet" || echo "OK: Not on mainnet"
```
