---
name: migration-template
parent-skill: program-upgrade-guardian
description: >
  Production-ready Rust migration patterns for Anchor programs on Solana.
  Includes lazy migration, batch migration, account resize helpers, PDA versioning,
  enum evolution, rent exemption math, and a TypeScript IDL compatibility CI check.
  All patterns are copy-paste ready with exact byte-size comments.
---

# Migration Templates

> Production-ready Rust migration patterns for Solana Anchor programs.
> Copy, adapt, deploy. Every pattern includes exact byte-size math and safety checks.

---

## Table of Contents

- [Lazy Migration (Preferred)](#lazy-migration-preferred)
- [Batch Migration (Emergency)](#batch-migration-emergency)
- [Account Resize Pattern](#account-resize-pattern)
- [Rent Exemption Math](#rent-exemption-math)
- [PDA Versioning Pattern](#pda-versioning-pattern)
- [Enum Evolution Pattern](#enum-evolution-pattern)
- [IDL Compatibility Check (TypeScript)](#idl-compatibility-check-typescript)
- [Common Pitfalls](#common-pitfalls)

---

## Lazy Migration (Preferred)

Migrate accounts on first user interaction. Zero downtime, minimal blast radius, no admin keys required.

### Full Program Example

```rust
use anchor_lang::prelude::*;

// ── Program ID ──────────────────────────────────────────────
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_program {
    use super::*;

    /// Initialize a new UserProfile (v2 schema from day one).
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;
        profile.owner = ctx.accounts.authority.key();
        profile.created_at = Clock::get()?.unix_timestamp;
        profile.name = name;
        // v2 fields default
        profile.bio = None;
        profile.reputation_score = 0;
        profile.version = 2;
        Ok(())
    }

    /// Update profile — triggers lazy migration if account is still v1.
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;

        // ═══════════════════════════════════════════════════════
        // LAZY MIGRATION: v1 -> v2
        // ═══════════════════════════════════════════════════════
        if profile.version == 1 {
            // 1. Set new fields to safe defaults BEFORE realloc
            profile.bio = None;
            profile.reputation_score = 0;
            profile.version = 2;

            // 2. Calculate exact new size (see UserProfile::size below)
            let new_size = UserProfile::size(&profile.name, profile.bio.as_ref());

            // 3. Realloc with zero-init = false (we just wrote the new fields)
            profile
                .to_account_info()
                .realloc(new_size, false)?;

            msg!(
                "Migrated UserProfile {} v1 -> v2 ({} bytes)",
                profile.key(),
                new_size
            );
        }

        // ── Business logic ──
        profile.name = name;
        Ok(())
    }

    /// Update bio — only works on v2+ accounts.
    pub fn update_bio(
        ctx: Context<UpdateBio>,
        bio: Option<String>,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;

        // Guard: must be migrated first
        require!(profile.version >= 2, ErrorCode::AccountNotMigrated);

        // Realloc if bio grew
        let old_size = profile.to_account_info().data_len();
        let new_size = UserProfile::size(&profile.name, bio.as_ref());

        if new_size != old_size {
            profile.to_account_info().realloc(new_size, false)?;
        }

        profile.bio = bio;
        Ok(())
    }
}

// ── Accounts ────────────────────────────────────────────────

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + UserProfile::size(&"".to_string(), None),
        seeds = [b"user_profile", authority.key().as_ref()],
        bump,
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut, has_one = owner)]
    pub user_profile: Account<'info, UserProfile>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateBio<'info> {
    #[account(mut, has_one = owner)]
    pub user_profile: Account<'info, UserProfile>,
    pub owner: Signer<'info>,
}

// ── Account Struct ────────────────────────────────────────────

#[account]
pub struct UserProfile {
    // ── v1 fields ──
    pub owner: Pubkey,          // 32 bytes
    pub created_at: i64,        // 8 bytes
    pub name: String,           // 4 + len bytes

    // ── v2 fields (APPEND ONLY) ──
    pub bio: Option<String>,    // 1 + (4 + len) bytes
    pub reputation_score: u64,  // 8 bytes
    pub version: u8,            // 1 byte  ← MUST BE LAST
}

impl UserProfile {
    /// Exact byte size for account allocation and realloc.
    ///
    /// Breakdown:
    ///   8   = Anchor discriminator
    ///   32  = owner: Pubkey
    ///   8   = created_at: i64
    ///   4 + name.len()  = name: String (u32 len prefix + UTF-8 bytes)
    ///   1   = bio: Option discriminant (0 = None, 1 = Some)
    ///   bio.map_or(0, |b| 4 + b.len()) = bio: String payload (only if Some)
    ///   8   = reputation_score: u64
    ///   1   = version: u8
    pub fn size(name: &str, bio: Option<&String>) -> usize {
        8                           // discriminator
            + 32                    // owner: Pubkey
            + 8                     // created_at: i64
            + 4 + name.len()        // name: String
            + 1                     // Option discriminant
            + bio.map_or(0, |b| 4 + b.len()) // Optional String payload
            + 8                     // reputation_score: u64
            + 1                     // version: u8
    }

    /// Maximum possible size (for buffer calculations).
    pub fn max_size() -> usize {
        // name: 32 chars, bio: 256 chars
        Self::size("x".repeat(32).as_str(), Some(&"x".repeat(256)))
    }
}

// ── Errors ────────────────────────────────────────────────────

#[error_code]
pub enum ErrorCode {
    #[msg("Account has not been migrated to the latest version")]
    AccountNotMigrated,
}
```

### Lazy Migration Checklist

- [ ] `version` field is **last** in struct (append-only discipline)
- [ ] Migration gate (`if profile.version == 1`) runs **before** business logic
- [ ] New fields initialized **before** `realloc` (avoid reading uninitialized memory)
- [ ] `size()` method has a comment explaining every byte term
- [ ] `realloc` uses `false` for `zero_init` (performance + safety)
- [ ] `msg!()` logs every migration for observability
- [ ] Instructions that require v2+ use `require!(version >= 2, ...)`
- [ ] Tested on Surfpool fork with real v1 accounts

---

## Batch Migration (Emergency)

Admin-only bulk migration for critical fixes. Use when lazy migration is impossible (e.g., breaking seed change, emergency patch).

### Full Program Example

```rust
use anchor_lang::prelude::*;

#[program]
pub mod my_program {
    use super::*;

    /// Admin-only: migrate a batch of v1 accounts to v2.
    ///
    /// # Arguments
    /// * `batch_size` — Number of accounts to process (max 50 per tx).
    ///
    /// # Accounts
    /// * `authority` — Must match `ADMIN_PUBKEY`.
    /// * `remaining_accounts` — Slice of `UserProfile` accounts to migrate.
    pub fn batch_migrate(
        ctx: Context<BatchMigrate>,
        batch_size: u16,
    ) -> Result<()> {
        // ── Authorization ──
        require!(
            ctx.accounts.authority.key() == ADMIN_PUBKEY,
            ErrorCode::Unauthorized
        );

        // ── Batch size cap (avoid Compute Unit limit) ──
        require!(batch_size <= 50, ErrorCode::BatchTooLarge);

        let mut migrated: u16 = 0;

        for acc_info in ctx.remaining_accounts.iter().take(batch_size as usize) {
            // Deserialize with try_from (validates owner + discriminator)
            let mut profile = Account::<UserProfile>::try_from(acc_info)?;

            if profile.version == 1 {
                // 1. Calculate new size
                let new_size = UserProfile::size(&profile.name, None);

                // 2. Realloc account
                acc_info.realloc(new_size, false)?;

                // 3. Set new fields
                profile.bio = None;
                profile.reputation_score = 0;
                profile.version = 2;

                // 4. Serialize back to account data
                profile.exit(&crate::ID)?;

                migrated += 1;

                msg!(
                    "Batch migrated {} -> v2 ({} bytes)",
                    acc_info.key,
                    new_size
                );
            }
        }

        msg!("Batch complete: {} / {} accounts migrated", migrated, batch_size);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BatchMigrate<'info> {
    pub authority: Signer<'info>,
    /// CHECK: Program account, validated in instruction logic.
    pub program: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized admin action")]
    Unauthorized,
    #[msg("Batch size must be <= 50 to avoid CU limit")]
    BatchTooLarge,
}

/// Admin pubkey — MUST be a multisig (Squads/Realms) on mainnet.
/// Rotate via `set_admin` instruction, not hardcoded redeploy.
const ADMIN_PUBKEY: Pubkey = pubkey!(
    "AdminPubkeyHere111111111111111111111111111111"
);
```

### Batch Migration Checklist

- [ ] `ADMIN_PUBKEY` is a **multisig PDA** on mainnet (never a hot wallet)
- [ ] `batch_size` hard-capped at 50 accounts (Compute Unit safety)
- [ ] Each account validated with `Account::try_from` (owner + discriminator check)
- [ ] `exit()` called after mutation to serialize state back
- [ ] Migration events logged with account pubkey and new size
- [ ] Tested on devnet with 100+ accounts across multiple transactions
- [ ] Admin rotation instruction exists (avoid hardcoded redeploy)

---

## Account Resize Pattern

Standalone helper for safe `realloc` with automatic rent-exemption top-up.

```rust
use anchor_lang::prelude::*;
use anchor_lang::system_program;

/// Resize an account safely, transferring rent difference from payer if needed.
///
/// # Safety
/// - Verifies new size <= 10,240 bytes (Solana max)
/// - Calculates exact rent-exempt lamports for new size
/// - Transfers difference from payer if account is underfunded
/// - Uses `zero_init = false` by default (pass `true` if reading new bytes)
pub fn safe_realloc<'info>(
    account: &AccountInfo<'info>,
    new_size: usize,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    zero_init: bool,
) -> Result<()> {
    // ── Hard limit ──
    require!(
        new_size <= 10_240,
        ErrorCode::AccountTooLarge
    );

    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);
    let lamports_diff = new_minimum_balance.saturating_sub(account.lamports());

    // ── Transfer rent difference from payer ──
    if lamports_diff > 0 {
        system_program::transfer(
            CpiContext::new(
                system_program.clone(),
                system_program::Transfer {
                    from: payer.clone(),
                    to: account.clone(),
                },
            ),
            lamports_diff,
        )?;
    }

    // ── Realloc ──
    account.realloc(new_size, zero_init)?;

    msg!(
        "Realloc: {} -> {} bytes (rent diff: {} lamports)",
        account.data_len(),
        new_size,
        lamports_diff
    );

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account exceeds 10,240 byte limit")]
    AccountTooLarge,
}
```

### Usage in Instruction

```rust
pub fn update_bio(ctx: Context<UpdateBio>, bio: Option<String>) -> Result<()> {
    let profile = &mut ctx.accounts.user_profile;
    let new_size = UserProfile::size(&profile.name, bio.as_ref());

    safe_realloc(
        &profile.to_account_info(),
        new_size,
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        false, // no zero-init needed
    )?;

    profile.bio = bio;
    Ok(())
}
```

---

## Rent Exemption Math

Pre-calculate rent-exempt lamports for account sizing.

```rust
use solana_program::rent::Rent;

/// Pre-calculate rent-exempt lamports for a given account size.
/// Use this in your client/frontend to fund accounts correctly.
pub fn rent_exempt_lamports(data_size: usize) -> u64 {
    let rent = Rent::default();
    rent.minimum_balance(data_size)
}

// Examples:
// rent_exempt_lamports(165)  -> 0.002_039_280 SOL (Token account)
// rent_exempt_lamports(200)  -> 0.002_473_920 SOL
// rent_exempt_lamports(10240)-> 0.126_489_600 SOL (max)
```

### Rent Exemption Quick Reference

| Account Size | Rent-Exempt Lamports | Approx SOL |
|-------------|---------------------|------------|
| 128 bytes | 890,880 | 0.00089 |
| 165 bytes | 2,039,280 | 0.00204 |
| 200 bytes | 2,473,920 | 0.00247 |
| 500 bytes | 6,168,960 | 0.00617 |
| 1,000 bytes | 12,336,000 | 0.01234 |
| 5,000 bytes | 61,684,800 | 0.06168 |
| 10,000 bytes | 123,369,600 | 0.12337 |
| 10,240 bytes (max) | 126,489,600 | 0.12649 |

---

## PDA Versioning Pattern

When you MUST break seeds, version the PDA instead of mutating existing seeds.

### Bad vs Good

```rust
// ❌ BAD: Changing seeds breaks ALL existing PDAs
pub fn get_user_profile_pda_bad(user: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"user_profile", user.as_ref()],  // old seed
        &crate::ID,
    )
}

// ✅ GOOD: Version the seed, keep old PDAs reachable
pub fn get_user_profile_pda(user: &Pubkey, version: u8) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"user_profile", user.as_ref(), &[version]],
        &crate::ID,
    )
}
```

### Migration Instruction: Copy v1 -> v2 PDA

```rust
#[program]
pub mod my_program {
    use super::*;

    /// Migrate a user's data from v1 PDA to v2 PDA, then close v1.
    /// User pays for new PDA rent; old rent is refunded to user.
    pub fn migrate_pda(ctx: Context<MigratePda>) -> Result<()> {
        let v1 = &ctx.accounts.old_profile;
        let v2 = &mut ctx.accounts.new_profile;

        // Copy v1 fields
        v2.owner = v1.owner;
        v2.created_at = v1.created_at;
        v2.name = v1.name.clone();

        // Set v2 defaults
        v2.bio = None;
        v2.reputation_score = 0;
        v2.version = 2;

        // Close old PDA — rent refunded to user
        ctx.accounts.old_profile.close(
            ctx.accounts.user.to_account_info()
        )?;

        msg!("Migrated PDA {} -> {}", ctx.accounts.old_profile.key(), v2.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MigratePda<'info> {
    #[account(
        mut,
        seeds = [b"user_profile", user.key().as_ref()],
        bump,
        close = user,  // rent refund destination
    )]
    pub old_profile: Account<'info, UserProfile>,

    #[account(
        init,
        payer = user,
        space = 8 + UserProfile::max_size(),
        seeds = [b"user_profile", user.key().as_ref(), &[2]],
        bump,
    )]
    pub new_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
```

### PDA Versioning Checklist

- [ ] Old PDA seeds remain unchanged (users can still read historical data)
- [ ] New PDA includes version byte in seeds
- [ ] Old PDA closed with `close = user` (rent refund, not burned)
- [ ] Client code updated to derive v2 PDA for new operations
- [ ] v1 instructions remain functional for backward compatibility

---

## Enum Evolution Pattern

Safe enum evolution: append variants, never reorder, never remove.

```rust
// v1 enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,    // discriminant 0
    Filled,     // discriminant 1
    Cancelled,  // discriminant 2
}

// v2 enum — SAFE: append only
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,      // 0
    Filled,       // 1
    Cancelled,    // 2
    Expired,      // 3 ← APPEND ONLY
    PartialFill,  // 4 ← APPEND ONLY
}
```

### Enum with Data

```rust
// v1
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PaymentMethod {
    Sol,           // 0
    SplToken(Pubkey), // 1 + 32 bytes
}

// v2 — SAFE: append new variant with data
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum PaymentMethod {
    Sol,                    // 0
    SplToken(Pubkey),      // 1 + 32
    CreditCardHash([u8; 32]), // 2 + 32 ← APPEND ONLY
}
```

### Handling Unknown Variants (Forward Compatibility)

```rust
/// Deserialize safely, handling unknown future variants.
pub fn safe_deserialize_status(data: &[u8]) -> Result<OrderStatus> {
    match data.first() {
        Some(0) => Ok(OrderStatus::Pending),
        Some(1) => Ok(OrderStatus::Filled),
        Some(2) => Ok(OrderStatus::Cancelled),
        Some(3) => Ok(OrderStatus::Expired),
        Some(4) => Ok(OrderStatus::PartialFill),
        Some(d) => {
            msg!("Unknown OrderStatus discriminant: {}", d);
            Err(ErrorCode::UnknownVariant.into())
        }
        None => Err(ErrorCode::InvalidData.into()),
    }
}
```

---

## IDL Compatibility Check (TypeScript)

Add this to your CI pipeline to catch Borsh drift before deployment.

```typescript
// scripts/idl-check.ts
import { Idl } from "@coral-xyz/anchor";
import { readFileSync } from "fs";

const OLD_IDL_PATH = "idl/onchain.json";
const NEW_IDL_PATH = "target/idl/my_program.json";

interface IdlCheckResult {
    passed: boolean;
    errors: string[];
    warnings: string[];
}

function checkIdlCompatibility(oldIdl: Idl, newIdl: Idl): IdlCheckResult {
    const errors: string[] = [];
    const warnings: string[] = [];

    // ── Check accounts ──
    for (const oldAcc of oldIdl.accounts ?? []) {
        const newAcc = newIdl.accounts?.find((a) => a.name === oldAcc.name);

        if (!newAcc) {
            errors.push(`BLOCK: Account '${oldAcc.name}' removed from IDL`);
            continue;
        }

        const oldFields = (oldAcc.type as any).fields.map((f: any) => ({
            name: f.name,
            type: JSON.stringify(f.type),
        }));
        const newFields = (newAcc.type as any).fields.map((f: any) => ({
            name: f.name,
            type: JSON.stringify(f.type),
        }));

        // Check field order (critical for Borsh)
        for (let i = 0; i < oldFields.length; i++) {
            if (i >= newFields.length) {
                errors.push(
                    `BLOCK: Field '${oldFields[i].name}' missing from '${oldAcc.name}' (would shift indices)`
                );
                continue;
            }
            if (oldFields[i].name !== newFields[i].name) {
                errors.push(
                    `BLOCK: Field order changed in '${oldAcc.name}'. ` +
                    `Expected '${oldFields[i].name}' at index ${i}, found '${newFields[i].name}'`
                );
            }
            if (oldFields[i].type !== newFields[i].type) {
                errors.push(
                    `BLOCK: Field '${oldFields[i].name}' type changed in '${oldAcc.name}'. ` +
                    `Old: ${oldFields[i].type}, New: ${newFields[i].type}`
                );
            }
        }

        // Check for removed fields
        for (const oldField of oldFields) {
            if (!newFields.some((f: any) => f.name === oldField.name)) {
                errors.push(
                    `BLOCK: Field '${oldField.name}' removed from '${oldAcc.name}'`
                );
            }
        }

        // Warn about new fields (expected, but flag for review)
        for (const newField of newFields) {
            if (!oldFields.some((f: any) => f.name === newField.name)) {
                warnings.push(
                    `INFO: New field '${newField.name}' added to '${oldAcc.name}' at index ${newFields.indexOf(newField)}`
                );
            }
        }
    }

    // ── Check enums ──
    for (const oldType of oldIdl.types ?? []) {
        if (oldType.type.kind !== "enum") continue;

        const newType = newIdl.types?.find((t) => t.name === oldType.name);
        if (!newType) {
            errors.push(`BLOCK: Enum '${oldType.name}' removed from IDL`);
            continue;
        }

        const oldVariants = (oldType.type as any).variants.map((v: any) => v.name);
        const newVariants = (newType.type as any).variants.map((v: any) => v.name);

        // Check variant order
        for (let i = 0; i < oldVariants.length; i++) {
            if (i >= newVariants.length) {
                errors.push(
                    `BLOCK: Enum variant '${oldVariants[i]}' missing from '${oldType.name}'`
                );
                continue;
            }
            if (oldVariants[i] !== newVariants[i]) {
                errors.push(
                    `BLOCK: Enum variant order changed in '${oldType.name}'. ` +
                    `Expected '${oldVariants[i]}' at index ${i}, found '${newVariants[i]}'`
                );
            }
        }

        // Check for removed variants
        for (const v of oldVariants) {
            if (!newVariants.includes(v)) {
                errors.push(`BLOCK: Enum variant '${v}' removed from '${oldType.name}'`);
            }
        }
    }

    // ── Check instructions ──
    for (const oldIx of oldIdl.instructions ?? []) {
        const newIx = newIdl.instructions?.find((i) => i.name === oldIx.name);
        if (!newIx) {
            warnings.push(`INFO: Instruction '${oldIx.name}' removed (backward compat risk)`);
        }
    }

    return {
        passed: errors.length === 0,
        errors,
        warnings,
    };
}

// ── Run check ──
const oldIdl: Idl = JSON.parse(readFileSync(OLD_IDL_PATH, "utf-8"));
const newIdl: Idl = JSON.parse(readFileSync(NEW_IDL_PATH, "utf-8"));

const result = checkIdlCompatibility(oldIdl, newIdl);

console.log("
═══════════════════════════════════════════════════");
console.log("  IDL Compatibility Check");
console.log("═══════════════════════════════════════════════════
");

if (result.warnings.length > 0) {
    console.log("⚠️  Warnings:");
    result.warnings.forEach((w) => console.log(`   ${w}`));
    console.log();
}

if (result.errors.length > 0) {
    console.log("❌ BLOCKING ERRORS:");
    result.errors.forEach((e) => console.log(`   ${e}`));
    console.log("
⛔ Upgrade BLOCKED. Fix errors before deploying.
");
    process.exit(1);
} else {
    console.log("✅ IDL compatible: append-only changes detected.
");
    process.exit(0);
}
```

### CI Integration

```yaml
# .github/workflows/idl-check.yml
name: IDL Compatibility Check

on:
  pull_request:
    paths:
      - "programs/**"
      - "target/idl/**"

jobs:
  idl-check:
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
```

---

## Common Pitfalls

| Pitfall | Why It Breaks | Solution |
|---------|---------------|----------|
| Forgetting `8` byte discriminator | Anchor adds 8 bytes before your data; size math is wrong by 8 | Always include `8 +` in `space =` and `size()` |
| `String` without `Option` | Empty `""` still costs 4 bytes; `None` costs 1 | Use `Option<String>` for optional text |
| `realloc` with `zero_init = true` on existing data | Overwrites existing fields with zeros | Use `false` unless reading new uninitialized bytes |
| Missing `exit()` in batch migration | Changes not serialized back to account data | Always call `profile.exit(&crate::ID)?` |
| Changing `u64` to `u128` | Doubles field size, shifts all subsequent fields | Append new `u128` field, deprecate old `u64` |
| Enum without explicit discriminant | Compiler may reorder variants | Never rely on implicit ordering; append only |
| Not testing with largest real account | `AccountTooLarge` panic on mainnet | Surfpool test with max-size account from production |

---

*Version: 2026.06 | Always run Surfpool before mainnet.*
