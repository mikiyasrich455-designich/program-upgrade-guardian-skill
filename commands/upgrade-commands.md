# Safe Upgrade Commands (2026 Best Practices)

## 1. Buffer Workflow - The Safest Method (Recommended for Mainnet)

```bash
# Step 1: Build your program
anchor build --verifiable

# Step 2: Write program to buffer (upload new code)
solana program write-buffer ./target/deploy/my_program.so \
  --url https://mainnet.helius-rpc.com/?api-key=YOUR_HELIUS_KEY

# Step 3: Set buffer authority to multisig (VERY IMPORTANT SAFETY STEP)
solana program set-buffer-authority <BUFFER_PUBKEY> \
  --new-buffer-authority <YOUR_MULTISIG_PDA> \
  --url https://mainnet.helius-rpc.com/?api-key=YOUR_HELIUS_KEY

# Step 4: Final upgrade (executed by multisig)
solana program deploy --program-id <YOUR_PROGRAM_ID> --buffer <BUFFER_PUBKEY>
