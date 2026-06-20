#!/bin/bash
set -euo pipefail

# ═══════════════════════════════════════════════════════════════════════
# Program Upgrade Guardian — Setup Checker
# ═══════════════════════════════════════════════════════════════════════

RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
NC="\033[0m" # No Color

PASS="${GREEN}✓${NC}"
FAIL="${RED}✗${NC}"
WARN="${YELLOW}⚠${NC}"

check_command() {
    local cmd="$1"
    local min_version="$2"
    local install_hint="$3"

    if command -v "$cmd" &> /dev/null; then
        local version
        version=$($cmd --version 2>/dev/null | head -n1 || echo "unknown")
        echo -e "${PASS} ${cmd}: ${version}"
        return 0
    else
        echo -e "${FAIL} ${cmd}: NOT FOUND"
        echo -e "   Install: ${install_hint}"
        return 1
    fi
}

echo ""
echo "═══════════════════════════════════════════════════════"
echo "  Program Upgrade Guardian — Setup Checker"
echo "═══════════════════════════════════════════════════════"
echo ""

MISSING=0

# ── Check core tools ──
echo "Checking core tools..."
check_command "anchor" "0.30.0" "https://www.anchor-lang.com/docs/installation" || ((MISSING++))
check_command "solana" "1.18.0" "https://docs.solanalabs.com/cli/install" || ((MISSING++))
check_command "sqd" "latest" "npm install -g @sqds/cli" || echo -e "${WARN} sqd: OPTIONAL (Squads CLI for multisig)"
check_command "node" "20.0.0" "https://nodejs.org/" || ((MISSING++))
check_command "rustc" "1.75.0" "https://rustup.rs/" || ((MISSING++))

echo ""

# ── Check Solana config ──
echo "Checking Solana configuration..."
if solana config get &> /dev/null; then
    CLUSTER=$(solana config get | grep "RPC URL" | awk '{print $3}' || echo "unknown")
    KEYPAIR=$(solana config get | grep "Keypair Path" | awk '{print $3}' || echo "unknown")
    echo -e "${PASS} Solana cluster: ${CLUSTER}"
    echo -e "${PASS} Solana keypair: ${KEYPAIR}"
else
    echo -e "${FAIL} Solana not configured. Run: solana config set --url mainnet-beta"
    ((MISSING++))
fi

echo ""

# ── Check environment variables ──
echo "Checking environment variables..."
if [[ -n "${PROGRAM_ID:-}" ]]; then
    echo -e "${PASS} PROGRAM_ID: ${PROGRAM_ID}"
else
    echo -e "${WARN} PROGRAM_ID: not set (set before upgrades)"
fi

if [[ -n "${SQUADS_AUTHORITY:-}" ]]; then
    echo -e "${PASS} SQUADS_AUTHORITY: ${SQUADS_AUTHORITY}"
else
    echo -e "${WARN} SQUADS_AUTHORITY: not set (set before mainnet upgrades)"
fi

echo ""

# ── Summary ──
echo "═══════════════════════════════════════════════════════"
if [[ $MISSING -eq 0 ]]; then
    echo -e "  ${GREEN}All required tools installed.${NC}"
    echo -e "  ${GREEN}You are ready to use the Guardian Skill.${NC}"
    echo "═══════════════════════════════════════════════════════"
    echo ""
    echo "Next steps:"
    echo "  1. Review SKILL.md for the full Guardian Pipeline"
    echo "  2. Set your environment variables in .env"
    echo "  3. Run: source .env && ./install.sh"
    echo ""
    exit 0
else
    echo -e "  ${RED}${MISSING} required tool(s) missing.${NC}"
    echo -e "  ${RED}Install missing tools before using this skill.${NC}"
    echo "═══════════════════════════════════════════════════════"
    echo ""
    exit 1
fi
