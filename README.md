# Program Upgrade Guardian Skill

> Production-Ready Skill for Solana AI Kit — A meticulous senior Solana engineer that safely guides builders through live program upgrades, state migrations, authority transfers, and zero-downtime deployments.

---

## The Problem

Upgrading live Solana programs remains one of the scariest and highest-risk tasks in the ecosystem.

One small mistake can:

- Corrupt user data forever
- Brick the program
- Lock funds
- Damage community trust

---

## The Solution

Program Upgrade Guardian turns any AI into a careful, paranoid senior Solana engineer that guides you safely through every step using 2026 best practices.

- Full Guardian Pipeline (Discovery → Analysis → Testing → Upgrade → Rollback)
- Automatic Borsh layout drift detection
- Safe buffer + multisig workflow
- Realistic mainnet forking with Surfpool + LiteSVM
- State migration blueprints + rollback plans
- Strict safety rules & red flags
- Post-upgrade cleanup & rent reclamation

---

## Repository Structure

```plain
program-upgrade-guardian-skill/
├── SKILL.md                          # Main skill definition
├── README.md                         # This file
├── LICENSE                           # MIT License
├── install.sh                        # Dependency checker & setup helper
├── .github/
│   └── workflows/
│       └── idl-check.yml             # CI: IDL compatibility gate
├── agents/
│   └── guardian-agent.md             # Agent personas & tone specs
├── commands/
│   └── upgrade-commands.md           # Exact copy-paste CLI commands
├── docs/
│   ├── incident-response.md          # Emergency playbooks (SEV-1 to SEV-4)
│   └── program-lifecycle.md          # Deploy → Operate → Upgrade → Sunset
├── rules/
│   └── safety-rules.md               # Non-negotiable safety matrix
├── templates/
│   └── migration-template.rs         # Copy-paste Rust migration patterns
└── tests/
    └── upgrade-test-suite.md         # LiteSVM → Surfpool → Devnet → Mainnet
```

---

## Quick Start

### Prerequisites

| Tool        | Minimum Version | Check               |
|-------------|-----------------|---------------------|
| Anchor      | 0.30.0          | `anchor --version`  |
| Solana CLI  | 1.18.0          | `solana --version`  |
| Squads CLI  | Latest          | `sqd --version`     |
| Node.js     | 20.x            | `node --version`    |
| Rust        | 1.75+           | `rustc --version`   |

### Installation

```bash
# Clone the skill
git clone https://github.com/mikiyasrich455-designich/program-upgrade-guardian-skill.git
cd program-upgrade-guardian-skill

# Run the setup checker
chmod +x install.sh
./install.sh
```

### Usage

Just ask your AI assistant:

- `"Help me safely upgrade my Anchor program on mainnet"`
- `"I want to add a new field to my User account — what's the safe way?"`
- `"Transfer upgrade authority to Squads multisig"`
- `"What if my upgrade bricks the program?"`

The skill automatically loads the right agent (`upgrade-warden`, `risk-analyst`, or `migration-engineer`) and walks you through the Guardian Pipeline.

---

## How It Works

### The Guardian Pipeline

```plain
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

- Always append fields at the end of structs
- Prefer `Option<T>` for new string fields
- Never use direct `solana program deploy` on mainnet
- Always move authority to multisig before production upgrade

### Agent Personas

| Agent               | Role                                      | Trigger                    |
|---------------------|-------------------------------------------|----------------------------|
| `upgrade-warden`    | Main Guardian (orchestrates pipeline)     | Default for all requests   |
| `risk-analyst`      | Quantifies danger (scores 1-10)           | Risk check or score > 5    |
| `migration-engineer`| Writes bulletproof migration code         | State migration needed     |

---

## Testing

This skill includes a complete test pyramid:

| Layer   | Tool           | Purpose                                  |
|---------|----------------|------------------------------------------|
| Unit    | LiteSVM        | Instruction logic, migration math        |
| Fork    | Surfpool       | Real mainnet account data                |
| Staging | Devnet         | Full deployment + multisig rehearsal     |
| Shadow  | Mainnet RPC    | Pre-launch schema validation             |
| Monitor | Custom scripts | 24h post-upgrade health checks           |

See `tests/upgrade-test-suite.md` for full details.

---

## Dependencies

**Rust** (for program development)

```toml
[dependencies]
anchor-lang = "0.30.0"
anchor-spl = "0.30.0"

[dev-dependencies]
litesvm = "0.6"
solana-sdk = "~1.18"
```

**TypeScript** (for CI and client scripts)

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

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-thing`)
3. Commit your changes (`git commit -m 'Add amazing thing'`)
4. Push to the branch (`git push origin feature/amazing-thing`)
5. Open a Pull Request

All contributions must pass the IDL compatibility check and include tests for any new migration patterns.

---

## License

MIT License — see `LICENSE` for details.

---

## Acknowledgments

- Built for the Solana AI Kit Bounty 2026
- Inspired by real mainnet upgrade incidents and the lessons learned from them
- Safety rules validated against Anchor 0.30+ and Solana CLI 1.18+

---

> Version: 2026.06 | Stack: Anchor + Surfpool + LiteSVM | Authority: Multisig-required on mainnet
