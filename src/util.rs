use crate::models::BookStreamWrapper;
use crate::models::LimitOrderBook;

pub fn format_best(lob: &LimitOrderBook) -> String {
    let best_ask = lob.asks.iter().next().unwrap();    
    let best_bid = lob.bids.iter().next_back().unwrap();
    format!(
        "Best ask: {}, Quantity: {}\nBest bid: {}, Quantity {}",
        best_ask.1.price,
        best_ask.1.qty,
        best_bid.1.price,
        best_bid.1.qty,
    )
}

pub fn check_id_status(change_id: i64 ,parsed: &BookStreamWrapper ) -> i64 {
    match parsed.params.data.r#type.as_str() {
        "snapshot" => {
            parsed.params.data.change_id
        }
        "change" => { 
            if parsed.params.data.prev_change_id == Some(change_id) {
                parsed.params.data.change_id
            } else {
                -1
            }
        }
        _ => -1,
    }
}
