# Safe Upgrade Commands

## Buffer Workflow (Safest Method)
```bash
# 1. Write program to buffer
solana program write-buffer ./target/deploy/my_program.so --url https://mainnet.helius-rpc.com/?api-key=YOUR_KEY

# 2. Set buffer authority to multisig
solana program set-buffer-authority <BUFFER_PUBKEY> \
  --new-buffer-authority <MULTISIG_ADDRESS> \
  --url https://mainnet.helius-rpc.com/?api-key=YOUR_KEY
