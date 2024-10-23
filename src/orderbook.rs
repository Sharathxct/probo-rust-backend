use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Price = u64;
type StockSymbol = String;

#[derive(Debug)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug)]
pub enum StockType {
    Yes,
    No,
}

#[derive(Debug)]
pub struct Order {
    pub user_id: String,
    pub order_type: OrderType,
    pub quantity: u64,
}

#[derive(Debug)]
pub struct PriceEntry {
    pub total: u64,
    pub orders: HashMap<u64, Order>,
    pub key: u64,
}

#[derive(Debug)]
pub struct StockOrders {
    pub yes: HashMap<Price, PriceEntry>,
    pub no: HashMap<Price, PriceEntry>,
}

#[derive(Debug)]
pub struct OrderBook {
    pub orders: HashMap<StockSymbol, StockOrders>,
}

impl OrderBook {
    pub fn instance() -> Arc<Mutex<OrderBook>> {
        static mut INSTANCE: Option<Arc<Mutex<OrderBook>>> = None;
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Arc::new(Mutex::new(OrderBook {
                    orders: HashMap::new(),
                })));
            }
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    // create market
    pub fn create_market(&mut self, stock_symbol: String) {
        self.orders.entry(stock_symbol).or_insert(StockOrders {
            yes: HashMap::new(),
            no: HashMap::new(),
        });
    }

    fn process_order(
        &mut self,
        user_id: String,
        stock_symbol: String,
        stock_type: StockType,
        price: u64,
        order_type: OrderType,
        quantity: u64,
    ) {
        // Ensure the market exists
        if !self.orders.contains_key(&stock_symbol) {
            println!("Market for {} does not exist.", stock_symbol);
            return;
        }

        let stock_orders = self.orders.get_mut(&stock_symbol).unwrap();

        // Determine the price entry based on stock type
        let price_entry = match stock_type {
            StockType::Yes => stock_orders.yes.entry(price).or_insert_with(|| PriceEntry {
                total: 0,
                orders: HashMap::new(),
                key: 0,
            }),
            StockType::No => stock_orders.no.entry(price).or_insert_with(|| PriceEntry {
                total: 0,
                orders: HashMap::new(),
                key: 0,
            }),
        };

        // Update the price entry and add the order
        price_entry.total += quantity;
        price_entry.orders.insert(
            price_entry.key,
            Order {
                user_id,
                order_type,
                quantity,
            },
        );
        price_entry.key += 1;
    }

    // order buy yes
    pub fn order_buy_yes(
        &mut self,
        user_id: String,
        stock_symbol: String,
        price: Price,
        quantity: u64,
    ) {
        self.process_order(
            user_id,
            stock_symbol,
            StockType::Yes,
            price,
            OrderType::Buy,
            quantity,
        );
    }

    // Order buy no
    pub fn order_buy_no(
        &mut self,
        user_id: String,
        stock_symbol: String,
        price: Price,
        quantity: u64,
    ) {
        self.process_order(
            user_id,
            stock_symbol,
            StockType::No,
            price,
            OrderType::Buy,
            quantity,
        );
    }

    // Order sell yes
    pub fn order_sell_yes(
        &mut self,
        user_id: String,
        stock_symbol: String,
        price: Price,
        quantity: u64,
    ) {
        self.process_order(
            user_id,
            stock_symbol,
            StockType::Yes,
            price,
            OrderType::Sell,
            quantity,
        );
    }

    // Order sell no
    pub fn order_sell_no(
        &mut self,
        user_id: String,
        stock_symbol: String,
        price: Price,
        quantity: u64,
    ) {
        self.process_order(
            user_id,
            stock_symbol,
            StockType::No,
            price,
            OrderType::Sell,
            quantity,
        );
    }

    // cancel order

    // instant buy

    // instant sell

    // getBestprice

    // match order
}
