#[cfg(test)]
mod test_borsh_layout {
    use anchor_lang::prelude::*;
    use anchor_lang::AnchorSerialize;

    // Mock structs for testing layout compatibility
    #[account]
    pub struct UserV1 {
        pub owner: Pubkey,
        pub balance: u64,
    }

    #[account]
    pub struct UserV2 {
        pub owner: Pubkey,
        pub balance: u64,
        pub nickname: Option<String>,
        pub created_at: i64,
    }

    #[test]
    fn test_user_v1_size() {
        let user = UserV1 {
            owner: Pubkey::default(),
            balance: 0,
        };
        let serialized = user.try_to_vec().unwrap();
        // 32 (Pubkey) + 8 (u64) = 40 bytes
        assert_eq!(serialized.len(), 40, "UserV1 should be 40 bytes");
    }

    #[test]
    fn test_user_v2_size() {
        let user = UserV2 {
            owner: Pubkey::default(),
            balance: 0,
            nickname: None,
            created_at: 0,
        };
        let serialized = user.try_to_vec().unwrap();
        // V2 should be larger than V1
        assert!(serialized.len() > 40, "UserV2 should be larger than UserV1");
    }

    #[test]
    fn test_backward_compatibility_simulation() {
        // Simulate: old account data (V1) should be prefix of new struct
        let old_data = UserV1 {
            owner: Pubkey::new_unique(),
            balance: 1_000_000,
        }.try_to_vec().unwrap();

        // In real migration, V2 would be deserialized from extended account
        // Here we just verify the size relationship
        let new_data = UserV2 {
            owner: Pubkey::new_unique(),
            balance: 1_000_000,
            nickname: Some("test".to_string()),
            created_at: 1234567890,
        }.try_to_vec().unwrap();

        assert!(new_data.len() > old_data.len(), 
            "New version must be larger for append-only migration");
    }

    #[test]
    fn test_struct_field_order_unchanged() {
        // Verify first 40 bytes of V2 match V1 layout when fields are default
        let owner = Pubkey::new_unique();
        let balance = 42u64;

        let v1 = UserV1 { owner, balance }.try_to_vec().unwrap();
        let v2 = UserV2 { owner, balance, nickname: None, created_at: 0 }.try_to_vec().unwrap();

        // First 40 bytes should be identical (same fields in same order)
        assert_eq!(&v1[..], &v2[..40], "First 40 bytes must match for backward compatibility");
    }
}
