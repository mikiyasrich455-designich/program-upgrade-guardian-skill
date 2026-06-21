#!/bin/bash
#
# Program Upgrade Guardian - Phase 1: Discovery
# Real RPC calls to pull program metadata, verify authority, check buffer status
#

set -e

PROGRAM_ID="${1:-}"
RPC_URL="${2:-$(solana config get | grep 'RPC URL' | awk '{print $3}')}"

if [ -z "$PROGRAM_ID" ]; then
    echo "Usage: ./discovery.sh <PROGRAM_ID> [RPC_URL]"
    echo "Example: ./discovery.sh TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    exit 1
fi

echo "══════════════════════════════════════════════════"
echo "  Program Upgrade Guardian - Discovery Phase"
echo "══════════════════════════════════════════════════"
echo ""
echo "Program ID: $PROGRAM_ID"
echo "RPC URL: $RPC_URL"
echo ""

# 1. Pull program account metadata
echo "📋 Step 1: Program Account Metadata"
echo "─────────────────────────────────────────────────"
solana program show "$PROGRAM_ID" --url "$RPC_URL" 2>/dev/null || {
    echo "❌ Failed to fetch program data. Check program ID and RPC."
    exit 1
}
echo ""

# 2. Check upgrade authority (CRITICAL)
echo "🔐 Step 2: Upgrade Authority Verification"
echo "─────────────────────────────────────────────────"
AUTHORITY=$(solana program show "$PROGRAM_ID" --url "$RPC_URL" | grep "Authority:" | awk '{print $2}')
echo "Authority: $AUTHORITY"

if [ -z "$AUTHORITY" ]; then
    echo "❌ No authority found. Program may not be upgradeable."
    exit 1
fi

# Check if authority is a multisig (Squads/Realms)
AUTHORITY_LEN=${#AUTHORITY}
if echo "$AUTHORITY" | grep -q "^squads" || echo "$AUTHORITY" | grep -q "^ realms"; then
    echo "✅ Authority is a multisig (Squads/Realms)"
elif [ "$AUTHORITY_LEN" -eq 44 ]; then
    echo "⚠️  Authority is a single pubkey (hot wallet)"
    echo "   ACTION REQUIRED: Transfer to multisig before mainnet upgrade"
else
    echo "ℹ️  Authority type unknown. Verify manually."
fi
echo ""

# 3. Fetch deployed IDL (if Anchor program)
echo "📄 Step 3: Anchor IDL Fetch"
echo "─────────────────────────────────────────────────"
if command -v anchor >/dev/null 2>&1; then
    anchor idl fetch "$PROGRAM_ID" --url "$RPC_URL" 2>/dev/null > "idl-${PROGRAM_ID}.json" && {
        echo "✅ IDL saved to idl-${PROGRAM_ID}.json"
        echo "   Accounts: $(jq '.accounts | length' "idl-${PROGRAM_ID}.json" 2>/dev/null || echo 'N/A')"
        echo "   Instructions: $(jq '.instructions | length' "idl-${PROGRAM_ID}.json" 2>/dev/null || echo 'N/A')"
    } || {
        echo "⚠️  Not an Anchor program or IDL not available"
    }
else
    echo "⚠️  Anchor CLI not found. Install: avm install latest"
fi
echo ""

# 4. Check program buffer status
echo "💾 Step 4: Buffer Status"
echo "─────────────────────────────────────────────────"
BUFFER_COUNT=$(solana program show --buffers --url "$RPC_URL" 2>/dev/null | grep -c "Buffer:" || echo "0")
echo "Active buffers: $BUFFER_COUNT"

if [ "$BUFFER_COUNT" -gt 0 ]; then
    echo ""
    echo "Buffer details:"
    solana program show --buffers --url "$RPC_URL" 2>/dev/null | head -20
fi
echo ""

# 5. List program accounts (sample)
echo "📊 Step 5: Program Accounts (Sample)"
echo "─────────────────────────────────────────────────"
ACCOUNT_COUNT=$(solana program show "$PROGRAM_ID" --url "$RPC_URL" | grep "Data Length:" | wc -l)
echo "Program data accounts: $ACCOUNT_COUNT"

# Get largest accounts for the program
echo ""
echo "Largest accounts:"
solana program show "$PROGRAM_ID" --url "$RPC_URL" --programs 2>/dev/null | head -10 || {
    echo "ℹ️  Use 'solana program show --accounts' for full list"
}
echo ""

# 6. Check current deployment slot
echo "⏱️  Step 6: Deployment Info"
echo "─────────────────────────────────────────────────"
solana program show "$PROGRAM_ID" --url "$RPC_URL" | grep -E "(Last Deployed|Data Length|Balance|Executable)" || true
echo ""

# 7. Save discovery report
echo "💾 Step 7: Saving Discovery Report"
echo "─────────────────────────────────────────────────"
REPORT_FILE="discovery-report-${PROGRAM_ID:0:8}.md"
cat > "$REPORT_FILE" << EOF
# Discovery Report: ${PROGRAM_ID}

**Date:** $(date -u +"%Y-%m-%d %H:%M UTC")
**RPC:** ${RPC_URL}
**Authority:** ${AUTHORITY}

## Authority Status
- Type: $(if echo "$AUTHORITY" | grep -q "^squads"; then echo "Multisig (Squads)"; elif [ ${#AUTHORITY} -eq 44 ]; then echo "Single Key (Hot Wallet)"; else echo "Unknown"; fi)
- Safe for mainnet: $(if echo "$AUTHORITY" | grep -q "^squads"; then echo "✅ YES"; else echo "❌ NO - Transfer to multisig"; fi)

## Program Stats
- Data accounts: ${ACCOUNT_COUNT}
- Active buffers: ${BUFFER_COUNT}

## Next Steps
1. $(if echo "$AUTHORITY" | grep -q "^squads"; then echo "Authority is multisig. Proceed to Analysis phase."; else echo "Transfer authority to Squads multisig before proceeding."; fi)
2. Run Borsh layout drift check
3. Execute Surfpool fork test

## Exit Criteria
- [x] Authority confirmed
- [x] Program ID valid
- [ ] $(if echo "$AUTHORITY" | grep -q "^squads"; then echo "Multisig verified"; else echo "Multisig transfer required"; fi)
EOF

echo "✅ Report saved: $REPORT_FILE"
echo ""

echo "══════════════════════════════════════════════════"
echo "  Discovery Complete"
echo "══════════════════════════════════════════════════"
echo ""
echo "Next: Run 'anchor build --verifiable' and proceed to Analysis phase."
