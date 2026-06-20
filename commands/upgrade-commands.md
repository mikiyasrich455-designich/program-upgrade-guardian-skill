Upgrade Commands
Every command you need, ready to copy-paste. Replace placeholders before executing.
Table of Contents
Prerequisites

Phase 1: Discovery
Phase 2: Build & Verify
Phase 3: Buffer Management
Phase 4: Multisig Flow (Squads)
Phase 5: Deploy & Verify
Phase 6: Rollback
Environment Variables
Prerequisites
Required Tools
bash
# Verify installations
anchor --version      # >= 0.30.0
solana --version      # >= 1.18.0
sqd --version         # Squads CLI (optional but recommended)
Environment Variables
Set these in your shell or .env:
bash
export PROGRAM_ID="YourProgramPubkeyHere"
export BUFFER_AUTHORITY="YourBufferAuthorityPubkeyHere"
export SQUADS_AUTHORITY="YourSquadsAuthorityPubkeyHere"
export CLUSTER="mainnet-beta"  # or devnet
export KEYPAIR="~/.config/solana/id.json"
Phase 1: Discovery
Pull Program Metadata
bash
# Full program info
solana program show $PROGRAM_ID --output json

# Just the authority
solana program show $PROGRAM_ID | grep "Authority"

# Program accounts (requires Helius or similar)
curl -X POST https://mainnet.helius-rpc.com/?api-key=<KEY> \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getProgramAccounts",
    "params": [
      "'"$PROGRAM_ID"'",
      {"encoding": "base64", "filters": []}
    ]
  }'
IDL Comparison
bash
# Download on-chain IDL
anchor idl fetch $PROGRAM_ID --provider.cluster $CLUSTER > idl_onchain.json

# Compare with local
anchor idl parse --file target/idl/my_program.json > idl_local.json
diff idl_onchain.json idl_local.json
Phase 2: Build & Verify
Verifiable Build
bash
# Standard build
anchor build

# Verifiable build (required for mainnet)
anchor build --verifiable

# Output paths:
# target/deploy/my_program.so
# target/idl/my_program.json
# target/types/my_program.ts
Verify Build Hash
bash
# Get buffer hash
solana program dump $PROGRAM_ID /tmp/onchain_program.so
sha256sum /tmp/onchain_program.so
sha256sum target/deploy/my_program.so

# They must match before upgrade
Phase 3: Buffer Management
Write Buffer
bash
# Write program to buffer
solana program write-buffer target/deploy/my_program.so \
  --buffer-authority $BUFFER_AUTHORITY \
  --keypair $KEYPAIR

# Capture BUFFER_PUBKEY from output
export BUFFER_PUBKEY="BufferPubkeyFromOutput"
Verify Buffer
bash
# Check buffer matches local build
solana program show $BUFFER_PUBKEY --output json

# Dump and hash-check
solana program dump $BUFFER_PUBKEY /tmp/buffer_program.so
sha256sum /tmp/buffer_program.so target/deploy/my_program.so
Set Buffer Authority (if needed)
bash
# Transfer buffer authority to Squads
solana program set-buffer-authority $BUFFER_PUBKEY \
  --new-buffer-authority $SQUADS_AUTHORITY \
  --keypair $KEYPAIR
List Buffers
bash
# Find all buffers owned by your key
solana program show --buffers --keypair $KEYPAIR
Close Buffer (cleanup)
bash
# Recover rent from old buffer
solana program close $BUFFER_PUBKEY \
  --keypair $KEYPAIR \
  --recipient <RENT_RECOVERY_PUBKEY>
Phase 4: Multisig Flow (Squads)
Option A: Squads UI (Recommended)
Go to https://v3.squads.so
Select your multisig
Navigate to Programs → Upgrade Program
Paste PROGRAM_ID and BUFFER_PUBKEY
Propose transaction
Share proposal link with signers
Execute after threshold reached
Option B: Squads CLI
bash
# Install if needed
npm install -g @sqds/cli

# Propose upgrade
sqd program-upgrade propose \
  --program-id $PROGRAM_ID \
  --buffer $BUFFER_PUBKEY \
  --multisig $SQUADS_VAULT_PUBKEY \
  --keypair $KEYPAIR \
  --cluster $CLUSTER

# List pending proposals
sqd program-upgrade list --multisig $SQUADS_VAULT_PUBKEY

# Approve (as a signer)
sqd program-upgrade approve \
  --proposal <PROPOSAL_PUBKEY> \
  --keypair $KEYPAIR

# Execute after threshold
sqd program-upgrade execute \
  --proposal <PROPOSAL_PUBKEY> \
  --keypair $KEYPAIR
Option C: Manual Multisig (Realms / Custom)
bash
# Each signer runs:
solana program deploy --buffer $BUFFER_PUBKEY $PROGRAM_ID \
  --keypair $SIGNER_KEYPAIR \
  --sign-only \
  --blockhash <RECENT_BLOCKHASH>

# Collect all partial signatures, then:
solana program deploy --buffer $BUFFER_PUBKEY $PROGRAM_ID \
  --signer <PUBKEY1>=<SIGNATURE1> \
  --signer <PUBKEY2>=<SIGNATURE2> \
  --blockhash <RECENT_BLOCKHASH>
Phase 5: Deploy & Verify
Post-Deploy Verification
bash
# 1. Confirm new program hash
solana program show $PROGRAM_ID --output json | jq '.programData[] | .upgradeAuthority, .lastDeploySlot'

# 2. Verify IDL updated
anchor idl fetch $PROGRAM_ID --provider.cluster $CLUSTER > idl_post.json
diff idl_local.json idl_post.json

# 3. Test critical instruction
anchor test --skip-build --provider.cluster $CLUSTER

# 4. Monitor for 24h
# Set up alerts for:
#   - Transaction error rate spikes
#   - Unusual instruction patterns
#   - Account deserialization failures
Priority Fee Deployment (High Traffic)
bash
# Add priority fee for faster inclusion
solana program deploy --buffer $BUFFER_PUBKEY $PROGRAM_ID \
  --with-compute-unit-price 10000 \
  --max-sign-attempts 10
Phase 6: Rollback
Emergency Rollback to Previous Buffer
bash
# You must have kept the old buffer (Rule #8)
export OLD_BUFFER="OldBufferPubkeyHere"

# Via Squads UI: propose upgrade to OLD_BUFFER
# Or via CLI:
sqd program-upgrade propose \
  --program-id $PROGRAM_ID \
  --buffer $OLD_BUFFER \
  --multisig $SQUADS_VAULT_PUBKEY
Close Failed Buffer
bash
# If new buffer was never deployed, close it
solana program close $BUFFER_PUBKEY --keypair $KEYPAIR
Environment Variables
Quick Reference
Table
Variable	Description	Example
PROGRAM_ID	On-chain program address	Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
BUFFER_PUBKEY	Temporary buffer address	7nY7H...
BUFFER_AUTHORITY	Key that can write/close buffer	Your deployer key
SQUADS_AUTHORITY	Multisig PDA that controls program	8kJ...
SQUADS_VAULT_PUBKEY	Squads vault for proposals	3xK...
CLUSTER	Target cluster	mainnet-beta, devnet
KEYPAIR	Path to your local keypair	~/.config/solana/id.json
Version: 2026.06 | Tested on: Solana CLI 1.18+, Anchor 0.30+, Squads v3
