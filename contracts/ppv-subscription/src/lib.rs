#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String};

// ─── Data Types ──────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct Content {
    pub content_id: u64,
    pub title:      String,
    pub owner:      Address,
    pub price:      i128,   // in stroops (1 XLM = 10_000_000)
    pub is_active:  bool,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Subscription {
    pub subscriber: Address,
    pub content_id: u64,
    pub expires_at: u64,    // ledger sequence number
    pub is_active:  bool,
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct PPVContract;

#[contractimpl]
impl PPVContract {

    // ── CREATE ─ Register new payable content ─────────────────────────────────
    pub fn add_content(
        env: Env,
        title: String,
        owner: Address,
        price: i128,
    ) -> u64 {
        owner.require_auth();
        let id = env
            .storage()
            .instance()
            .get(&symbol_short!("NEXT_ID"))
            .unwrap_or(0u64)
            + 1;
        let content = Content {
            content_id: id,
            title,
            owner,
            price,
            is_active: true,
        };
        env.storage().persistent().set(&id, &content);
        env.storage()
            .instance()
            .set(&symbol_short!("NEXT_ID"), &id);
        id
    }

    // ── CREATE ─ Subscribe to content (pay-per-view) ──────────────────────────
    pub fn subscribe(
        env: Env,
        subscriber: Address,
        content_id: u64,
        duration: u64, // in ledgers (~5 seconds each)
    ) {
        subscriber.require_auth();
        let content: Content = env
            .storage()
            .persistent()
            .get(&content_id)
            .unwrap();
        assert!(content.is_active, "Content not active");
        let expires = env.ledger().sequence() + duration as u32;
        let sub = Subscription {
            subscriber: subscriber.clone(),
            content_id,
            expires_at: expires as u64,
            is_active: true,
        };
        env.storage()
            .persistent()
            .set(&(subscriber, content_id), &sub);
    }

    // ── READ ─ Get content metadata ───────────────────────────────────────────
    pub fn get_content(env: Env, id: u64) -> Content {
        env.storage().persistent().get(&id).unwrap()
    }

    // ── READ ─ Get subscription details ───────────────────────────────────────
    pub fn get_subscription(
        env: Env,
        subscriber: Address,
        content_id: u64,
    ) -> Subscription {
        env.storage()
            .persistent()
            .get(&(subscriber, content_id))
            .unwrap()
    }

    // ── READ ─ Check if address has active access ─────────────────────────────
    pub fn check_access(env: Env, subscriber: Address, content_id: u64) -> bool {
        let result: Option<Subscription> = env
            .storage()
            .persistent()
            .get(&(subscriber, content_id));
        match result {
            Some(s) => s.is_active && s.expires_at > env.ledger().sequence() as u64,
            None => false,
        }
    }

    // ── UPDATE ─ Content owner updates the price ──────────────────────────────
    pub fn update_price(env: Env, owner: Address, content_id: u64, price: i128) {
        owner.require_auth();
        let mut content: Content = env.storage().persistent().get(&content_id).unwrap();
        assert!(content.owner == owner, "Not owner");
        content.price = price;
        env.storage().persistent().set(&content_id, &content);
    }

    // ── UPDATE ─ Renew an existing subscription ───────────────────────────────
    pub fn renew_subscription(
        env: Env,
        subscriber: Address,
        content_id: u64,
        extra_ledgers: u64,
    ) {
        subscriber.require_auth();
        let mut s: Subscription = env
            .storage()
            .persistent()
            .get(&(subscriber.clone(), content_id))
            .unwrap();
        s.expires_at += extra_ledgers;
        s.is_active = true;
        env.storage()
            .persistent()
            .set(&(subscriber, content_id), &s);
    }

    // ── DELETE ─ Owner removes content from the platform ─────────────────────
    pub fn remove_content(env: Env, owner: Address, content_id: u64) {
        owner.require_auth();
        let mut content: Content = env.storage().persistent().get(&content_id).unwrap();
        assert!(content.owner == owner, "Not owner");
        content.is_active = false;
        env.storage().persistent().set(&content_id, &content);
    }

    // ── DELETE ─ Subscriber cancels their subscription ────────────────────────
    pub fn cancel_subscription(env: Env, subscriber: Address, content_id: u64) {
        subscriber.require_auth();
        let mut s: Subscription = env
            .storage()
            .persistent()
            .get(&(subscriber.clone(), content_id))
            .unwrap();
        s.is_active = false;
        env.storage()
            .persistent()
            .set(&(subscriber, content_id), &s);
    }
}