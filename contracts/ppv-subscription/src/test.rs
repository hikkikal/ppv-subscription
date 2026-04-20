#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Env, Address, String};

    fn setup() -> (Env, PPVContractClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, PPVContract);
        let client = PPVContractClient::new(&env, &contract_id);
        (env, client)
    }

    #[test]
    fn test_add_content() {
        let (env, client) = setup();
        let owner = Address::generate(&env);

        let id = client.add_content(
            &String::from_str(&env, "Intro to Soroban"),
            &owner,
            &5_000_000i128, // 0.5 XLM
        );
        assert_eq!(id, 1);

        let content = client.get_content(&id);
        assert_eq!(content.content_id, 1);
        assert_eq!(content.price, 5_000_000);
        assert!(content.is_active);
    }

    #[test]
    fn test_subscribe_and_check_access() {
        let (env, client) = setup();
        let owner = Address::generate(&env);
        let viewer = Address::generate(&env);

        let id = client.add_content(
            &String::from_str(&env, "Advanced DeFi"),
            &owner,
            &10_000_000i128, // 1 XLM
        );

        // No access before subscribing
        assert!(!client.check_access(&viewer, &id));

        // Subscribe for 10_000 ledgers
        client.subscribe(&viewer, &id, &10_000u64);

        // Should have access now
        assert!(client.check_access(&viewer, &id));
    }

    #[test]
    fn test_update_price() {
        let (env, client) = setup();
        let owner = Address::generate(&env);

        let id = client.add_content(
            &String::from_str(&env, "Stellar Workshop"),
            &owner,
            &5_000_000i128,
        );

        client.update_price(&owner, &id, &8_000_000i128);

        let content = client.get_content(&id);
        assert_eq!(content.price, 8_000_000);
    }

    #[test]
    fn test_renew_subscription() {
        let (env, client) = setup();
        let owner = Address::generate(&env);
        let viewer = Address::generate(&env);

        let id = client.add_content(
            &String::from_str(&env, "Blockchain Basics"),
            &owner,
            &2_000_000i128,
        );

        client.subscribe(&viewer, &id, &5_000u64);
        client.renew_subscription(&viewer, &id, &5_000u64);

        let sub = client.get_subscription(&viewer, &id);
        assert!(sub.is_active);
        // expires_at should be original + 5000
    }

    #[test]
    fn test_cancel_subscription() {
        let (env, client) = setup();
        let owner = Address::generate(&env);
        let viewer = Address::generate(&env);

        let id = client.add_content(
            &String::from_str(&env, "NFT Deep Dive"),
            &owner,
            &3_000_000i128,
        );

        client.subscribe(&viewer, &id, &10_000u64);
        assert!(client.check_access(&viewer, &id));

        client.cancel_subscription(&viewer, &id);
        assert!(!client.check_access(&viewer, &id));
    }

    #[test]
    fn test_remove_content() {
        let (env, client) = setup();
        let owner = Address::generate(&env);

        let id = client.add_content(
            &String::from_str(&env, "Old Course"),
            &owner,
            &1_000_000i128,
        );

        client.remove_content(&owner, &id);

        let content = client.get_content(&id);
        assert!(!content.is_active);
    }
}