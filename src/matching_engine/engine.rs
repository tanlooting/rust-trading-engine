use super::orderbook::{Order, Orderbook};
use std::collections::HashMap;
use rust_decimal::prelude::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    // e.g. BTCUSD: BTC base, USD quote
    base: String, 
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair{base, quote}
    }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("Opening new orderbook for market: {:?}", pair);
    }
    
    pub fn place_limit_order(
        &mut self, pair: TradingPair, 
        price: Decimal, 
        order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                println!("placed limit order at price: {}", price);
                Ok(())
            },
            None => {
                Err(format!("No orderbook found for market: {}", pair.to_string()))
            }
        }
    }
}