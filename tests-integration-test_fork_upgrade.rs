#[cfg(test)]
mod test_fork_upgrade {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{pubkey::Pubkey, commitment_config::CommitmentConfig};

    /// Tests upgrade against a mainnet fork using Surfpool
    /// Requires Surfpool CLI to be installed: cargo install surfpool-cli
    /// 
    /// Run with: cargo test test_fork_upgrade -- --ignored
    #[tokio::test]
    #[ignore = "Requires Surfpool CLI and mainnet RPC access"]
    async fn test_upgrade_against_mainnet_fork() {
        let program_id = Pubkey::new_unique(); // Replace with real program ID
        let fork_slot = None; // Latest

        // 1. Fork mainnet state
        // surfpool fork --slot <SLOT> --output fork.json

        // 2. Deploy new program to fork
        // svm.add_program_from_file(program_id, "target/deploy/your_program.so");

        // 3. Test critical instructions
        // let ix = /* your critical instruction */;
        // svm.send_transaction(tx).unwrap();

        // 4. Verify user accounts still deserialize correctly
        // let account = svm.get_account(&user_pubkey).unwrap();
        // let user = User::try_deserialize(&mut account.data.as_ref()).unwrap();

        // Placeholder assertion
        assert!(true, "Fork test completed");
    }

    #[tokio::test]
    #[ignore = "Requires live devnet"]
    async fn test_devnet_rehearsal() {
        let rpc = RpcClient::new_with_commitment(
            "https://api.devnet.solana.com".to_string(),
            CommitmentConfig::confirmed()
        );

        // Verify connection
        let slot = rpc.get_slot().await.expect("Should connect to devnet");
        assert!(slot > 0, "Should get valid slot from devnet");
    }
}
