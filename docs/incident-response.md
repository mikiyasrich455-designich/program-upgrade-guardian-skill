Incident Response

When everything breaks at 3 AM, you do not think. You follow the checklist.

Table of Contents

Severity Levels

Response Team

Scenario: Authority Compromised

Scenario: Bricked Upgrade

Scenario: Active Fund Drainage

Scenario: Chain-Wide Incident

Scenario: Dependency CVE

Communication Templates

Post-Incident Review

Runbook Maintenance

Severity Levels

Table

Level	Name	Examples	Response Time	Escalation

SEV-1	Critical	Authority stolen, active drain, program bricked	5 min	All hands, CEO notified
SEV-2	High	>5% error rate, CU exhaustion, IDL mismatch	15 min	Engineering + Security
SEV-3	Medium	Deprecation dependency, performance degradation	1 hour	Engineering lead
SEV-4	Low	Monitoring false positive, documentation error	24 hours	On-call engineer
Severity Escalation Rules
plain
SEV-4 detected
    │
    ▼
Can it affect user funds?
    ├── YES ──▶ Escalate to SEV-3
    │
    └── NO
        │
        ▼
    Is it reproducible?
        ├── YES ──▶ Monitor, keep SEV-4
        │
        └── NO ──▶ Close as transient

SEV-3 detected
    │
    ▼
Is it spreading?
    ├── YES ──▶ Escalate to SEV-2
    │
    └── NO
        │
        ▼
    Can it become SEV-1?
        ├── YES ──▶ Escalate to SEV-2 (precautionary)
        │
        └── NO ──▶ Keep SEV-3, 1-hour checkpoint

SEV-2 detected
    │
    ▼
Is user funds at risk NOW?
    ├── YES ──▶ Escalate to SEV-1 immediately
    │
    └── NO
        │
        ▼
    Can it become SEV-1 within 1 hour?
        ├── YES ──▶ Escalate to SEV-1
        │
        └── NO ──▶ Keep SEV-2, 15-min checkpoints
Response Team
Table
Role	Responsibility	Primary	Backup
Incident Commander	Decision authority, communication	Tech Lead	CTO
Solana Engineer	On-chain actions, tx execution	Senior Engineer	Guardian Agent
Security Lead	Threat analysis, forensics	Security Lead	External auditor
Communications	User updates, social media	Community Manager	CEO
Legal	Regulatory, liability	General Counsel	External counsel
On-Call Rotation
plain
Week 1: Alice (Engineer) + Bob (Security)
Week 2: Carol (Engineer) + Dave (Security)
Week 3: Eve (Engineer) + Frank (Security)

Handoff: Every Monday 10 AM UTC
  - Review open SEV-3+ incidents
  - Verify pagerduty / Opsgenie access
  - Test emergency multisig signing (devnet)
Scenario: Authority Compromised
Detection
bash
# Automated check (runs every 1 min via cron)
CURRENT_AUTHORITY=$(solana program show $PROGRAM_ID | grep "Authority" | awk '{print $2}')
if [[ "$CURRENT_AUTHORITY" != "$EXPECTED_AUTHORITY" ]]; then
  echo "ALERT: Authority changed! $CURRENT_AUTHORITY"
  # Trigger PagerDuty / Slack
fi
Response Flow
plain
T+0:00  DETECT — Authority mismatch confirmed
        □ Verify on multiple RPC endpoints (Helius, QuickNode, Triton)
        □ Check transaction history for unauthorized SetUpgradeAuthority

T+0:02  PAGE — Incident Commander + Security Lead
        □ Slack: #incidents
        □ PagerDuty: SEV-1 page
        □ Phone call if no response in 2 min

T+0:05  ASSESS — Determine if compromise is active
        □ Is attacker deploying malicious code? Check recent deploy slots
        □ Is attacker draining funds? Check program token accounts

T+0:10  CONTAIN — If possible, revoke authority
        □ If multisig still has quorum: propose emergency SetUpgradeAuthority
        □ If hot wallet stolen: execute emergency program closure

T+0:15  COMMUNICATE — First user notification
        □ Twitter: "We are investigating a security incident. Do not interact with [program]."
        □ Discord: Pin message in #announcements
        □ Status page: Mark as "Major Incident"

T+0:30  RECOVER — Deploy new program or restore authority
        □ Option A: Restore authority via remaining multisig signers
        □ Option B: Close compromised program, deploy fresh with new ID
        □ Option C: If funds drained, engage law enforcement + insurance

T+1:00  VERIFY — Confirm containment
        □ Authority matches expected
        □ No unauthorized transactions in last 10 slots
        □ All user funds accounted for

T+24:00  POST-INCIDENT — Review and report
        □ Timeline reconstruction
        □ Root cause analysis
        □ Process improvements
        □ Public post-mortem (if user funds affected)
Exact Commands
bash
# Verify authority across multiple RPCs
for RPC in "https://api.mainnet-beta.solana.com"            "https://mainnet.helius-rpc.com/?api-key=$KEY"            "https://solana-api.project.nexus"; do
  echo "=== $RPC ==="
  solana program show $PROGRAM_ID --url $RPC | grep "Authority"
done

# Check recent deploys
solana transaction-history $PROGRAM_ID --limit 20 | grep -i "deploy\|upgrade\|authority"

# Emergency authority transfer (if you still control current authority)
solana program set-upgrade-authority $PROGRAM_ID \
  --new-upgrade-authority $EMERGENCY_MULTISIG \
  --keypair $REMAINING_SIGNER_KEYPAIR

# Emergency program closure (nuclear option)
solana program close $PROGRAM_ID \
  --recipient $RECOVERY_VAULT \
  --keypair $MULTISIG_SIGNER_KEYPAIR
Scenario: Bricked Upgrade
Detection
Table
Symptom	Cause	Confirm With
InstructionError: 0x0	Borsh deserialization failure	simulateTransaction on old accounts
AccountDidNotDeserialize	Account struct mismatch	getAccountInfo + manual decode
Error rate >50%	Critical instruction broken	Monitoring dashboard
Specific instruction failing	Logic error in new code	Isolate instruction in LiteSVM
Response Flow
plain
T+0:00  DETECT — Error rate spike after upgrade
        □ Confirm: program hash changed in last 10 slots
        □ Identify: which instruction is failing

T+0:02  PAGE — On-call engineer + Tech Lead

T+0:05  ISOLATE — Determine if rollback is safe
        □ Is old buffer still available? (Rule: keep 7 days)
        □ Will rollback break NEW accounts created post-upgrade?
        □ If rollback unsafe: prepare hotfix instead

T+0:10  DECISION — Rollback or hotfix?
        ┌─────────────────────────────────────────────────────────┐
        │ Rollback if:                                            │
        │   - Old buffer exists                                   │
        │   - No new accounts created with new schema             │
        │   - Failure is widespread (>20% of transactions)      │
        │                                                         │
        │ Hotfix if:                                              │
        │   - New accounts exist with new schema                  │
        │   - Failure is isolated to specific instruction         │
        │   - Rollback would cause more damage than fix           │
        └─────────────────────────────────────────────────────────┘

T+0:15  EXECUTE — Rollback or hotfix
        □ Rollback: Propose via Squads, execute after quorum
        □ Hotfix: Build fix, write buffer, deploy via Squads

T+0:30  VERIFY — Confirm recovery
        □ Error rate returns to baseline
        □ Critical instructions pass on Surfpool fork
        □ User reports stop

T+1:00  COMMUNICATE — All-clear
        □ Twitter: "Issue resolved. All systems operational."
        □ Discord: Unpin incident message
        □ Status page: Mark resolved

T+24:00  POST-INCIDENT — Why did this happen?
        □ Was Surfpool fork test skipped? (REQUIRE before mainnet)
        □ Was IDL check bypassed? (CI should block)
        □ Was multisig rushed? (48h review for non-emergency)
Rollback Transaction
bash
# Pre-staged rollback (do this BEFORE upgrade)
export OLD_BUFFER="BufferPubkeyFromPreviousVersion"
export PROGRAM_ID="YourProgramPubkey"
export SQUADS_VAULT="YourSquadsVaultPubkey"

# Stage rollback proposal in Squads (do not execute yet)
sqd program-upgrade propose \
  --program-id $PROGRAM_ID \
  --buffer $OLD_BUFFER \
  --multisig $SQUADS_VAULT \
  --draft-only  # Save as draft, execute only if needed

# If rollback needed:
sqd program-upgrade execute --proposal <PROPOSAL_PUBKEY>
Scenario: Active Fund Drainage
Detection
bash
# Monitor token account balances
for TOKEN_ACCOUNT in "$VAULT_1" "$VAULT_2" "$VAULT_3"; do
  BALANCE=$(spl-token balance $TOKEN_ACCOUNT --url mainnet-beta)
  echo "$TOKEN_ACCOUNT: $BALANCE"
  # Alert if balance changes >threshold in <5 min window
done

# Monitor suspicious transaction patterns
solana transaction-history $PROGRAM_ID --limit 100 | \
  awk '/transfer/ {print $0}' | \
  sort | uniq -c | sort -rn | head -20
Response Flow
plain
T+0:00  DETECT — Unusual outbound transfer pattern
        □ Verify: Is this a legitimate user withdrawal?
        □ Check: Transaction signatures against known users

T+0:01  CONFIRM — Is this an attack?
        □ Pattern: Same instruction, many accounts, rapid succession
        □ Amount: Draining significant % of TVL
        □ Source: Unknown wallets, no prior interaction

T+0:02  PAGE — SEV-1, all hands

T+0:03  CONTAIN — Emergency pause
        □ If program has pause instruction: execute immediately
        □ If no pause: emergency authority transfer to offline key
        □ If no authority control: coordinate with validators (extreme)

T+0:05  TRACE — Identify attack vector
        □ Which instruction is being exploited?
        □ What validation is missing?
        □ Is this a known vulnerability pattern?

T+0:10  COMMUNICATE — User warning
        □ "DO NOT INTERACT WITH [PROGRAM]. Active security incident."
        □ All channels simultaneously

T+0:30  PATCH — Deploy fix
        □ If pause worked: unpause after fix deployed
        □ If no pause: new program deployment with fix
        □ If funds already drained: engage recovery efforts

T+1:00  VERIFY — Attack stopped
        □ No new suspicious transactions
        □ Fix instruction passes all tests
        □ User funds secure (remaining)

T+24:00  RECOVERY — Fund recovery / compensation
        □ Calculate total loss
        □ Engage insurance (if applicable)
        □ Plan compensation (if protocol fault)
        □ Law enforcement report (if criminal)
Emergency Pause Instruction
rust
#[program]
pub mod my_program {
    use super::*;

    pub fn emergency_pause(ctx: Context<AdminOnly>) -> Result<()> {
        require!(
            ctx.accounts.authority.key() == EMERGENCY_ADMIN,
            ErrorCode::Unauthorized
        );

        let state = &mut ctx.accounts.program_state;
        state.paused = true;
        state.pause_timestamp = Clock::get()?.unix_timestamp;

        msg!("EMERGENCY PAUSE activated at slot {}", Clock::get()?.slot);
        Ok(())
    }

    pub fn emergency_unpause(ctx: Context<AdminOnly>) -> Result<()> {
        // Same authority check
        // Can only unpause after 24h cooldown (prevent flip-flopping)
        let state = &ctx.accounts.program_state;
        require!(
            Clock::get()?.unix_timestamp > state.pause_timestamp + 86_400,
            ErrorCode::PauseCooldown
        );

        ctx.accounts.program_state.paused = false;
        Ok(())
    }

    // Every user-facing instruction checks pause
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(!ctx.accounts.program_state.paused, ErrorCode::ProgramPaused);
        // ... deposit logic
    }
}
Scenario: Chain-Wide Incident
Table
Incident Type	Impact	Response
Solana network halt	All transactions stop	Wait for validator restart; monitor @solanastatusus
Major validator outage	Degraded performance	Increase priority fees; reduce instruction complexity
Hard fork	State divergence	Verify program on both forks; follow validator majority
Governance attack	Protocol parameters changed	If DAO-governed: participate in emergency vote
Bridge hack (Wormhole/etc)	Cross-chain funds at risk	Pause bridge interactions; audit exposure
Scenario: Dependency CVE
bash
# Check for known vulnerabilities
cargo audit

# If CVE found in anchor-lang, solana-program, etc.:
# 1. Assess: Is the vulnerable code path reachable from your program?
# 2. If YES: Plan upgrade within 48 hours
# 3. If NO: Plan upgrade within 2 weeks (standard cycle)

# Emergency dependency upgrade
# DO NOT just bump version. Run full Guardian Pipeline.
# Dependency changes can affect Borsh serialization.
Communication Templates
SEV-1 Initial Alert (Twitter)
plain
🚨 SECURITY INCIDENT — [Program Name]

We have detected an active security incident affecting [Program].

IMMEDIATE ACTION REQUIRED:
→ Do NOT interact with [Program]
→ Do NOT deposit funds
→ Withdrawals may be paused

We are investigating with our security team.
Updates every 30 min: [status page link]

Incident ID: [UUID]
SEV-1 Resolution (Twitter)
plain
✅ RESOLVED — [Program Name] Incident [ID]

The security incident has been contained.

What happened: [1-2 sentence summary]
What we did: [Containment action]
User impact: [Funds affected / no impact]
Next steps: [Post-mortem date, compensation if applicable]

Full report: [link]
Internal Slack Update (Every 30 min during SEV-1)
plain
[SEV-1] [Program] Incident Update — T+[HH:MM]

Status: [CONTAINED / INVESTIGATING / RECOVERING]
Impact: [User funds / TVL / transactions affected]
Actions taken: [List]
Blockers: [What we need]
Next update: [Time]
IC: [Name]
Post-Incident Review
Template
Markdown
Fullscreen 
Download 
Fit
Code
Preview
Date: [YYYY-MM-DD]
Severity: [SEV-1/2/3/4]
Duration: [HH:MM]
Program: [ID]
Incident Commander: [Name]
Metadata
Time	Event	Source
T+0	Detection	[How detected]
T+2	Page sent	PagerDuty
T+5	Containment	[Action taken]
T+30	Resolution	[How fixed]
Timeline
Root Cause
User funds: [Amount / None]
TVL: [Change %]
Reputation: [Assessment]
Impact
What Went Well
What Went Wrong
#	Action	Owner	Due
1			
2			
3			
Action Items
Lessons Learned
Post-Incident Review: [Incident ID]
Runbook Maintenance
Table
Task	Frequency	Owner
Verify emergency contacts	Monthly	Operations
Test multisig emergency signing	Monthly	Security
Update RPC endpoints	Quarterly	Engineering
Review and rotate emergency keys	Quarterly	Security
Simulate each scenario	Quarterly	Engineering
Update communication templates	After every incident	Communications
Audit on-call rotation	Monthly	Operations
Version: 2026.06 | When in doubt: contain first, investigate second, communicate third.
