
use tungstenite::connect;
use url::Url;
// use std::collections::BTreeMap;
use serde_json::{Value, json};
// use float_cmp::ApproxEq;
// use float_ord::FloatOrd;
// use ordered_float::OrderedFloat;
// use std::cmp::Ordering;
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::{WebSocket, Message};
// use std::thread;
// use std::sync::{Arc, Mutex};
mod models;
use crate::models::Order;
use crate::models::BookStreamWrapper;
use crate::models::LimitOrderBook;
// use crate::models::ChangeId;

// use lobr::{print_map_values, price_key};

pub static DERIBIT_WS_API: &str = "wss://www.deribit.com/ws/api/v2";
pub static DERIBIT_WS_API_TESTNET: &str = "wss://test.deribit.com/ws/api/v2";
 
fn print_lob(lob: &LimitOrderBook) {
    let best_ask = lob.asks.iter().next().unwrap();    
    println!(
        "Best ask: {}, Quantity: {}",
        best_ask.1.price,
        best_ask.1.qty,
    );
    let best_bid = lob.bids.iter().next_back().unwrap();
    println!(
        "Best bid: {}, Quantity{}",
        best_bid.1.price,
        best_bid.1.qty,
    )
}

fn parse_json_data() -> Value {
    json!({
        "jsonrpc": "2.0",
            "method": "public/subscribe",
            "id": "42",
            "params": {
                "channels": [
                    "book.BTC-PERPETUAL.100ms"]}
    })
}

fn deribit_connect() -> (WebSocket<AutoStream>, Response) {
    let ws_url = DERIBIT_WS_API;
    connect(Url::parse(ws_url).unwrap()).expect("Can't connect")
}

fn send_subscribe_msg(mut socket: WebSocket<AutoStream>) -> WebSocket<AutoStream>{
    let json_data = parse_json_data();
    let msg = tungstenite::Message::Text(json_data.to_string());
    let _ = socket.write_message(msg);
    socket
}

fn subscription_response(mut socket: WebSocket<AutoStream>) -> (WebSocket<AutoStream>, Message) {
    let recv = socket.read_message().expect("Error reading message");
    (socket, recv)
}

fn read_subscription(mut socket: WebSocket<AutoStream>) -> (WebSocket<AutoStream>, BookStreamWrapper) {
    let recv = socket.read_message().expect("Error reading message");
    let recv = match recv {
        tungstenite::Message::Text(s) => s,
        _ => {
            panic!("Error getting text");
        }
    };
    (
        socket,
        serde_json::from_str(&recv).expect("Can't parse")
    )
}

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

fn resolve_deltas(mut lob: LimitOrderBook, parsed: &BookStreamWrapper) -> LimitOrderBook {
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



fn main() {

    let mut lob = LimitOrderBook::new();
    // let mut change_id: i64;
    let mut socket: WebSocket<AutoStream>;
    (socket, _) = deribit_connect();
    socket = send_subscribe_msg(socket);
    (socket, _) = subscription_response(socket);
    let _ = std::thread::spawn(|| {
        
    });
    loop {
        let parsed: BookStreamWrapper;
        (socket, parsed) = read_subscription(socket);
        // TODO -----------------------------
        //  Check Reconnection
        //      -> renew connection
        //      -> clear lob
        // -----------------------------------
    //     let s_type = parsed.params.data.r#type;
    //    if let s_type.as_str() = "snapshot" {
    //        change_id = parsed.params.data.change_id;
    //    }


        lob = resolve_deltas(lob, &parsed);
        // TODO -----------------------------
        //  find best bid/ask
        // -----------------------------------
        // println!("{}", parsed.params.data.timestamp);

        // TODO -----------------------------
        //  Print Prices 
        //  -> per second
        // -----------------------------------
        print_lob(&lob);
    }
}
