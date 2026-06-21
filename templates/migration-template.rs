use anchor_lang::prelude::*;
use anchor_lang::system_program;

// ============================================================================
// MIGRATION PATTERN 1: Append Fields (Safest - No realloc needed)
// ============================================================================
// Use when: Adding new fields to the END of an existing struct
// Backward compatible: YES - old accounts deserialize with defaults

/// BEFORE:
/// #[account]
/// pub struct User {
///     pub owner: Pubkey,
///     pub balance: u64,
/// }

/// AFTER:
#[account]
pub struct User {
    pub owner: Pubkey,      // 32 bytes
    pub balance: u64,       // 8 bytes
    pub nickname: Option<String>, // NEW - appended at end
    pub created_at: i64,    // NEW - appended at end
}

/// Migration instruction - only needed if you want to populate new fields
pub fn migrate_user_append(ctx: Context<MigrateUserAppend>) -> Result<()> {
    let user = &mut ctx.accounts.user;

    // Only migrate if fields are not already set
    if user.nickname.is_none() {
        user.nickname = Some("".to_string());
    }
    if user.created_at == 0 {
        user.created_at = Clock::get()?.unix_timestamp;
    }

    emit!(UserMigrated {
        user: user.key(),
        migration_type: "append_fields".to_string(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct MigrateUserAppend<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
}

// ============================================================================
// MIGRATION PATTERN 2: Account Resize (Requires realloc + rent)
// ============================================================================
// Use when: Adding fields that exceed current account size
// Backward compatible: NO - requires explicit migration instruction

/// BEFORE:
/// #[account]
/// pub struct Vault {
///     pub authority: Pubkey,  // 32
///     pub balance: u64,       // 8
///     pub bump: u8,           // 1
/// } // Total: 41 bytes + 8 discriminator = 49

/// AFTER:
#[account]
pub struct Vault {
    pub authority: Pubkey,      // 32
    pub balance: u64,           // 8
    pub bump: u8,               // 1
    pub strategy: StrategyType, // NEW - 1 byte enum
    pub max_deposit: u64,     // NEW - 8 bytes
    pub fee_account: Pubkey,  // NEW - 32 bytes
} // Total: 82 bytes + 8 discriminator = 90

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum StrategyType {
    Conservative,
    Balanced,
    Aggressive,
}

pub fn migrate_vault_resize(ctx: Context<MigrateVaultResize>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let current_size = vault.to_account_info().data_len();
    let new_size = 8 + Vault::INIT_SPACE; // discriminator + struct

    // CRITICAL: Calculate required rent for new size
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);
    let lamports_diff = new_minimum_balance.saturating_sub(vault.to_account_info().lamports());

    // Transfer additional rent if needed
    if lamports_diff > 0 {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: vault.to_account_info(),
                },
            ),
            lamports_diff,
        )?;
    }

    // CRITICAL: Reallocate account size
    vault.to_account_info().realloc(new_size, false)?;

    // Initialize new fields with safe defaults
    vault.strategy = StrategyType::Conservative;
    vault.max_deposit = u64::MAX;
    vault.fee_account = vault.authority; // Default to authority

    emit!(VaultMigrated {
        vault: vault.key(),
        old_size: current_size as u64,
        new_size: new_size as u64,
        additional_rent: lamports_diff,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct MigrateVaultResize<'info> {
    #[account(
        mut,
        constraint = vault.authority == authority.key() @ ErrorCode::InvalidAuthority,
        // Ensure this is the expected old vault
        constraint = vault.to_account_info().data_len() < 90 @ ErrorCode::AlreadyMigrated,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// ============================================================================
// MIGRATION PATTERN 3: Enum Variant Addition (Safe if handled)
// ============================================================================
// Use when: Adding new variants to an existing enum
// Backward compatible: PARTIAL - old accounts deserialize, new variants need handling

/// BEFORE:
/// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
/// pub enum OrderStatus {
///     Pending,
///     Filled,
///     Cancelled,
/// }

/// AFTER:
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
    PartiallyFilled, // NEW - appended at end
    Expired,         // NEW - appended at end
}

/// State struct using the enum
#[account]
pub struct Order {
    pub owner: Pubkey,
    pub status: OrderStatus,
    pub amount: u64,
}

/// Migration: Update logic to handle new variants
pub fn process_order_v2(ctx: Context<ProcessOrderV2>) -> Result<()> {
    let order = &mut ctx.accounts.order;

    match order.status {
        OrderStatus::Pending => {
            // Existing logic
        }
        OrderStatus::Filled => {
            // Existing logic
        }
        OrderStatus::Cancelled => {
            // Existing logic
        }
        // NEW: Handle new variants safely
        OrderStatus::PartiallyFilled => {
            msg!("Order partially filled - new logic");
            // Implement partial fill logic
        }
        OrderStatus::Expired => {
            msg!("Order expired - new logic");
            order.status = OrderStatus::Cancelled;
        }
    }

    emit!(OrderProcessed {
        order: order.key(),
        status: order.status,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ProcessOrderV2<'info> {
    #[account(mut)]
    pub order: Account<'info, Order>,
    pub authority: Signer<'info>,
}

// ============================================================================
// ROLLBACK PATTERN: Emergency State Recovery
// ============================================================================
// Use when: Upgrade failed and you need to revert state changes

#[event]
pub struct MigrationEvent {
    pub account: Pubkey,
    pub migration_type: String,
    pub old_state_hash: [u8; 32],
    pub new_state_hash: [u8; 32],
    pub timestamp: i64,
}

#[event]
pub struct UserMigrated {
    pub user: Pubkey,
    pub migration_type: String,
    pub timestamp: i64,
}

#[event]
pub struct VaultMigrated {
    pub vault: Pubkey,
    pub old_size: u64,
    pub new_size: u64,
    pub additional_rent: u64,
    pub timestamp: i64,
}

#[event]
pub struct OrderProcessed {
    pub order: Pubkey,
    pub status: OrderStatus,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid authority for migration")]
    InvalidAuthority,
    #[msg("Account already migrated")]
    AlreadyMigrated,
    #[msg("Migration not needed")]
    NotNeeded,
    #[msg("Insufficient rent for resize")]
    InsufficientRent,
}
