#[cfg(test)]
mod test_migration_append {
    use litesvm::LiteSVM;
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
    use solana_program::pubkey::Pubkey;
    use anchor_lang::{AccountDeserialize, InstructionData};

    #[test]
    fn test_append_fields_migration() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

        // Deploy program
        let program_id = Pubkey::new_unique();
        svm.add_program_from_file(program_id, "target/deploy/your_program.so").unwrap();

        // Create old-style user account (without new fields)
        let user_keypair = Keypair::new();
        let old_user_data = vec![
            // discriminator (8 bytes) - placeholder
            0,0,0,0,0,0,0,0,
            // owner: Pubkey (32 bytes)
            payer.pubkey().to_bytes().to_vec(),
            // balance: u64 (8 bytes) LE
            100u64.to_le_bytes().to_vec(),
        ].concat();

        svm.create_account(&user_keypair.pubkey(), old_user_data.len() as u64, program_id).unwrap();
        svm.set_account_data(&user_keypair.pubkey(), old_user_data).unwrap();

        // Execute migration
        let ix = solana_sdk::instruction::Instruction {
            program_id,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(user_keypair.pubkey(), false),
                solana_sdk::instruction::AccountMeta::new_readonly(payer.pubkey(), true),
            ],
            data: vec![], // Migration instruction discriminator
        };

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[&payer],
            svm.latest_blockhash(),
        );

        svm.send_transaction(tx).unwrap();

        // Verify account still exists and has data
        let account = svm.get_account(&user_keypair.pubkey()).unwrap();
        assert!(account.data.len() >= old_user_data.len());
    }

    #[test]
    fn test_migration_idempotent() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

        let program_id = Pubkey::new_unique();
        svm.add_program_from_file(program_id, "target/deploy/your_program.so").unwrap();

        let user_keypair = Keypair::new();
        let user_data = vec![0u8; 48]; // discriminator + owner + balance
        svm.create_account(&user_keypair.pubkey(), user_data.len() as u64, program_id).unwrap();
        svm.set_account_data(&user_keypair.pubkey(), user_data).unwrap();

        let ix = solana_sdk::instruction::Instruction {
            program_id,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(user_keypair.pubkey(), false),
                solana_sdk::instruction::AccountMeta::new_readonly(payer.pubkey(), true),
            ],
            data: vec![],
        };

        // First migration
        let tx1 = Transaction::new_signed_with_payer(
            &[ix.clone()],
            Some(&payer.pubkey()),
            &[&payer],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx1).unwrap();

        // Second migration - should succeed (idempotent)
        let tx2 = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer.pubkey()),
            &[&payer],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx2).unwrap(); // Should not panic
    }
}
