use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StockQuantity {
    pub quantity: u64,
    pub locked: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserStocks {
    pub yes: StockQuantity,
    pub no: StockQuantity,
}

type UserId = String;
type StockSymbol = String;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StockBalance {
    user_stocks: HashMap<UserId, HashMap<StockSymbol, UserStocks>>, // userId -> (stockSymbol -> UserStocks)
}

impl StockBalance {
    pub fn instance() -> Arc<Mutex<StockBalance>> {
        static mut INSTANCE: Option<Arc<Mutex<StockBalance>>> = None;

        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Arc::new(Mutex::new(StockBalance {
                    user_stocks: HashMap::new(),
                })));
            }
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    // Method to get stock quantities for a user
    pub fn get_stocks(&self, user_id: &str) -> Option<&HashMap<String, UserStocks>> {
        self.user_stocks.get(user_id)
    }

    // Method to increase stock quantity for a user
    pub fn increase_stock_quantity(
        &mut self,
        user_id: String,
        stock_symbol: String,
        stock_type: String,
        quantity: u64,
    ) {
        let user_stocks = self
            .user_stocks
            .entry(user_id)
            .or_default()
            .entry(stock_symbol)
            .or_default();
        if stock_type == "yes" {
            user_stocks.yes.quantity += quantity;
        } else if stock_type == "no" {
            user_stocks.no.quantity += quantity;
        }
    }

    // Method to decrease stock quantity for a user
    pub fn decrease_stock_quantity(
        &mut self,
        user_id: String,
        stock_symbol: String,
        stock_type: String,
        quantity: u64,
    ) -> Result<(), String> {
        let yes_balance = self
            .user_stocks
            .get(&user_id)
            .unwrap()
            .get(&stock_symbol)
            .unwrap()
            .yes
            .quantity;
        let no_balance = self
            .user_stocks
            .get(&user_id)
            .unwrap()
            .get(&stock_symbol)
            .unwrap()
            .no
            .quantity;

        if stock_type == "yes" && yes_balance < quantity
            || stock_type == "no" && no_balance < quantity
        {
            return Err("Insufficient balance".to_string());
        }
        let user_stocks = self
            .user_stocks
            .entry(user_id)
            .or_default()
            .entry(stock_symbol)
            .or_default();
        if stock_type == "yes" {
            user_stocks.yes.quantity -= quantity;
            Ok(())
        } else {
            user_stocks.no.quantity -= quantity;
            Ok(())
        }
    }

    // Method to get stock balance for a user
    pub fn get_stock_balance(&self, user_id: &str, stock_symbol: &str) -> Option<&UserStocks> {
        Some(
            self.user_stocks
                .get(user_id)
                .unwrap()
                .get(stock_symbol)
                .unwrap(),
        )
    }

    // lock stock balance
    pub fn lock_stock(
        &mut self,
        user_id: String,
        stock_symbol: String,
        stock_type: String,
        quantity: u64,
    ) -> Result<(), String> {
        let user_stocks = self
            .user_stocks
            .entry(user_id)
            .or_default()
            .entry(stock_symbol)
            .or_default();

        if stock_type == "yes" && user_stocks.yes.quantity < quantity
            || stock_type == "no" && user_stocks.no.quantity < quantity
        {
            return Err("Insufficient balance".to_string());
        }
        if stock_type == "yes" {
            user_stocks.yes.quantity -= quantity;
            user_stocks.yes.locked += quantity;
            Ok(())
        } else {
            user_stocks.no.quantity -= quantity;
            user_stocks.no.locked += quantity;
            Ok(())
        }
    }

    // Unlock stock balance
    pub fn unlock_stock(
        &mut self,
        user_id: String,
        stock_symbol: String,
        stock_type: String,
        quantity: u64,
    ) -> Result<(), String> {
        let user_stocks = self
            .user_stocks
            .entry(user_id)
            .or_default()
            .entry(stock_symbol)
            .or_default();
        if stock_type == "yes" && user_stocks.yes.locked < quantity
            || stock_type == "no" && user_stocks.no.locked < quantity
        {
            return Err("Insufficient balance".to_string());
        }
        if stock_type == "yes" {
            user_stocks.yes.locked -= quantity;
            user_stocks.yes.quantity += quantity;
            Ok(())
        } else {
            user_stocks.no.locked -= quantity;
            user_stocks.no.quantity += quantity;
            Ok(())
        }
    }
}

// TODO authenticate user before entry
