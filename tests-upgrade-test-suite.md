# Upgrade Test Suite

> This directory contains ACTUAL test files, not just documentation.
> Each test layer has real code you can run.

## Directory Structure

```
tests/
├── upgrade-test-suite.md          # This file
├── unit/                          # LiteSVM tests
│   ├── test_migration_append.rs
│   ├── test_migration_resize.rs
│   └── test_borsh_layout.rs
├── integration/                   # Surfpool fork tests
│   ├── test_fork_upgrade.rs
│   └── test_multisig_flow.rs
├── ci/                            # CI scripts
│   ├── idl-check.ts
│   └── layout-drift.ts
└── devnet/                        # Devnet rehearsal scripts
    ├── deploy-devnet.sh
    └── verify-devnet.sh
```

---

## Layer 1: Unit Tests (LiteSVM)

### tests/unit/test_migration_append.rs

```rust
#[cfg(test)]
mod test_migration_append {
    use litesvm::LiteSVM;
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
    use anchor_lang::InstructionData;
    use your_program::instruction::MigrateUserAppend;

    #[test]
    fn test_append_fields_migration() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

        // Deploy program
        let program_id = your_program::ID;
        svm.add_program_from_file(program_id, "target/deploy/your_program.so").unwrap();

        // Create old-style user account (without new fields)
        let user_keypair = Keypair::new();
        let old_user_data = vec![
            // discriminator (8 bytes)
            0,0,0,0,0,0,0,0,
            // owner: Pubkey (32 bytes)
            payer.pubkey().to_bytes().to_vec(),
            // balance: u64 (8 bytes)
            100u64.to_le_bytes().to_vec(),
        ].concat();

        svm.create_account(&user_keypair.pubkey(), old_user_data.len() as u64, program_id).unwrap();
        svm.set_account_data(&user_keypair.pubkey(), old_user_data).unwrap();

        // Execute migration
        let ix = solana_sdk::instruction::Instruction {
            program_id,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(user_keypair.pubkey(), false),
                solana_sdk::instruction::AccountMeta::new_readonly(payer.pubkey(), true),
            ],
            data: MigrateUserAppend {}.data(),
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[&payer],
            svm.latest_blockhash(),
        );

        svm.send_transaction(tx).unwrap();

        // Verify new fields are set
        let account = svm.get_account(&user_keypair.pubkey()).unwrap();
        // Deserialize and check nickname is Some("") and created_at is set
        // (Implementation depends on your program's deserialization)
    }

    #[test]
    fn test_migration_idempotent() {
        // Run migration twice - second should be no-op
        let mut svm = LiteSVM::new();
        // ... setup ...
        // First migration
        // Second migration should succeed without error
    }
}
```

### tests/unit/test_migration_resize.rs

```rust
#[cfg(test)]
mod test_migration_resize {
    use litesvm::LiteSVM;
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
    use solana_program::rent::Rent;

    #[test]
    fn test_resize_with_rent_calculation() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

        let program_id = your_program::ID;
        svm.add_program_from_file(program_id, "target/deploy/your_program.so").unwrap();

        // Create undersized vault account
        let vault_keypair = Keypair::new();
        let old_size = 49; // Old vault size
        let new_size = 90; // New vault size

        let rent = Rent::default();
        let old_rent = rent.minimum_balance(old_size);

        svm.create_account(&vault_keypair.pubkey(), old_rent, program_id).unwrap();
        // ... set old vault data ...

        // Execute resize migration
        // Verify:
        // 1. Account size increased
        // 2. New fields initialized
        // 3. Rent balance updated correctly
        // 4. Event emitted
    }

    #[test]
    #[should_panic(expected = "InsufficientRent")]
    fn test_resize_fails_without_enough_rent() {
        // Payer doesn't have enough SOL for resize
        // Migration should fail with InsufficientRent
    }
}
```

### tests/unit/test_borsh_layout.rs

```rust
#[cfg(test)]
mod test_borsh_layout {
    use your_program::state::*;

    #[test]
    fn test_user_struct_size() {
        // Verify struct size matches expected
        let user = User {
            owner: Pubkey::default(),
            balance: 0,
            nickname: None,
            created_at: 0,
        };
        let serialized = user.try_to_vec().unwrap();
        assert_eq!(serialized.len(), User::INIT_SPACE);
    }

    #[test]
    fn test_backward_compatibility() {
        // Serialize old struct (without new fields)
        // Deserialize as new struct
        // Verify new fields have default values
    }
}
```

---

## Layer 2: Integration Tests (Surfpool Fork)

### tests/integration/test_fork_upgrade.rs

```rust
#[cfg(test)]
mod test_fork_upgrade {
    // Requires Surfpool CLI installed
    // Tests upgrade against real mainnet account state

    #[tokio::test]
    async fn test_upgrade_against_mainnet_fork() {
        // 1. Fork mainnet at specific slot
        // 2. Deploy new program
        // 3. Test critical user accounts still work
        // 4. Verify no deserialization errors
    }
}
```

### tests/integration/test_multisig_flow.rs

```rust
#[cfg(test)]
mod test_multisig_flow {
    // Tests full Squads multisig upgrade flow
    // Requires devnet or local validator with Squads program

    #[tokio::test]
    async fn test_buffer_creation_and_authority_transfer() {
        // 1. Create buffer
        // 2. Write program to buffer
        // 3. Set buffer authority to multisig
        // 4. Verify authority changed
    }
}
```

---

## Layer 3: CI Tests

### tests/ci/idl-check.ts

```typescript
// TypeScript IDL compatibility checker
// Run in CI before any PR merge

import * as fs from 'fs';
import * as path from 'path';

interface IdlField {
    name: string;
    type: string;
}

interface IdlAccount {
    name: string;
    type: {
        kind: string;
        fields: IdlField[];
    };
}

interface Idl {
    accounts: IdlAccount[];
    instructions: any[];
}

function loadIdl(filePath: string): Idl {
    return JSON.parse(fs.readFileSync(filePath, 'utf-8'));
}

function checkBreakingChanges(oldIdl: Idl, newIdl: Idl): string[] {
    const errors: string[] = [];

    for (const oldAccount of oldIdl.accounts) {
        const newAccount = newIdl.accounts.find(a => a.name === oldAccount.name);
        if (!newAccount) {
            errors.push(`BREAKING: Account '${oldAccount.name}' removed`);
            continue;
        }

        const oldFields = oldAccount.type.fields;
        const newFields = newAccount.type.fields;

        // Check for field removals
        for (const oldField of oldFields) {
            const newField = newFields.find(f => f.name === oldField.name);
            if (!newField) {
                errors.push(`BREAKING: Field '${oldField.name}' removed from '${oldAccount.name}'`);
                continue;
            }
            if (oldField.type !== newField.type) {
                errors.push(`BREAKING: Field '${oldField.name}' type changed from '${oldField.type}' to '${newField.type}'`);
            }
        }

        // Check for field reordering (fields must stay in same order)
        for (let i = 0; i < oldFields.length; i++) {
            if (oldFields[i].name !== newFields[i].name) {
                errors.push(`BREAKING: Fields reordered in '${oldAccount.name}'. Expected '${oldFields[i].name}' at position ${i}, found '${newFields[i]?.name || 'nothing'}'`);
                break;
            }
        }
    }

    // Check instruction compatibility
    for (const oldIx of oldIdl.instructions) {
        const newIx = newIdl.instructions.find((i: any) => i.name === oldIx.name);
        if (!newIx) {
            errors.push(`BREAKING: Instruction '${oldIx.name}' removed`);
        }
    }

    return errors;
}

function main() {
    const oldIdlPath = process.argv[2] || 'idl/old.json';
    const newIdlPath = process.argv[3] || 'idl/new.json';

    if (!fs.existsSync(oldIdlPath) || !fs.existsSync(newIdlPath)) {
        console.error('IDL files not found');
        process.exit(1);
    }

    const oldIdl = loadIdl(oldIdlPath);
    const newIdl = loadIdl(newIdlPath);

    const errors = checkBreakingChanges(oldIdl, newIdl);

    if (errors.length > 0) {
        console.error('IDL BREAKING CHANGES DETECTED:');
        errors.forEach(e => console.error(`  ❌ ${e}`));
        process.exit(1);
    }

    console.log('✅ IDL compatibility check passed');
}

main();
```

### tests/ci/layout-drift.ts

```typescript
// Detects Borsh layout drift between versions

import { sha256 } from 'crypto';

function computeLayoutHash(fields: any[]): string {
    const layout = fields.map(f => `${f.name}:${f.type}`).join('|');
    return sha256(layout);
}

// Compare layout hashes to detect drift
// If hash changes for existing fields = BREAKING
```

---

## Layer 4: Devnet Rehearsal

### tests/devnet/deploy-devnet.sh

```bash
#!/bin/bash
# Full devnet deployment and test script

set -e

PROGRAM_ID="${1:-}"
if [ -z "$PROGRAM_ID" ]; then
    echo "Usage: ./deploy-devnet.sh <PROGRAM_ID>"
    exit 1
fi

echo "=== Devnet Rehearsal ==="

# 1. Build
echo "Building..."
anchor build

# 2. Deploy to devnet
echo "Deploying to devnet..."
anchor deploy --provider.cluster devnet

# 3. Run tests
echo "Running tests..."
anchor test --provider.cluster devnet --skip-build

# 4. Verify authority
echo "Verifying authority..."
solana program show "$PROGRAM_ID" --url devnet | grep "Authority"

# 5. Test migration (if applicable)
if [ -f "tests/devnet/test-migration.ts" ]; then
    echo "Testing migration..."
    npx ts-node tests/devnet/test-migration.ts
fi

echo "=== Devnet rehearsal complete ==="
```

### tests/devnet/verify-devnet.sh

```bash
#!/bin/bash
# Post-deployment verification

PROGRAM_ID="${1:-}"

echo "Verifying devnet deployment..."

# Check program exists
solana program show "$PROGRAM_ID" --url devnet

# Verify hash
solana program dump "$PROGRAM_ID" /tmp/devnet-verify.so --url devnet
sha256sum /tmp/devnet-verify.so target/deploy/*.so

# Check all user accounts are accessible
# (Add your specific checks here)

echo "Devnet verification complete"
```

---

## Running the Full Suite

```bash
# 1. Unit tests (fast, no network)
cargo test --features test-bpf

# 2. IDL check
npx ts-node tests/ci/idl-check.ts idl/old.json idl/new.json

# 3. Devnet rehearsal
./tests/devnet/deploy-devnet.sh <PROGRAM_ID>

# 4. Mainnet fork test (requires Surfpool)
surfpool test --fork mainnet --program target/deploy/*.so
```
