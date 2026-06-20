#!/bin/bash
# ═══════════════════════════════════════════════════════════════════
# Program Upgrade Guardian — Repo Cleanup Script
# Run this from your repo root, then push
# ═══════════════════════════════════════════════════════════════════

set -euo pipefail

echo ""
echo "═══════════════════════════════════════════════════════"
echo "  Cleaning up repo structure..."
echo "═══════════════════════════════════════════════════════"
echo ""

# 1. Move CI workflow to correct location
echo "[1/5] Moving idl-check.yml to .github/workflows/..."
mkdir -p .github/workflows
if [ -f "idl-check.yml" ]; then
    mv idl-check.yml .github/workflows/idl-check.yml
    echo "  ✅ Moved idl-check.yml → .github/workflows/"
else
    echo "  ⚠️  idl-check.yml not found in root (already moved?)"
fi

# 2. Move test suite to tests/
echo ""
echo "[2/5] Moving upgrade-test-suite.md to tests/..."
mkdir -p tests
if [ -f "upgrade-test-suite.md" ]; then
    mv upgrade-test-suite.md tests/upgrade-test-suite.md
    echo "  ✅ Moved upgrade-test-suite.md → tests/"
else
    echo "  ⚠️  upgrade-test-suite.md not found in root (already moved?)"
fi

# 3. Move IDL checker to scripts/ with correct name
echo ""
echo "[3/5] Moving idl-check (2).ts to scripts/idl-check.ts..."
mkdir -p scripts
if [ -f "idl-check (2).ts" ]; then
    mv "idl-check (2).ts" scripts/idl-check.ts
    echo "  ✅ Moved idl-check (2).ts → scripts/idl-check.ts"
else
    echo "  ⚠️  idl-check (2).ts not found in root (already moved?)"
fi

# 4. Fix env filename (add dot prefix)
echo ""
echo "[4/5] Renaming env.example → .env.example..."
if [ -f "env.example" ]; then
    mv env.example .env.example
    echo "  ✅ Renamed env.example → .env.example"
else
    echo "  ⚠️  env.example not found (already renamed?)"
fi

# 5. Fix SKILL.md buffer placeholders
echo ""
echo "[5/5] Fixing SKILL.md buffer workflow placeholders..."
if [ -f "SKILL.md" ]; then
    # Use sed to fix the broken placeholders
    sed -i '' 's/--buffer-authority$/--buffer-authority $BUFFER_AUTHORITY/g' SKILL.md 2>/dev/null ||     sed -i 's/--buffer-authority$/--buffer-authority $BUFFER_AUTHORITY/g' SKILL.md

    sed -i '' 's/solana program verify  target/solana program verify $BUFFER_PUBKEY target/g' SKILL.md 2>/dev/null ||     sed -i 's/solana program verify  target/solana program verify $BUFFER_PUBKEY target/g' SKILL.md

    sed -i '' 's/solana program deploy --buffer$/solana program deploy --buffer $BUFFER_PUBKEY $PROGRAM_ID/g' SKILL.md 2>/dev/null ||     sed -i 's/solana program deploy --buffer$/solana program deploy --buffer $BUFFER_PUBKEY $PROGRAM_ID/g' SKILL.md

    echo "  ✅ Fixed buffer workflow placeholders in SKILL.md"
else
    echo "  ⚠️  SKILL.md not found"
fi

echo ""
echo "═══════════════════════════════════════════════════════"
echo "  Cleanup complete!"
echo "═══════════════════════════════════════════════════════"
echo ""
echo "Verify the changes:"
echo "  ls -la .github/workflows/"
echo "  ls -la tests/"
echo "  ls -la scripts/"
echo "  ls -la .env.example"
echo ""
echo "Then commit and push:"
echo "  git add -A"
echo "  git commit -m 'fix: organize files into correct directories'"
echo "  git push origin main"
echo ""
