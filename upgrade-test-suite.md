---
name: upgrade-test-suite
parent-skill: program-upgrade-guardian
description: >
  Complete testing strategy for Solana program upgrades — LiteSVM unit tests,
  Surfpool fork tests, devnet rehearsals, mainnet shadow testing, post-upgrade
  monitoring, and automated alerting. Every test you need, ready to run.
---

# Upgrade Test Suite

> Every test you need to run before, during, and after a program upgrade.
> No test, no deploy. No exception.

---

## Table of Contents

- [Test Pyramid](#test-pyramid)
- [Phase 1: LiteSVM Unit Tests](#phase-1-litesvm-unit-tests)
- [Phase 2: Surfpool Fork Tests](#phase-2-surfpool-fork-tests)
- [Phase 3: Devnet Rehearsal](#phase-3-devnet-rehearsal)
- [Phase 4: Mainnet Shadow Testing](#phase-4-mainnet-shadow-testing)
- [Phase 5: Post-Upgrade Monitoring](#phase-5-post-upgrade-monitoring)
- [Test Automation](#test-automation)
- [Master Checklist](#master-checklist)

---

## Test Pyramid

```
         ┌─────────────────┐
         │   Mainnet Live  │  ← Shadow + 24h Monitor (real users, real money)
         │    Monitoring   │
         ├─────────────────┤
         │   Devnet Full   │  ← End-to-End Rehearsal (multisig + buffer + migration)
         │    Rehearsal    │
         ├─────────────────┤
         │  Surfpool Fork  │  ← Real State Simulation (mainnet accounts, local execution)
         │    Test         │
         ├─────────────────┤
         │   LiteSVM Unit  │  ← Instruction Logic (sub-second, deterministic)
         │    Simulation   │
         └─────────────────┘
```

| Layer | Speed | Fidelity | Cost | When to Run |
|-------|-------|----------|------|-------------|
| LiteSVM | <1s | Low | Free | Every commit, CI |
| Surfpool | 30-120s | High | Free | Every PR affecting accounts |
| Devnet | 2-5min | Medium | ~0.01 SOL | Before every mainnet upgrade |
| Mainnet Shadow | Real-time | Exact | RPC only | Hours before upgrade |
| Post-Upgrade | Ongoing | Exact | Free | 24-72h after upgrade |

---

## Phase 1: LiteSVM Unit Tests

### Setup

```toml
# Cargo.toml [dev-dependencies]
[dev-dependencies]
litesvm = "0.6"
solana-sdk = "~1.18"
solana-program = "~1.18"
anchor-lang = "0.30.0"
```

### Test 1: Lazy Migration v1 -> v2

```rust
#[cfg(test)]
mod lazy_migration_tests {
    use litesvm::LiteSVM;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        transaction::Transaction,
    };
    use std::path::PathBuf;

    fn deploy_program(svm: &mut LiteSVM, so_path: &str) -> Pubkey {
        let program_id = Pubkey::new_unique();
        let so_data = std::fs::read(so_path).expect("Program .so not found");
        svm.add_program(program_id, &so_data);
        program_id
    }

    fn create_account(owner: &Pubkey, data: Vec<u8>, lamports: u64) -> solana_sdk::account::Account {
        solana_sdk::account::Account {
            lamports,
            data,
            owner: *owner,
            executable: false,
            rent_epoch: 0,
        }
    }

    #[test]
    fn test_lazy_migration_v1_to_v2_success() {
        let mut svm = LiteSVM::new();

        // Fund payer
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

        // Deploy program
        let program_id = deploy_program(&mut svm, "target/deploy/my_program.so");

        // Create user and derive PDA
        let user = Keypair::new();
        let (pda, _bump) = Pubkey::find_program_address(
            &[b"user_profile", user.pubkey().as_ref()],
            &program_id,
        );

        // ═══════════════════════════════════════════════════════
        // Manually construct v1 account data (simulates old mainnet state)
        // Layout: [disc:8][owner:32][created_at:8][name_len:4][name:5]
        // Total: 8 + 32 + 8 + 4 + 5 = 57 bytes
        // ═══════════════════════════════════════════════════════
        let name = b"alice";
        let mut v1_data = vec![0u8; 57];

        // Anchor discriminator (first 8 bytes)
        v1_data[0..8].copy_from_slice(&[
            0x8a, 0x1a, 0x7c, 0x2e, 0x3f, 0x4d, 0x5e, 0x6f,
        ]);

        // owner: Pubkey (bytes 8..40)
        v1_data[8..40].copy_from_slice(&user.pubkey().to_bytes());

        // created_at: i64 (bytes 40..48)
        v1_data[40..48].copy_from_slice(&i64::to_le_bytes(1_700_000_000));

        // name: String length prefix u32 (bytes 48..52)
        v1_data[48..52].copy_from_slice(&u32::to_le_bytes(name.len() as u32));

        // name: String bytes (bytes 52..57)
        v1_data[52..57].copy_from_slice(name);

        // Set account on-chain
        svm.set_account(
            pda,
            create_account(&program_id, v1_data, 2_039_280),
        );

        // ═══════════════════════════════════════════════════════
        // Call update_profile — should trigger lazy migration
        // ═══════════════════════════════════════════════════════
        let ix_data = {
            let mut data = vec![0u8; 8 + 4 + 5];
            data[0..8].copy_from_slice(&[
                0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x7a, 0x8b,
            ]);
            data[8..12].copy_from_slice(&u32::to_le_bytes(5u32));
            data[12..17].copy_from_slice(b"alice");
            data
        };

        let ix = Instruction::new_with_bytes(
            program_id,
            &ix_data,
            vec![
                AccountMeta::new(pda, false),
                AccountMeta::new_readonly(user.pubkey(), true),
            ],
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[&payer, &user],
            svm.latest_blockhash(),
        );

        svm.send_transaction(tx).expect("Lazy migration transaction failed");

        // ═══════════════════════════════════════════════════════
        // Verify v2 state
        // ═══════════════════════════════════════════════════════
        let account = svm.get_account(&pda).expect("Account missing after migration");
        let data = account.data.borrow();

        assert!(
            data.len() >= 67,
            "Account did not grow: expected >= 67, got {}",
            data.len()
        );

        assert_eq!(data[data.len() - 1], 2, "Version not updated to v2");
        assert_eq!(data[57], 0, "Bio discriminant should be None (0)");

        let reputation = u64::from_le_bytes(data[58..66].try_into().unwrap());
        assert_eq!(reputation, 0, "Reputation score should default to 0");

        msg!("✅ Lazy migration v1 -> v2 passed");
    }

    #[test]
    fn test_lazy_migration_idempotent() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

        let program_id = deploy_program(&mut svm, "target/deploy/my_program.so");
        let user = Keypair::new();
        let (pda, _) = Pubkey::find_program_address(
            &[b"user_profile", user.pubkey().as_ref()],
            &program_id,
        );

        // Setup v1 account and call update_profile twice
        // Assert version stays 2, no double-migration side effects
    }
}
```

### Test 2: Realloc Size Exactness

```rust
#[cfg(test)]
mod realloc_tests {
    use super::*;

    #[test]
    fn test_size_empty_name_no_bio() {
        let size = UserProfile::size("", None);
        assert_eq!(size, 62); // 8 + 32 + 8 + 4 + 0 + 1 + 0 + 8 + 1
    }

    #[test]
    fn test_size_with_name_no_bio() {
        let size = UserProfile::size("alice", None);
        assert_eq!(size, 67); // 8 + 32 + 8 + 4 + 5 + 1 + 0 + 8 + 1
    }

    #[test]
    fn test_size_with_name_and_bio() {
        let bio = "hello world".to_string();
        let size = UserProfile::size("alice", Some(&bio));
        assert_eq!(size, 82); // 8 + 32 + 8 + 4 + 5 + 1 + 4 + 11 + 8 + 1
    }

    #[test]
    fn test_size_maximum() {
        let max_name = "x".repeat(32);
        let max_bio = "x".repeat(256);
        let size = UserProfile::size(&max_name, Some(&max_bio));
        assert!(size <= 10_240, "Max size {} exceeds Solana limit", size);
    }
}
```

### LiteSVM Checklist

- [ ] Lazy migration v1 -> v2 succeeds and produces correct state
- [ ] Lazy migration is idempotent (second call is no-op)
- [ ] Realloc size math is exact for all field combinations
- [ ] Max-size account fits within 10,240 bytes
- [ ] Old data is not corrupted after realloc
- [ ] Batch migration rejects non-admin signer
- [ ] Batch migration enforces size cap
- [ ] All tests run in <5 seconds total

---

## Phase 2: Surfpool Fork Tests

### Setup

```bash
npm install -g @surfpool/cli
npm install @surfpool/sdk
```

### Fork Test: Full Upgrade Simulation

```typescript
// tests/surfpool-fork.test.ts
import { Surfpool } from "@surfpool/sdk";
import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { MyProgram } from "../target/types/my_program";

const FORK_STATE = "./test-fixtures/fork-state.json";
const PROGRAM_ID = new PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
const NEW_PROGRAM_SO = "./target/deploy/my_program.so";

describe("Surfpool Fork Test: Program Upgrade", () => {
    let surfpool: Surfpool;
    let connection: Connection;
    let program: Program<MyProgram>;

    beforeAll(async () => {
        surfpool = await Surfpool.fromFile(FORK_STATE);
        connection = surfpool.getConnection();
        const wallet = surfpool.getPayer();
        const provider = new AnchorProvider(connection, wallet, { commitment: "confirmed" });
        await surfpool.deployProgram(NEW_PROGRAM_SO, PROGRAM_ID);
        program = new Program<MyProgram>(require("../target/idl/my_program.json"), provider);
    }, 120_000);

    test("all existing accounts deserialize without error", async () => {
        const accounts = await connection.getProgramAccounts(PROGRAM_ID);
        const failures: string[] = [];
        for (const { pubkey } of accounts) {
            try { await program.account.userProfile.fetch(pubkey); }
            catch (e: any) { failures.push(`${pubkey.toBase58()}: ${e.message}`); }
        }
        expect(failures).toHaveLength(0);
    }, 60_000);

    test("lazy migration succeeds on diverse account sizes", async () => {
        const accounts = await connection.getProgramAccounts(PROGRAM_ID);
        const sorted = accounts.sort((a, b) => a.account.data.length - b.account.data.length);
        const testAccounts = [sorted[0], sorted[Math.floor(sorted.length / 2)], sorted[sorted.length - 1]];
        for (const { pubkey } of testAccounts) {
            const tx = await program.methods.updateProfile("test_migration")
                .accounts({ userProfile: pubkey, owner: surfpool.getPayer().publicKey })
                .simulate();
            expect(tx.value.err).toBeNull();
        }
    }, 60_000);
});
```

### Surfpool Checklist

- [ ] Fork from slot within 100 of current
- [ ] Test with >=50 real accounts (diverse sizes/versions)
- [ ] Largest account migrates without `AccountTooLarge`
- [ ] Every IDL instruction simulates without error
- [ ] Rent exemption verified post-realloc
- [ ] Authority unchanged after simulated upgrade
- [ ] Test run twice: clean state + previously-migrated state

---

## Phase 3: Devnet Rehearsal

### Full Deployment Dry Run

```bash
#!/bin/bash
# scripts/devnet-rehearsal.sh
set -euo pipefail

PROGRAM_ID="Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
CLUSTER="devnet"
KEYPAIR="~/.config/solana/id.json"

echo "═══════════════════════════════════════════════════"
echo "  Devnet Rehearsal"
echo "═══════════════════════════════════════════════════"

echo "[1/8] Building..."
anchor build --verifiable

echo "[2/8] Deploying to devnet..."
anchor deploy --provider.cluster $CLUSTER

echo "[3/8] Writing buffer..."
BUFFER_PUBKEY=$(solana program write-buffer target/deploy/my_program.so \
  --buffer-authority $KEYPAIR --url $CLUSTER | grep "Buffer:" | awk '{print $2}')
echo "Buffer: $BUFFER_PUBKEY"

echo "[4/8] Verifying buffer..."
solana program dump $BUFFER_PUBKEY /tmp/buffer.so --url $CLUSTER
sha256sum /tmp/buffer.so target/deploy/my_program.so

echo "[5/8] Squads flow (devnet)..."
echo "   → https://devnet.squads.so"
read -p "Press Enter after Squads proposal executed..."

echo "[6/8] Integration tests..."
anchor test --provider.cluster $CLUSTER --skip-build

echo "[7/8] IDL match..."
anchor idl fetch $PROGRAM_ID --provider.cluster $CLUSTER > /tmp/idl_devnet.json
diff target/idl/my_program.json /tmp/idl_devnet.json

echo "[8/8] Rollback test..."
read -p "Press Enter after rollback test complete..."

echo "✅ Devnet rehearsal complete."
```

### Devnet Checklist

- [ ] Buffer workflow end-to-end
- [ ] Multisig proposal created, signed, executed
- [ ] All integration tests pass
- [ ] IDL byte-for-byte match
- [ ] Migration instructions tested
- [ ] Rollback tested and documented

---

## Phase 4: Mainnet Shadow Testing

```typescript
// scripts/shadow-test.ts
import { Connection, PublicKey } from "@solana/web3.js";
import { BorshAccountsCoder } from "@coral-xyz/anchor";
import idl from "../target/idl/my_program.json";

const HELIUS_RPC = process.env.HELIUS_RPC_URL!;
const PROGRAM_ID = new PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
const SAMPLE_SIZE = 100;

async function shadowTest() {
    const conn = new Connection(HELIUS_RPC, "confirmed");
    const coder = new BorshAccountsCoder(idl as any);
    const accounts = await conn.getProgramAccounts(PROGRAM_ID, { limit: SAMPLE_SIZE });

    let success = 0, failures: Array<{pubkey: string; error: string}> = [];
    for (const { pubkey, account } of accounts) {
        try { coder.decode("userProfile", account.data); success++; }
        catch (e: any) { failures.push({ pubkey: pubkey.toBase58(), error: e.message }); }
    }

    console.log(`\n✅ ${success} / ${accounts.length} accounts decoded`);
    if (failures.length > 0) {
        console.log("❌ Failures:", failures.slice(0, 10));
        process.exit(1);
    }
}
shadowTest();
```

---

## Phase 5: Post-Upgrade Monitoring

### 24-Hour Health Dashboard

| Metric | Alert Threshold | Check Frequency |
|--------|----------------|-----------------|
| TX error rate | >1% | Every 5 min |
| Account deserialization failures | >0 | Every 5 min |
| Instruction latency p99 | >2x baseline | Every 1 min |
| Unexpected program closes | >0 | Every 5 min |
| Authority changes | Any | Every 1 min |

### Automated Monitor Script

```bash
#!/bin/bash
# scripts/monitor.sh — run as cron every 5 minutes
set -euo pipefail

PROGRAM_ID="${PROGRAM_ID:-Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS}"
EXPECTED_AUTHORITY="${EXPECTED_AUTHORITY:-}"
WEBHOOK_URL="${WEBHOOK_URL:-}"
RPC_URL="${HELIUS_RPC_URL:-https://api.mainnet-beta.solana.com}"

send_alert() {
    local msg="$1"
    echo "[ALERT] $msg"
    [[ -n "$WEBHOOK_URL" ]] && curl -s -X POST -H "Content-type: application/json" \
        --data "{"text":"🚨 $msg"}" "$WEBHOOK_URL" > /dev/null || true
}

# Check authority
AUTHORITY=$(solana program show "$PROGRAM_ID" --url "$RPC_URL" 2>/dev/null | grep "Authority" | awk '{print $2}' || echo "UNKNOWN")
if [[ -n "$EXPECTED_AUTHORITY" && "$AUTHORITY" != "$EXPECTED_AUTHORITY" ]]; then
    send_alert "Authority changed! Current: $AUTHORITY"
fi

# Check errors
ERROR_COUNT=$(solana transaction-history "$PROGRAM_ID" --limit 100 --url "$RPC_URL" 2>/dev/null | grep -c "Error" || echo "0")
if [[ "$ERROR_COUNT" -gt 5 ]]; then
    send_alert "$ERROR_COUNT errors in last 100 txs"
fi

echo "[$(date -Iseconds)] Check complete — authority: $AUTHORITY, errors: $ERROR_COUNT"
```

---

## Test Automation

### GitHub Actions Pipeline

```yaml
# .github/workflows/upgrade-tests.yml
name: Upgrade Test Suite

on:
  push:
    branches: [main]
  pull_request:
    paths:
      - "programs/**"
      - "tests/**"
      - "Cargo.toml"
      - "Anchor.toml"

jobs:
  litesvm:
    name: LiteSVM Unit Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo test --lib --features test-bpf

  idl-check:
    name: IDL Compatibility
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: "20"
      - run: npm ci
      - run: anchor build
      - run: anchor idl fetch $PROGRAM_ID --provider.cluster mainnet > idl/onchain.json
      - run: npx ts-node scripts/idl-check.ts

  surfpool:
    name: Surfpool Fork Test
    runs-on: ubuntu-latest
    needs: [litesvm, idl-check]
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: "20"
      - run: npm ci
      - run: npm run test:surfpool
    timeout-minutes: 10
```

---

## Master Checklist

Before any mainnet upgrade:

- [ ] **LiteSVM:** All unit tests pass
- [ ] **IDL Check:** CI confirms append-only changes
- [ ] **Surfpool:** Fork test passes with >=50 real accounts
- [ ] **Devnet:** Full rehearsal complete
- [ ] **Shadow:** New schema deserializes 100+ mainnet accounts
- [ ] **Authority:** Verified as Squads/Realms multisig
- [ ] **Rollback:** Old buffer kept, rollback tx documented
- [ ] **Monitor:** Post-upgrade alerts configured

**If any item is unchecked, the upgrade is BLOCKED.**

---

*Version: 2026.06 | Test everything. Trust nothing. Deploy once.*
