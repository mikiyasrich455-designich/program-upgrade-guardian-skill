# Program Upgrade Guardian — Live Demo

> A working Anchor program that demonstrates every concept from the Guardian Skill in action.

---

## What This Demo Proves

| Guardian Skill Claim | This Demo Shows |
|---------------------|-----------------|
| "Lazy migration upgrades accounts safely" | Run `anchor test`, watch v1 → v2 live |
| "Append-only Borsh is safe" | v2 adds fields without corrupting v1 data |
| "Buffer workflow prevents bricking" | Deploy v2 via buffer, rollback to v1 if needed |
| "Multisig authority is mandatory" | Show authority transfer flow |
| "Realloc math must be exact" | Account grows from 57 → 67 bytes, verified |
| "Emergency pause stops drains" | Pause instruction halts all user-facing Ixs |

---

## Quick Start

### Prerequisites

```bash
anchor --version      # >= 0.30.0
solana --version      # >= 1.18.0
```

### Run the Demo

```bash
cd demo
npm install
anchor build
anchor test
```

Expected output:
```
✅ test_initialize_v1 ... ok
✅ test_lazy_migration_v1_to_v2 ... ok
✅ test_idempotent_migration ... ok
✅ test_realloc_exactness ... ok
✅ test_batch_migration ... ok
✅ test_emergency_pause ... ok
✅ test_rollback_to_v1 ... ok
```

---

## Demo Scenarios

### Scenario 1: Lazy Migration (The Safe Way)

```
User has v1 account (57 bytes)
    │
    ▼
Calls update_profile (any instruction)
    │
    ▼
Program detects version == 1
    │
    ▼
Auto-migrates: adds bio, reputation_score, version
    │
    ▼
Account grows 57 → 67 bytes
    │
    ▼
User continues with v2 features
```

**Run it:** `anchor test test_lazy_migration_v1_to_v2`

### Scenario 2: Borsh Safety (Append vs Reorder)

| Action | Result | Why |
|--------|--------|-----|
| Append field at end | ✅ Works | Borsh indices unchanged |
| Insert field in middle | ❌ BREAKS | Shifts all subsequent indices |
| Reorder fields | ❌ BREAKS | Corrupts existing accounts |
| Remove field | ❌ BREAKS | Deserialization fails |

**Run it:** `anchor test test_borsh_append_only`

### Scenario 3: Emergency Pause (Stop the Bleeding)

```
Attack detected → Admin calls emergency_pause()
    │
    ▼
All user instructions return ErrorCode::ProgramPaused
    │
    ▼
Fix deployed via buffer + multisig
    │
    ▼
emergency_unpause() after 24h cooldown
```

**Run it:** `anchor test test_emergency_pause`

### Scenario 4: Rollback (When Upgrade Goes Wrong)

```
v2 deployed → Bug found → Users can't interact
    │
    ▼
Deploy v1 buffer (kept for 7 days)
    │
    ▼
Program restored to last known good state
    │
    ▼
Fix bug, run full Guardian Pipeline, redeploy v2.1
```

**Run it:** `anchor test test_rollback_to_v1`

---

## File Structure

```
demo/
├── Anchor.toml              # Anchor config
├── Cargo.toml               # Workspace config
├── package.json             # Node deps
├── README.md                # This file
├── app/
│   └── client.ts            # TypeScript client for testing
└── programs/
    └── demo_program/
        ├── Cargo.toml       # Program deps
        ├── src/
        │   ├── lib.rs       # v2 program with migration
        │   └── lib_v1.rs    # Original v1 (for rollback tests)
        └── tests/
            └── test_migration.rs  # All test scenarios
```

---

## Key Concepts Demonstrated

### 1. Version Tracking

```rust
#[account]
pub struct UserProfile {
    // v1 fields
    pub owner: Pubkey,          // 32
    pub created_at: i64,      // 8
    pub name: String,         // 4 + len
    // v2 fields (APPEND ONLY)
    pub bio: Option<String>,   // 1 + (4 + len)
    pub reputation_score: u64, // 8
    pub version: u8,           // 1 ← LAST
}
```

### 2. Lazy Migration Gate

```rust
if profile.version == 1 {
    profile.bio = None;
    profile.reputation_score = 0;
    profile.version = 2;
    let new_size = UserProfile::size(&profile.name, None);
    profile.to_account_info().realloc(new_size, false)?;
}
```

### 3. Exact Realloc Math

```rust
pub fn size(name: &str, bio: Option<&String>) -> usize {
    8 + 32 + 8 + (4 + name.len()) + 1 + bio.map_or(0, |b| 4 + b.len()) + 8 + 1
}
// 8(disc) + 32(owner) + 8(created) + 4+len(name) + 1(Option) + bio + 8(score) + 1(version)
```

### 4. Emergency Pause

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(!ctx.accounts.program_state.paused, ErrorCode::ProgramPaused);
    // ... deposit logic
}
```

---

## Test Coverage

| Test | What It Validates | Guardian Rule |
|------|-------------------|---------------|
| `test_initialize_v1` | v1 account creation | Baseline |
| `test_lazy_migration_v1_to_v2` | Auto-migration on first touch | Lazy > Batch |
| `test_idempotent_migration` | Double-migration is no-op | Safety |
| `test_realloc_exactness` | Byte math is perfect | Exact realloc |
| `test_borsh_append_only` | Append safe, reorder breaks | Borsh rules |
| `test_batch_migration` | Admin bulk migration | Emergency only |
| `test_emergency_pause` | Halt all instructions | Containment |
| `test_rollback_to_v1` | Downgrade to previous buffer | Rollback plan |
| `test_authority_check` | Only multisig can upgrade | Authority rules |
| `test_size_maximum` | Account fits 10,240 limit | Solana limits |

---

## Running on Devnet

```bash
# 1. Switch to devnet
solana config set --url devnet

# 2. Airdrop SOL
solana airdrop 2

# 3. Deploy v1
anchor deploy --provider.cluster devnet

# 4. Create some v1 accounts
# (run app/client.ts or manual instructions)

# 5. Build v2
anchor build

# 6. Write buffer
solana program write-buffer target/deploy/demo_program.so \
  --buffer-authority ~/.config/solana/id.json

# 7. Deploy v2 (simulating upgrade)
anchor deploy --provider.cluster devnet

# 8. Interact with old accounts — watch lazy migration happen
```

---

## Lessons Learned

1. **Always append fields** — One reorder costs you every user account
2. **Version is your friend** — Check it before any business logic
3. **Realloc math is sacred** — Comment every byte, test every edge case
4. **Keep old buffers** — 7 days of rollback insurance is cheap
5. **Pause before panic** — Emergency pause stops damage in seconds
6. **Test on real accounts** — Surfpool fork with mainnet data catches what unit tests miss

---

*This demo is part of the Program Upgrade Guardian Skill. See root README.md for full skill documentation.*
