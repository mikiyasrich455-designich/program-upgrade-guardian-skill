#[cfg(test)]
mod test_migration_resize {
    use litesvm::LiteSVM;
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
    use solana_program::{pubkey::Pubkey, rent::Rent};

    #[test]
    fn test_resize_with_rent_calculation() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

        let program_id = Pubkey::new_unique();
        svm.add_program_from_file(program_id, "target/deploy/your_program.so").unwrap();

        // Create undersized vault account (old size: 49 bytes + 8 discriminator = 57)
        let vault_keypair = Keypair::new();
        let old_size = 57;
        let new_size = 90;

        let rent = Rent::default();
        let old_rent = rent.minimum_balance(old_size);

        svm.create_account(&vault_keypair.pubkey(), old_rent, program_id).unwrap();

        // Set old vault data
        let old_vault_data = vec![0u8; old_size];
        svm.set_account_data(&vault_keypair.pubkey(), old_vault_data).unwrap();

        // Execute resize migration
        let ix = solana_sdk::instruction::Instruction {
            program_id,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(vault_keypair.pubkey(), false),
                solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new_readonly(payer.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: vec![1], // Resize instruction discriminator
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[&payer],
            svm.latest_blockhash(),
        );

        svm.send_transaction(tx).unwrap();

        // Verify account size increased
        let account = svm.get_account(&vault_keypair.pubkey()).unwrap();
        assert!(account.data.len() >= new_size, 
            "Account should be resized to at least {} bytes, got {}", 
            new_size, account.data.len());

        // Verify rent was paid
        let new_minimum = rent.minimum_balance(new_size);
        assert!(account.lamports >= new_minimum,
            "Account should have at least {} lamports for rent, got {}",
            new_minimum, account.lamports);
    }

    #[test]
    fn test_resize_fails_without_enough_rent() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        // Give payer barely enough for fees but not for resize
        svm.airdrop(&payer.pubkey(), 10_000).unwrap();

        let program_id = Pubkey::new_unique();
        svm.add_program_from_file(program_id, "target/deploy/your_program.so").unwrap();

        let vault_keypair = Keypair::new();
        let old_size = 57;
        let rent = Rent::default();
        let old_rent = rent.minimum_balance(old_size);

        svm.create_account(&vault_keypair.pubkey(), old_rent, program_id).unwrap();

        let ix = solana_sdk::instruction::Instruction {
            program_id,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(vault_keypair.pubkey(), false),
                solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new_readonly(payer.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
            data: vec![1],
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[&payer],
            svm.latest_blockhash(),
        );

        // Should fail due to insufficient rent
        let result = svm.send_transaction(tx);
        assert!(result.is_err(), "Should fail when payer has insufficient funds for resize");
    }
}
