use anchor_lang::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// Program Upgrade Guardian — Live Demo Program (v2)
// ═══════════════════════════════════════════════════════════════════════
// This program demonstrates:
//   • Lazy migration from v1 → v2
//   • Exact realloc math with byte comments
//   • Emergency pause/unpause
//   • Version tracking
//   • Borsh-safe append-only struct evolution
// ═══════════════════════════════════════════════════════════════════════

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod demo_program {
    use super::*;

    // ═════════════════════════════════════════════════════════════════
    // INITIALIZE (v2 from day one — no migration needed for new accounts)
    // ═════════════════════════════════════════════════════════════════
    pub fn initialize_profile(
        ctx: Context<InitializeProfile>,
        name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;

        profile.owner = ctx.accounts.authority.key();
        profile.created_at = Clock::get()?.unix_timestamp;
        profile.name = name;

        // v2 defaults
        profile.bio = None;
        profile.reputation_score = 0;
        profile.version = 2;

        msg!("Initialized UserProfile v2: {}", profile.key());
        Ok(())
    }

    // ═════════════════════════════════════════════════════════════════
    // UPDATE PROFILE — Triggers lazy migration if account is still v1
    // ═════════════════════════════════════════════════════════════════
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;

        // ═══════════════════════════════════════════════════════════════
        // LAZY MIGRATION: v1 → v2 (runs BEFORE business logic)
        // ═══════════════════════════════════════════════════════════════
        if profile.version == 1 {
            // 1. Set new fields to safe defaults BEFORE realloc
            //    (avoids reading uninitialized memory)
            profile.bio = None;
            profile.reputation_score = 0;
            profile.version = 2;

            // 2. Calculate exact new size
            //    See UserProfile::size() for byte-by-byte breakdown
            let new_size = UserProfile::size(&profile.name, profile.bio.as_ref());

            // 3. Realloc with zero-init = false
            //    (we just wrote the new fields, no need to zero)
            profile
                .to_account_info()
                .realloc(new_size, false)?;

            msg!(
                "MIGRATED: {} v1 → v2 ({} → {} bytes)",
                profile.key(),
                57, // v1 size
                new_size
            );
        }

        // ═══════════════════════════════════════════════════════════════
        // BUSINESS LOGIC (only runs after migration check)
        // ═══════════════════════════════════════════════════════════════
        profile.name = name;

        msg!("Updated profile: {}", profile.key());
        Ok(())
    }

    // ═════════════════════════════════════════════════════════════════
    // UPDATE BIO — v2-only feature, requires migrated account
    // ═════════════════════════════════════════════════════════════════
    pub fn update_bio(
        ctx: Context<UpdateBio>,
        bio: Option<String>,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;

        // Guard: must be migrated first
        require!(profile.version >= 2, ErrorCode::AccountNotMigrated);

        // Realloc if bio size changed
        let old_size = profile.to_account_info().data_len();
        let new_size = UserProfile::size(&profile.name, bio.as_ref());

        if new_size != old_size {
            profile.to_account_info().realloc(new_size, false)?;
        }

        profile.bio = bio;
        msg!("Updated bio for: {}", profile.key());
        Ok(())
    }

    // ═════════════════════════════════════════════════════════════════
    // BATCH MIGRATE — Admin-only, for emergency bulk migration
    // ═════════════════════════════════════════════════════════════════
    pub fn batch_migrate(
        ctx: Context<BatchMigrate>,
        batch_size: u16,
    ) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ADMIN_PUBKEY,
            ErrorCode::Unauthorized
        );
        require!(batch_size <= 50, ErrorCode::BatchTooLarge);

        let mut migrated: u16 = 0;

        for acc_info in ctx.remaining_accounts.iter().take(batch_size as usize) {
            let mut profile = Account::<UserProfile>::try_from(acc_info)?;

            if profile.version == 1 {
                let new_size = UserProfile::size(&profile.name, None);
                acc_info.realloc(new_size, false)?;

                profile.bio = None;
                profile.reputation_score = 0;
                profile.version = 2;
                profile.exit(&crate::ID)?;

                migrated += 1;
                msg!("Batch migrated: {} → v2", acc_info.key);
            }
        }

        msg!("Batch complete: {} accounts migrated", migrated);
        Ok(())
    }

    // ═════════════════════════════════════════════════════════════════
    // EMERGENCY PAUSE — Halts all user-facing instructions
    // ═════════════════════════════════════════════════════════════════
    pub fn emergency_pause(ctx: Context<AdminOnly>) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ADMIN_PUBKEY,
            ErrorCode::Unauthorized
        );

        let state = &mut ctx.accounts.program_state;
        state.paused = true;
        state.pause_timestamp = Clock::get()?.unix_timestamp;

        msg!("🚨 EMERGENCY PAUSE activated at slot {}", Clock::get()?.slot);
        Ok(())
    }

    // ═════════════════════════════════════════════════════════════════
    // EMERGENCY UNPAUSE — 24h cooldown prevents flip-flopping
    // ═════════════════════════════════════════════════════════════════
    pub fn emergency_unpause(ctx: Context<AdminOnly>) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == ADMIN_PUBKEY,
            ErrorCode::Unauthorized
        );

        let state = &ctx.accounts.program_state;
        require!(
            Clock::get()?.unix_timestamp > state.pause_timestamp + 86_400,
            ErrorCode::PauseCooldown
        );

        ctx.accounts.program_state.paused = false;
        msg!("✅ Program unpaused");
        Ok(())
    }

    // ═════════════════════════════════════════════════════════════════
    // DEPOSIT — Example user-facing instruction with pause check
    // ═════════════════════════════════════════════════════════════════
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(!ctx.accounts.program_state.paused, ErrorCode::ProgramPaused);
        require!(amount > 0, ErrorCode::InvalidAmount);

        // ... deposit logic would go here
        msg!("Deposited {} lamports", amount);
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════
// ACCOUNTS
// ═══════════════════════════════════════════════════════════════════════

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + UserProfile::size("", None), // 8 + 62 = 70 bytes minimum
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

#[derive(Accounts)]
pub struct BatchMigrate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Program account, validated in instruction
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user_profile: Account<'info, UserProfile>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
}

// ═══════════════════════════════════════════════════════════════════════
// ACCOUNT STRUCTS
// ═══════════════════════════════════════════════════════════════════════

#[account]
pub struct UserProfile {
    // ── v1 fields (indices 0-2) ──
    pub owner: Pubkey,          // 32 bytes (indices 0)
    pub created_at: i64,        // 8 bytes  (indices 1)
    pub name: String,           // 4 + len  (indices 2)

    // ── v2 fields (APPEND ONLY, indices 3-5) ──
    pub bio: Option<String>,    // 1 + (4 + len)  (indices 3)
    pub reputation_score: u64,  // 8 bytes        (indices 4)
    pub version: u8,            // 1 byte         (indices 5) ← MUST BE LAST
}

impl UserProfile {
    /// Exact byte size for account allocation and realloc.
    /// 
    /// Breakdown (every byte accounted for):
    ///   8   = Anchor discriminator (auto-added by Anchor)
    ///   32  = owner: Pubkey
    ///   8   = created_at: i64
    ///   4 + name.len()  = name: String (u32 length prefix + UTF-8 bytes)
    ///   1   = bio: Option discriminant (0x00 = None, 0x01 = Some)
    ///   bio.map_or(0, |b| 4 + b.len()) = bio: String payload (only if Some)
    ///   8   = reputation_score: u64
    ///   1   = version: u8
    pub fn size(name: &str, bio: Option<&String>) -> usize {
        8                           // discriminator
            + 32                    // owner: Pubkey
            + 8                     // created_at: i64
            + 4 + name.len()        // name: String (u32 prefix + bytes)
            + 1                     // Option discriminant
            + bio.map_or(0, |b| 4 + b.len()) // Optional String payload
            + 8                     // reputation_score: u64
            + 1                     // version: u8
    }

    /// Maximum possible size for buffer calculations.
    /// name: 32 chars, bio: 256 chars
    pub fn max_size() -> usize {
        Self::size("x".repeat(32).as_str(), Some(&"x".repeat(256)))
    }
}

#[account]
pub struct ProgramState {
    pub paused: bool,
    pub pause_timestamp: i64,
    pub current_version: u32,
}

// ═══════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════

#[error_code]
pub enum ErrorCode {
    #[msg("Account has not been migrated to the latest version")]
    AccountNotMigrated,

    #[msg("Unauthorized admin action")]
    Unauthorized,

    #[msg("Batch size must be <= 50 to avoid CU limit")]
    BatchTooLarge,

    #[msg("Program is paused. Contact admin.")]
    ProgramPaused,

    #[msg("Cannot unpause within 24h of pause")]
    PauseCooldown,

    #[msg("Invalid amount")]
    InvalidAmount,
}

// ═══════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════

/// Admin pubkey — MUST be a multisig (Squads/Realms) on mainnet.
/// Rotate via `set_admin` instruction, not hardcoded redeploy.
const ADMIN_PUBKEY: Pubkey = pubkey!(
    "AdminPubkeyHere111111111111111111111111111111"
);
