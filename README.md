# 🛡️ Program Upgrade Guardian

> **The only skill you need for fearless Solana program upgrades.**
> 
> One skill. **8 AI tools.** Zero bricked programs.

<p align="center">
  <img src="https://img.shields.io/badge/Anchor-0.30.0-9945FF?style=for-the-badge&logo=anchor" />
  <img src="https://img.shields.io/badge/Solana-1.18.0-14F195?style=for-the-badge&logo=solana" />
  <img src="https://img.shields.io/badge/AI_Tools-8-FF6B35?style=for-the-badge&logo=openai" />
  <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" />
</p>

---

## 🔥 The Problem

Upgrading live Solana programs is **terrifying**.

| What Goes Wrong | The Damage |
|----------------|------------|
| 💥 Corrupt user data | **Permanent loss** — no undo button on-chain |
| 🧱 Brick the program | **Dead contract** — users locked out forever |
| 🔒 Lock funds | **TVL frozen** — protocol becomes a vault with no key |
| 😤 Lose community trust | **Reputation death** — one tweet away from doom |

**One small mistake. Catastrophic consequences.**

---

## ✨ The Solution

**Program Upgrade Guardian** transforms **any AI** into a paranoid senior Solana engineer that holds your hand through every upgrade — from discovery to rollback.

### What You Get

| Feature | Why It Slaps |
|---------|-------------|
| 🔄 **Full Guardian Pipeline** | Discovery → Analysis → Test → Migrate → Deploy → Rollback |
| 🔍 **Borsh Layout Drift Detection** | Catches account corruption **before** it hits mainnet |
| 🛡️ **Buffer + Multisig Workflow** | Safe, auditable deployments — no cowboy deploys |
| 🧪 **Mainnet Forking** | Test against **real accounts** with Surfpool + LiteSVM |
| 📋 **State Migration Blueprints** | 3 production-ready Rust patterns — copy, paste, deploy |
| ⚠️ **14 Safety Rules** | Non-negotiable. Auto-blocked if violated. |
| 🧹 **Post-Upgrade Cleanup** | Rent reclamation + 24h health monitoring |

---

## 🤖 Universal AI Support

**We don't lock you into one AI.** Pick your weapon:

| # | Tool | Path | Best For |
|---|------|------|----------|
| 1 | 🟣 **Claude** | `.claude/skills/program-upgrade-guardian/` | General guardian |
| 2 | 🖥️ **Cursor** | `.cursor/skills/program-upgrade-guardian/` | IDE copilot |
| 3 | 🤖 **Codex** | `.codex/skills/program-upgrade-guardian/` | Program generation |
| 4 | 🔷 **Gemini** | `.gemini/skills/program-upgrade-guardian/` | Architecture |
| 5 | 🐙 **GitHub Copilot** | `.github/skills/program-upgrade-guardian/` | VS Code autocomplete |
| 6 | 🌊 **Windsurf** | `.windsurf/skills/program-upgrade-guardian/` | Agentic coding |
| 7 | 💬 **ChatGPT** | `.chatgpt/skills/program-upgrade-guardian/` | Interactive help |
| 8 | 🐋 **DeepSeek** | `.deepseek/skills/program-upgrade-guardian/` | Rust coding |

> **One skill. Eight platforms. Same safety.**  
> See [`SKILLS_INDEX.md`](SKILLS_INDEX.md) for the full breakdown.

---

## 🚀 Quick Start

### Prerequisites

| Tool | Min Version | Check |
|------|-------------|-------|
| Anchor | 0.30.0 | `anchor --version` |
| Solana CLI | 1.18.0 | `solana --version` |
| Squads CLI | Latest | `sqd --version` |
| Node.js | 20.x | `node --version` |
| Rust | 1.75+ | `rustc --version` |

### Install

```bash
git clone https://github.com/mikiyasrich455-designich/program-upgrade-guardian-skill.git
cd program-upgrade-guardian-skill
python3 install.py  # Read-only checker. Safe.
```

> **Note:** `install.py` is read-only. It checks your env. It does **not** modify your system.

### Use It

Just ask your AI:

> *"Help me safely upgrade my Anchor program on mainnet"*

> *"I want to add a new field to my User account — what's the safe way?"*

> *"Transfer upgrade authority to Squads multisig"*

> *"What if my upgrade bricks the program?"*

The skill auto-loads the right agent and walks you through the pipeline.

---

## ⚙️ How It Works

### The Guardian Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Discovery  │───▶│   Analysis  │───▶│ Local Test  │───▶│  Migration  │
│   (Pull)    │    │(Drift+Risk) │    │(Surfpool+   │    │   (Plan)    │
│             │    │             │    │  LiteSVM)   │    │             │
└─────────────┘    └─────────────┘    └─────────────┘    └──────┬──────┘
                                                                  │
┌─────────────┐    ┌─────────────┐    ┌─────────────┐            │
│  Rollback   │◀───│  Mainnet    │◀───│   Devnet    │◀───────────┘
│  & Cleanup  │    │  Upgrade    │    │  Rehearsal  │
│             │    │(Buffer+MSIG)│    │             │
└─────────────┘    └─────────────┘    └─────────────┘
```

### Safety-First Rules

| # | Rule | Enforced |
|---|------|----------|
| 1 | Append fields **only** at end of structs | ✅ Auto-block |
| 2 | Prefer `Option<T>` for new string fields | ✅ Auto-block |
| 3 | **Never** use `solana program deploy` on mainnet | ✅ Auto-block |
| 4 | Always move authority to **multisig** before upgrade | ✅ Auto-block |
| 5 | Risk score **≥ 7** → second human review required | ⚠️ Warn |
| 6 | All tests **must pass** before mainnet | ✅ Auto-block |
| 7 | Verify **Borsh layout** compatibility | ✅ Auto-block |
| 8 | **Never** remove existing fields from accounts | ✅ Auto-block |
| 9 | Always have a **rollback plan** | ⚠️ Warn |
| 10 | Test on **devnet** before mainnet | ✅ Auto-block |
| 11 | Verify **buffer hash** matches intended program | ✅ Auto-block |
| 12 | Check **compute unit** limits after upgrade | ⚠️ Warn |
| 13 | Ensure **rent exemption** for new accounts | ✅ Auto-block |
| 14 | Validate all **CPI program IDs** | ✅ Auto-block |

### Agent Personas

| Agent | Role | Trigger |
|-------|------|---------|
| `upgrade-warden` | 🛡️ Main Guardian (orchestrates pipeline) | Default |
| `risk-analyst` | 🔴 Quantifies danger (scores 1-10) | Risk check or score > 5 |
| `migration-engineer` | 🔧 Writes bulletproof migration code | State migration needed |

---

## 🧪 Testing Pyramid

| Layer | Tool | Purpose |
|-------|------|---------|
| Unit | LiteSVM | Instruction logic, migration math |
| Fork | Surfpool | Real mainnet account data |
| Staging | Devnet | Full deployment + multisig rehearsal |
| Shadow | Mainnet RPC | Pre-launch schema validation |
| Monitor | Custom scripts | 24h post-upgrade health checks |

See [`tests/upgrade-test-suite.md`](tests/upgrade-test-suite.md) for full details.

---

## 📦 Dependencies

**Rust**
```toml
[dependencies]
anchor-lang = "0.30.0"
anchor-spl = "0.30.0"

[dev-dependencies]
litesvm = "0.6"
solana-sdk = "~1.18"
```

**TypeScript**
```json
{
  "dependencies": {
    "@coral-xyz/anchor": "^0.30.0",
    "@solana/web3.js": "^1.91.0"
  },
  "devDependencies": {
    "typescript": "^5.4.0",
    "ts-node": "^10.9.0"
  }
}
```

---

## 🏗️ Repository Structure

```
program-upgrade-guardian-skill/
├── SKILL.md                          # Main skill definition
├── README.md                         # This file
├── SKILLS_INDEX.md                   # Multi-AI tool index
├── LICENSE                           # MIT
├── install.py                        # Safe setup checker
│
├── .claude/skills/program-upgrade-guardian/SKILL.md
├── .cursor/skills/program-upgrade-guardian/SKILL.md
├── .codex/skills/program-upgrade-guardian/SKILL.md
├── .gemini/skills/program-upgrade-guardian/SKILL.md
├── .github/skills/program-upgrade-guardian/SKILL.md
├── .windsurf/skills/program-upgrade-guardian/SKILL.md
├── .chatgpt/skills/program-upgrade-guardian/SKILL.md
├── .deepseek/skills/program-upgrade-guardian/SKILL.md
│
├── agents/
│   └── guardian-agent.md             # Agent personas & selection
├── commands/
│   └── upgrade-commands.md           # Copy-paste CLI commands
├── docs/
│   ├── incident-response.md          # SEV-1 to SEV-4 playbooks
│   └── program-lifecycle.md        # Deploy → Upgrade → Sunset
├── rules/
│   └── safety-rules.md             # 14 non-negotiable rules
├── templates/
│   └── migration-template.rs         # 3 migration patterns
└── tests/
    └── upgrade-test-suite.md       # Test pyramid
```

---

## 🎯 Why This Wins

| Other Skills | Program Upgrade Guardian |
|-------------|---------------------------|
| Single AI tool | ✅ **8 AI tools** — use what you already use |
| Generic Solana dev | ✅ **Upgrade-specific** — laser-focused on the scariest task |
| No safety enforcement | ✅ **14 auto-block rules** — violations caught before deployment |
| No rollback plan | ✅ **Built-in rollback** — every upgrade has an escape hatch |
| No testing strategy | ✅ **5-layer test pyramid** — from unit to 24h monitoring |
| No migration patterns | ✅ **3 production templates** — copy, paste, deploy |
| No incident response | ✅ **SEV-1 to SEV-4 playbooks** — know exactly what to do when shit hits the fan |

---

## 🤝 Contributing

1. Fork it
2. Branch: `git checkout -b feature/amazing-thing`
3. Commit: `git commit -m 'Add amazing thing'`
4. Push: `git push origin feature/amazing-thing`
5. Open PR

All PRs must pass IDL compatibility check + include tests for new migration patterns.

---

## 📜 License

MIT — see [`LICENSE`](LICENSE)

---

## 🙏 Acknowledgments

- Built for **Solana AI Kit Bounty 2026**
- Inspired by real mainnet upgrade incidents
- Safety rules validated against Anchor 0.30+ & Solana CLI 1.18+

---

<p align="center">
  <b>Version</b>: 2026.06 | 
  <b>Stack</b>: Anchor + Surfpool + LiteSVM | 
  <b>Authority</b>: Multisig-required on mainnet | 
  <b>AI Tools</b>: 8
</p>
