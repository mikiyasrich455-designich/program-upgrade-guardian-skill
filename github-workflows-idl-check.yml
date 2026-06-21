name: Guardian CI

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

env:
  SOLANA_CLI_VERSION: 1.18.0
  ANCHOR_VERSION: 0.30.0
  NODE_VERSION: 20

jobs:
  # ============================================================================
  # Gate 1: Lint & Format
  # ============================================================================
  lint:
    name: Lint & Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

  # ============================================================================
  # Gate 2: Unit Tests (LiteSVM)
  # ============================================================================
  unit-tests:
    name: Unit Tests (LiteSVM)
    runs-on: ubuntu-latest
    needs: lint
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2

      - name: Install Solana CLI
        run: |
          sh -c "$(curl -sSfL https://release.solana.com/v${SOLANA_CLI_VERSION}/install)"
          echo "$HOME/.local/share/solana/install/active_release/bin" >> $GITHUB_PATH

      - name: Install Anchor
        run: |
          cargo install --git https://github.com/coral-xyz/anchor --tag v${ANCHOR_VERSION} anchor-cli

      - name: Build program
        run: anchor build

      - name: Run unit tests
        run: cargo test --features test-bpf

      - name: Run LiteSVM tests
        run: cargo test --test test_migration_append --test test_migration_resize --test test_borsh_layout

  # ============================================================================
  # Gate 3: IDL Compatibility Check
  # ============================================================================
  idl-check:
    name: IDL Compatibility
    runs-on: ubuntu-latest
    needs: lint
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # Need full history to get old IDL

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: ${{ env.NODE_VERSION }}

      - name: Install dependencies
        run: npm install

      - name: Build new IDL
        run: |
          cargo install --git https://github.com/coral-xyz/anchor --tag v${ANCHOR_VERSION} anchor-cli
          anchor build

      - name: Extract old IDL from main branch
        run: |
          git show origin/main:target/idl/*.json > idl/old.json 2>/dev/null || echo "{}" > idl/old.json

      - name: Run IDL compatibility check
        run: npx ts-node tests/ci/idl-check.ts idl/old.json target/idl/*.json

      - name: Check Borsh layout drift
        run: npx ts-node tests/ci/layout-drift.ts idl/old.json target/idl/*.json

      - name: Upload IDL artifact
        uses: actions/upload-artifact@v4
        with:
          name: idl-${{ github.sha }}
          path: target/idl/*.json

  # ============================================================================
  # Gate 4: Security Audit
  # ============================================================================
  security-audit:
    name: Security Audit
    runs-on: ubuntu-latest
    needs: [unit-tests, idl-check]
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-audit
        run: cargo install cargo-audit

      - name: Run security audit
        run: cargo audit

      - name: Check for unsafe code
        run: |
          if grep -r "unsafe" programs/ --include="*.rs"; then
            echo "⚠️  Unsafe code detected - requires manual review"
            # Don't fail, just warn - some unsafe may be justified
          fi

  # ============================================================================
  # Gate 5: Build Verification
  # ============================================================================
  build-verify:
    name: Build Verification
    runs-on: ubuntu-latest
    needs: [unit-tests, idl-check]
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Solana CLI
        run: |
          sh -c "$(curl -sSfL https://release.solana.com/v${SOLANA_CLI_VERSION}/install)"
          echo "$HOME/.local/share/solana/install/active_release/bin" >> $GITHUB_PATH

      - name: Install Anchor
        run: |
          cargo install --git https://github.com/coral-xyz/anchor --tag v${ANCHOR_VERSION} anchor-cli

      - name: Build release
        run: anchor build --release

      - name: Verify build determinism
        run: |
          sha256sum target/deploy/*.so > build-hash.txt
          anchor build --release
          sha256sum -c build-hash.txt

      - name: Upload build artifact
        uses: actions/upload-artifact@v4
        with:
          name: program-${{ github.sha }}
          path: target/deploy/*.so

  # ============================================================================
  # Gate 6: Risk Assessment (for PRs with state changes)
  # ============================================================================
  risk-assessment:
    name: Risk Assessment
    runs-on: ubuntu-latest
    needs: [unit-tests, idl-check]
    if: github.event_name == 'pull_request'
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Detect state changes
        id: detect
        run: |
          # Check if any account structs changed
          if git diff origin/main -- programs/*/src/state/*.rs | grep -q "^[+-].*pub struct"; then
            echo "state_changes=true" >> $GITHUB_OUTPUT
            echo "⚠️  Account struct changes detected"
          else
            echo "state_changes=false" >> $GITHUB_OUTPUT
          fi

      - name: Require migration review
        if: steps.detect.outputs.state_changes == 'true'
        run: |
          echo "::error::Account struct changes require migration review"
          echo "Please ensure:"
          echo "  1. Migration code exists in templates/migration-template.rs"
          echo "  2. Tests cover the migration path"
          echo "  3. Risk assessment is documented in PR description"
          exit 1

  # ============================================================================
  # Final Gate: All checks passed
  # ============================================================================
  all-gates-passed:
    name: All Gates Passed
    runs-on: ubuntu-latest
    needs: [lint, unit-tests, idl-check, security-audit, build-verify, risk-assessment]
    if: always()
    steps:
      - name: Check all jobs passed
        run: |
          if [ "${{ needs.lint.result }}" != "success" ] || \
             [ "${{ needs.unit-tests.result }}" != "success" ] || \
             [ "${{ needs.idl-check.result }}" != "success" ] || \
             [ "${{ needs.security-audit.result }}" != "success" ] || \
             [ "${{ needs.build-verify.result }}" != "success" ]; then
            echo "::error::One or more CI gates failed"
            exit 1
          fi
          echo "✅ All guardian gates passed - upgrade is safe to proceed"
