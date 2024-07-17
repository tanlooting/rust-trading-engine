mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{Order, BidOrAsk, Orderbook};
use rust_decimal_macros::dec;

fn main() {

    let buy_order = Order::new(BidOrAsk::Bid, 2.45);
    let buy_order_2 = Order::new(BidOrAsk::Bid, 2.45);
    // // limit.add_order(buy_order);
    // // limit.add_order(sell_order);
    let mut orderbook = Orderbook::new();
    orderbook.add_order(dec!(4.4), buy_order);
    orderbook.add_order(dec!(4.4), buy_order_2);

    let sell_order = Order::new(BidOrAsk::Ask, 6.0);
    orderbook.add_order(dec!(20.0), sell_order);
    // println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.place_limit_order(pair,  dec!(10.000),  buy_order).unwrap(); // unwrap
}
