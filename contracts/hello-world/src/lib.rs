#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    HighestBid,
    HighestBidder,
}

#[contract]
pub struct SimpleAuction;

#[contractimpl]
impl SimpleAuction {

    // Initialize the auction with starting bid
    pub fn initialize(env: Env, starting_bid: i128) {
        env.storage().instance().set(&DataKey::HighestBid, &starting_bid);
    }

    // Place a bid
    pub fn bid(env: Env, bidder: Address, amount: i128) {
        bidder.require_auth();

        let current_bid: i128 = env
            .storage()
            .instance()
            .get(&DataKey::HighestBid)
            .unwrap_or(0);

        if amount <= current_bid {
            panic!("Bid must be higher than current highest bid");
        }

        env.storage().instance().set(&DataKey::HighestBid, &amount);
        env.storage().instance().set(&DataKey::HighestBidder, &bidder);
    }

    // Get current highest bid
    pub fn get_highest_bid(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::HighestBid).unwrap_or(0)
    }

    // Get current highest bidder
    pub fn get_highest_bidder(env: Env) -> Address {
        env.storage().instance().get(&DataKey::HighestBidder).unwrap()
    }
}