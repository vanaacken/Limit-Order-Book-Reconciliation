
use crate::models::Order;
use crate::models::BookStreamWrapper;
use crate::models::LimitOrderBook;

fn match_asks(to_match: String, mut lob: LimitOrderBook, order: Order) -> LimitOrderBook {
    match String::from(&to_match).as_str() {
        "new" => {
            lob = lob.asks_new(order)
        }
        "change" => {
            lob = lob.asks_change(order)
        }
        "delete" => {
            lob = lob.asks_delete(order)
        }
        _ => ()
    };
    lob
}

fn match_bids(to_match: String, mut lob: LimitOrderBook, order: Order) -> LimitOrderBook {
    match String::from(&to_match).as_str() {
        "new" => {
            lob = lob.bids_new(order)
        }
        "change" => {
            lob = lob.bids_change(order)
        }
        "delete" => {
            lob = lob.bids_delete(order)
        }
        _ => ()
    };
    lob
}

pub fn resolve_deltas(mut lob: LimitOrderBook, parsed: &BookStreamWrapper) -> LimitOrderBook {
    for i in 0..parsed.params.data.asks.len() {
        let order = Order::new(
            parsed.params.data.asks[i].1,
            parsed.params.data.asks[i].2,
        );
        let to_match = String::from(&parsed.params.data.asks[i].0);
        lob = match_asks(to_match, lob, order);
    };
    for i in 0..parsed.params.data.bids.len() {
        let order = Order::new(
            parsed.params.data.bids[i].1,
            parsed.params.data.bids[i].2,
        );
        let to_match = String::from(&parsed.params.data.bids[i].0);
        lob = match_bids(to_match, lob, order);
    };
    lob
}
