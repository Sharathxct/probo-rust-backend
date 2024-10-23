use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Serialize)]
pub struct UserBalance {
    pub balance: u64,
    pub locked: u64,
}

type UserId = String;

#[derive(Serialize)]
pub struct InrBalance {
    user_balances: HashMap<UserId, UserBalance>,
}

impl InrBalance {
    pub fn instance() -> Arc<Mutex<InrBalance>> {
        static mut INSTANCE: Option<Arc<Mutex<InrBalance>>> = None;

        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Arc::new(Mutex::new(InrBalance {
                    user_balances: HashMap::new(),
                })));
            }
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    // create new user
    pub fn add_user(&mut self, user_id: String) {
        self.user_balances.entry(user_id).or_default();
    }

    // Method to get the balance of a user
    pub fn get_balance(&self, user_id: String) -> Option<&UserBalance> {
        self.user_balances.get(&user_id)
    }

    // increase balance
    pub fn increase_bal(&mut self, user_id: String, amount: u64) {
        if let Some(user_balance) = self.user_balances.get_mut(&user_id) {
            user_balance.balance += amount;
        } else {
            // Optionally handle the case where the user does not exist
            println!("User {} not found.", user_id);
        }
    }

    // decrease balance
    pub fn deduct_balance(&mut self, user_id: String, amount: u64) {
        if let Some(user_balance) = self.user_balances.get_mut(&user_id) {
            if user_balance.balance >= amount {
                user_balance.balance -= amount;
            } else {
                println!("Insufficient funds");
            }
        } else {
            // Optionally handle the case where the user does not exist
            println!("User {} not found.", user_id);
        }
    }

    // Lock Balance
    pub fn lock_balance(&mut self, user_id: String, amount: u64) {
        if let Some(user_balance) = self.user_balances.get_mut(&user_id) {
            if user_balance.balance >= amount {
                user_balance.balance -= amount;
                user_balance.locked += amount;
            } else {
                println!("Balance not sufficient");
            }
        } else {
            // Optionally handle the case where the user does not exist
            println!("User {} not found.", user_id);
        }
    }

    // Unlock Balance
    pub fn unlock_balance(&mut self, user_id: String, amount: u64) {
        if let Some(user_balance) = self.user_balances.get_mut(&user_id) {
            if user_balance.locked >= amount {
                user_balance.locked -= amount;
                user_balance.balance += amount;
            } else {
                println!("Not enough locked balance to unlock");
            }
        } else {
            // Optionally handle the case where the user does not exist
            println!("User {} not found.", user_id);
        }
    }

    // deduct locked balance
    pub fn deduct_locked(&mut self, user_id: String, amount: u64) {
        if let Some(user_balance) = self.user_balances.get_mut(&user_id) {
            if user_balance.locked >= amount {
                user_balance.locked -= amount;
            } else {
                println!("Insufficient funds");
            }
        } else {
            // Optionally handle the case where the user does not exist
            println!("User {} not found.", user_id);
        }
    }
}

// TODO audit again
