#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct Rental {
    pub owner: Address,
    pub renter: Option<Address>,
    pub item_name: String,
    pub is_rented: bool,
}

#[contracttype]
pub enum DataKey {
    Item,
}

#[contract]
pub struct RentalContract;

#[contractimpl]
impl RentalContract {
    pub fn list_item(env: Env, owner: Address, item_name: String) {
        owner.require_auth();
        let rental = Rental {
            owner: owner.clone(),
            renter: None,
            item_name,
            is_rented: false,
        };
        env.storage().instance().set(&DataKey::Item, &rental);
        log!(&env, "Item listed by {}", owner);
    }

    pub fn rent_item(env: Env, renter: Address) {
        renter.require_auth();
        let mut rental: Rental = env.storage().instance().get(&DataKey::Item).expect("Item not listed");

        if rental.is_rented {
            panic!("Item is already rented");
        }

        rental.renter = Some(renter.clone());
        rental.is_rented = true;

        env.storage().instance().set(&DataKey::Item, &rental);
        log!(&env, "Item rented by {}", renter);
    }

    pub fn return_item(env: Env, renter: Address) {
        renter.require_auth();
        let mut rental: Rental = env.storage().instance().get(&DataKey::Item).expect("Item not listed");

        if rental.renter != Some(renter.clone()) {
            panic!("Only current renter can return the item");
        }

        rental.renter = None;
        rental.is_rented = false;

        env.storage().instance().set(&DataKey::Item, &rental);
        log!(&env, "Item returned by {}", renter);
    }

    pub fn get_item_status(env: Env) -> Rental {
        env.storage().instance().get(&DataKey::Item).expect("Item not listed")
    }
}
