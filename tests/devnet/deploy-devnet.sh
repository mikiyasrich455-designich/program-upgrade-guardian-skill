#!/bin/bash
# Devnet Rehearsal Script
# Full deployment and test script for devnet

set -euo pipefail

PROGRAM_ID="${1:-}"
if [ -z "$PROGRAM_ID" ]; then
    echo "Usage: ./deploy-devnet.sh <PROGRAM_ID>"
    echo "Example: ./deploy-devnet.sh 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"
    exit 1
fi

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "═══════════════════════════════════════"
echo "  Devnet Rehearsal - Guardian Skill"
echo "═══════════════════════════════════════"
echo ""

# Verify not on mainnet
echo -n "Checking cluster... "
RPC_URL=$(solana config get | grep "RPC URL" | awk '{print $3}')
if [[ "$RPC_URL" == *"mainnet"* ]]; then
    echo -e "${RED}FAIL${NC}"
    echo "  You are on MAINNET. Switch to devnet first:"
    echo "  solana config set --url https://api.devnet.solana.com"
    exit 1
fi
echo -e "${GREEN}OK${NC} ($RPC_URL)"

# Check balance
echo -n "Checking balance... "
BALANCE=$(solana balance 2>/dev/null || echo "0")
if (( $(echo "$BALANCE < 2" | bc -l 2>/dev/null || echo "0") )); then
    echo -e "${YELLOW}LOW${NC} ($BALANCE SOL)"
    echo "  Requesting airdrop..."
    solana airdrop 2
else
    echo -e "${GREEN}OK${NC} ($BALANCE SOL)"
fi

# 1. Build
echo ""
echo "[1/5] Building program..."
anchor build

# Verify build
if [ ! -f "target/deploy/*.so" ]; then
    echo -e "${RED}Build failed - no .so file found${NC}"
    exit 1
fi
echo -e "${GREEN}Build successful${NC}"

# 2. Deploy to devnet
echo ""
echo "[2/5] Deploying to devnet..."
anchor deploy --provider.cluster devnet

# 3. Run tests
echo ""
echo "[3/5] Running tests..."
if ! anchor test --provider.cluster devnet --skip-build; then
    echo -e "${RED}Tests failed on devnet${NC}"
    echo "  Fix issues before mainnet deployment."
    exit 1
fi
echo -e "${GREEN}All tests passed${NC}"

# 4. Verify authority
echo ""
echo "[4/5] Verifying authority..."
AUTHORITY=$(solana program show "$PROGRAM_ID" --url devnet | grep "Authority" | awk '{print $2}')
echo "  Program authority: $AUTHORITY"

# 5. Test migration if applicable
echo ""
echo "[5/5] Checking for migration tests..."
if [ -f "tests/devnet/test-migration.ts" ]; then
    echo "  Running migration tests..."
    npx ts-node tests/devnet/test-migration.ts
    echo -e "${GREEN}Migration tests passed${NC}"
else
    echo -e "${YELLOW}No migration tests found${NC} (skipping)"
fi

echo ""
echo "═══════════════════════════════════════"
echo -e "  ${GREEN}Devnet rehearsal complete!${NC}"
echo "═══════════════════════════════════════"
echo ""
echo "Next steps:"
echo "  1. Review test results above"
echo "  2. Run: ./tests/devnet/verify-devnet.sh $PROGRAM_ID"
echo "  3. If all good, proceed to mainnet upgrade"
