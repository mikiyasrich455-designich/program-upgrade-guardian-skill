#!/bin/bash
# Devnet Verification Script
# Post-deployment verification checks

set -euo pipefail

PROGRAM_ID="${1:-}"
if [ -z "$PROGRAM_ID" ]; then
    echo "Usage: ./verify-devnet.sh <PROGRAM_ID>"
    exit 1
fi

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "═══════════════════════════════════════"
echo "  Devnet Verification"
echo "═══════════════════════════════════════"
echo ""

# 1. Check program exists
echo -n "[1/5] Checking program exists... "
if solana program show "$PROGRAM_ID" --url devnet > /dev/null 2>&1; then
    echo -e "${GREEN}OK${NC}"
else
    echo -e "${RED}FAIL${NC}"
    echo "  Program not found on devnet"
    exit 1
fi

# 2. Verify hash
echo -n "[2/5] Verifying program hash... "
solana program dump "$PROGRAM_ID" /tmp/devnet-verify.so --url devnet > /dev/null 2>&1

if [ -f "target/deploy/*.so" ]; then
    LOCAL_HASH=$(sha256sum target/deploy/*.so | awk '{print $1}')
    DEVNET_HASH=$(sha256sum /tmp/devnet-verify.so | awk '{print $1}')

    if [ "$LOCAL_HASH" = "$DEVNET_HASH" ]; then
        echo -e "${GREEN}MATCH${NC}"
    else
        echo -e "${RED}MISMATCH${NC}"
        echo "  Local: $LOCAL_HASH"
        echo "  Devnet: $DEVNET_HASH"
        echo "  WARNING: Deployed binary differs from local build!"
    fi
else
    echo -e "${YELLOW}SKIP${NC} (no local .so found)"
fi

# 3. Check authority
echo -n "[3/5] Checking authority... "
AUTHORITY=$(solana program show "$PROGRAM_ID" --url devnet | grep "Authority" | awk '{print $2}')
echo "$AUTHORITY"

# 4. Check balance
echo -n "[4/5] Checking program balance... "
BALANCE=$(solana program show "$PROGRAM_ID" --url devnet | grep "Balance" | awk '{print $2}')
echo -e "${GREEN}$BALANCE${NC}"

# 5. Check data length
echo -n "[5/5] Checking program size... "
DATA_LEN=$(solana program show "$PROGRAM_ID" --url devnet | grep "Data Length" | awk '{print $3}')
echo -e "${GREEN}$DATA_LEN bytes${NC}"

echo ""
echo "═══════════════════════════════════════"
echo -e "  ${GREEN}Verification complete${NC}"
echo "═══════════════════════════════════════"
