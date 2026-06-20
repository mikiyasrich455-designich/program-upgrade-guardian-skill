name: program-lifecycle parent-skill: program-upgrade-guardian 
description: > Complete lifecycle management for Solana programs — from initial deployment through sunset deprecation. Covers deployment patterns, version tracking, emergency procedures, and program closure.

Program Lifecycle
From anchor init to program sunset. Every phase, every decision, every risk.

Table of Contents

Phase Overview

Phase 1: Deployment

Phase 2: Active Operations

Phase 3: Upgrade Cycles

Phase 4: Deprecation

Phase 5: Sunset & Closure


Version Tracking System
Emergency Playbook
Decision Matrix
Phase Overview
plain
┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│  DEPLOYMENT  │───▶│    ACTIVE    │───▶│   UPGRADE    │───▶│ DEPRECATION  │───▶│    SUNSET    │
│   (Initial)  │    │ (Operations) │    │   (Cycles)   │    │  (Warning)   │    │  (Closure)   │
│              │    │              │    │              │    │              │    │              │
│ - Authority  │    │ - Monitoring │    │ - Buffer     │    │ - Freeze     │    │ - Close      │
│   to multisig│    │ - Incident   │    │   workflow   │    │   deposits   │    │   program    │
│ - IDL freeze │    │   response   │    │ - Migration  │    │ - Migration  │    │ - Refund     │
│ - Verifiable │    │ - CU tuning  │    │   testing    │    │   window     │    │   rent       │
│   build      │    │              │    │ - Rollback   │    │ - Final      │    │ - Archive    │
│              │    │              │    │   plan       │    │   upgrade    │    │   state      │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
Table
Phase	Duration	Key Risk	Authority Required
Deployment	1-2 days	Hot wallet authority	Hot wallet → Multisig
Active	Ongoing	Operational bugs	Multisig
Upgrade	Hours to days	Borsh drift, migration failure	Multisig quorum
Deprecation	30-90 days	User funds trapped	Multisig
Sunset	1-7 days	Rent recovery disputes	Multisig + legal
Phase 1: Deployment
Initial Deployment Checklist
plain
□ 1. Build with verifiable flag
   anchor build --verifiable

□ 2. Deploy to devnet first
   anchor deploy --provider.cluster devnet

□ 3. Run full test suite on devnet
   anchor test --provider.cluster devnet

□ 4. Deploy to mainnet
   solana program deploy target/deploy/my_program.so

□ 5. Record program ID and deploy slot
   solana program show <PROGRAM_ID> --output json

□ 6. Transfer authority to multisig IMMEDIATELY
   solana program set-upgrade-authority <PROGRAM_ID> \
     --new-upgrade-authority <SQUADS_AUTHORITY>

□ 7. Verify authority transfer on-chain
   solana program show <PROGRAM_ID> | grep "Authority"

□ 8. Publish verified build to GitHub releases
   anchor verify <PROGRAM_ID>

□ 9. Document program ID in runbook
   Add to: docs/runbook.md, README.md, .env.example
Deployment Anti-Patterns
Table
Anti-Pattern	Why It Fails	Correct Approach
Deploy with hot wallet, "transfer later"	Later never comes; key gets compromised	Transfer to multisig in same deployment session
Skip devnet rehearsal	Mainnet-specific bugs (CU limits, rent)	Full devnet test including load simulation
No IDL backup	Cannot verify on-chain program later	Commit IDL to repo, tag release
Deploy without --verifiable	Cannot prove build matches source	Always use --verifiable; publish hash
Phase 2: Active Operations
Daily Operations
Table
Task	Frequency	Command / Tool	Owner
Error rate check	Every 5 min	scripts/monitor.sh	Automated
CU usage review	Weekly	Helius analytics dashboard	Engineering
Authority verification	Weekly	solana program show	Security
Dependency audit	Monthly	cargo audit	Security
IDL drift scan	Monthly	scripts/idl-check.ts	Engineering
Multisig signer health	Monthly	Squads member verification	Operations
Incident Response Tiers
Table
Tier	Trigger	Response Time	Action
P0	Authority changed unexpectedly	Immediate	Emergency rollback, page on-call
P1	>5% error rate for >10 min	15 min	Investigate, prepare hotfix buffer
P2	CU usage >80% median	1 hour	Optimize instruction, plan upgrade
P3	Deprecation warning from dependency	24 hours	Plan upgrade cycle
Phase 3: Upgrade Cycles
Upgrade Cadence
Table
Program Type	Recommended Cadence	Max Accumulated Changes
DeFi protocol	2-4 weeks	1-2 features per cycle
NFT marketplace	4-6 weeks	3-5 features per cycle
Governance DAO	8-12 weeks	Major features only
Infrastructure	Ad-hoc	Security patches only
Upgrade Decision Flow
plain
Need change?
    │
    ▼
Is it a security patch?
    ├── YES ──▶ Emergency pipeline (skip devnet, direct buffer + multisig)
    │
    └── NO
        │
        ▼
Does it touch account structs?
        ├── YES ──▶ Full Guardian Pipeline (Discovery → Analysis → Test → Deploy)
        │
        └── NO
            │
            ▼
        Is it instruction-only (no state change)?
            ├── YES ──▶ Standard pipeline (LiteSVM + Devnet + Buffer)
            │
            └── NO ──▶ Re-evaluate scope
Version Tracking
Every upgrade must update the on-chain version:
rust
#[account]
pub struct ProgramState {
    pub version: u32,           // Semver: 1_02_03 = v1.2.3
    pub last_upgrade_slot: u64,
    pub upgrade_authority: Pubkey,
}

// In initialize / upgrade instruction
program_state.version = 1_02_03;  // v1.2.3
program_state.last_upgrade_slot = Clock::get()?.slot;
Phase 4: Deprecation
Deprecation Timeline
Table
Day	Action	Communication
0	Announce deprecation	Twitter, Discord, in-app banner
7	Freeze new deposits / mints	Instruction returns Deprecated error
14	Publish migration guide	Docs site, GitHub release notes
30	Begin forced migration window	Auto-migrate on user interaction
60	Final upgrade (remove old instructions)	Last chance for users
90	Program closure	Rent refund, state archive
Deprecation Patterns
rust
#[error_code]
pub enum ErrorCode {
    #[msg("This instruction is deprecated. Migrate to v2.")]
    InstructionDeprecated,
    #[msg("New deposits are frozen. Withdraw only.")]
    DepositsFrozen,
}

pub fn old_instruction(ctx: Context<OldIx>) -> Result<()> {
    // Return error after deprecation date
    if Clock::get()?.unix_timestamp > DEPRECATION_TIMESTAMP {
        return Err(ErrorCode::InstructionDeprecated.into());
    }
    // ... old logic
}
Phase 5: Sunset & Closure
Program Closure Requirements
Before closing a program, you MUST:
Verify no user funds remain
bash
solana program show <PROGRAM_ID>
# Check all token accounts, PDAs, vaults
Announce closure 30 days in advance
Provide withdrawal window
rust
pub fn emergency_withdraw(ctx: Context<EmergencyWithdraw>) -> Result<()> {
    // Allow users to withdraw even after program closure
    // This instruction remains functional until all funds recovered
}
Execute closure via multisig
bash
solana program close <PROGRAM_ID> \
  --recipient <RENT_RECOVERY_ADDRESS> \
  --keypair <MULTISIG_KEYPAIR>
Archive final state
bash
# Dump all program accounts before closure
solana program show --accounts <PROGRAM_ID> --output json > final-state.json
Closure Checklist
[ ] All user funds withdrawn or explicitly forfeited (legal review)
[ ] All token accounts closed (zero balance)
[ ] All PDAs emptied
[ ] Final state snapshot archived
[ ] Rent recovery address documented
[ ] Closure transaction proposed via multisig
[ ] 48-hour waiting period after proposal (no objections)
[ ] Post-closure verification: program no longer on-chain
Version Tracking System
On-Chain Version Registry
rust
#[account]
pub struct VersionRegistry {
    pub current_version: u32,
    pub supported_versions: Vec<u32>,  // Backward compat list
    pub deprecated_versions: Vec<u32>,
    pub min_supported_client: u32,     // Force client upgrades
}
Off-Chain Version Tracking
Table
Source	Purpose	Update Trigger
Cargo.toml	Build version	Every commit
Anchor.toml	Framework version	Anchor upgrade
Git tag	Release version	Every mainnet deploy
GitHub Release	Verified build artifact	After devnet + mainnet pass
On-chain ProgramState	Runtime version	Every upgrade instruction
IDL version	Client compatibility	Every account change
Version Compatibility Matrix
Table
Client Version	Program v1.x	Program v2.x	Program v3.x
Client 1.x	✅ Full	✅ Read-only	❌ Incompatible
Client 2.x	✅ Backward compat	✅ Full	✅ Read-only
Client 3.x	❌ Incompatible	✅ Backward compat	✅ Full
Emergency Playbook
Scenario: Authority Compromised
plain
1. IMMEDIATE (0-5 min):
   □ Verify compromise (unauthorized tx in history)
   □ Page security team
   □ Prepare emergency closure transaction

2. SHORT TERM (5-30 min):
   □ If possible: revoke authority with remaining multisig signers
   □ If not possible: execute emergency program closure
   □ Notify users via all channels

3. RECOVERY (30 min - 24h):
   □ Deploy new program with clean authority
   □ Migrate state if possible
   □ Post-incident report
Scenario: Bricked Upgrade
plain
1. DETECT (0-1 min):
   □ Monitor alerts: error rate spike
   □ Confirm: program hash changed, instructions failing

2. RESPOND (1-10 min):
   □ Identify last known good buffer
   □ Propose rollback via multisig
   □ Execute rollback if quorum available

3. VERIFY (10-30 min):
   □ Test critical instructions post-rollback
   □ Confirm error rate returns to baseline
   □ Document root cause

4. FIX (1-24h):
   □ Fix bug in local environment
   □ Full Guardian Pipeline on fixed version
   □ Re-deploy with additional testing
Decision Matrix
Table
Situation	Decision	Rationale
Security patch at 3 AM	Emergency pipeline, skip devnet	Speed > completeness for critical vulns
Feature request from PM	Standard pipeline, no skipping	Features can wait for proper testing
Dependency has CVE	Standard pipeline, expedited	Security patch, but test migration
User reports fund stuck	Hotfix pipeline, 24h turnaround	User funds > process rigidity
6 months since last upgrade	Mandatory full pipeline	Stale codebase = unknown risks
Authority key lost	Emergency closure + redeploy	No authority = no control
Version: 2026.06 | Lifecycle: Deploy → Operate → Upgrade → Deprecate → Sunset | Authority: Multisig at all times
