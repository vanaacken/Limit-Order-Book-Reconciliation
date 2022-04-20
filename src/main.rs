
use tungstenite::connect;
use url::Url;
use serde_json::{Value, json};
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::{WebSocket, Message};
use tungstenite::protocol::CloseFrame;
use tungstenite::protocol::frame::coding::CloseCode;
use std::{thread, time};
use std::sync::{Mutex, Arc};
mod models;
use crate::models::Order;
use crate::models::BookStreamWrapper;
use crate::models::LimitOrderBook;

pub static DERIBIT_WS_API: &str = "wss://www.deribit.com/ws/api/v2";
pub static DERIBIT_WS_API_TESTNET: &str = "wss://test.deribit.com/ws/api/v2";
 
fn format_best(lob: &LimitOrderBook) -> String {
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

fn reconnect(mut socket: WebSocket<AutoStream>, mut lob: LimitOrderBook) -> (WebSocket<AutoStream>,BookStreamWrapper,LimitOrderBook) {
    let parsed: BookStreamWrapper;
    let close_frame = CloseFrame  {
        code: CloseCode::Normal,
        reason: Default::default(),
    };
    _ = socket.close(Some(close_frame));
    (socket, _) = deribit_connect();
    socket = send_subscribe_msg(socket);
    (socket, _) = subscription_response(socket);
    (socket, parsed) = read_subscription(socket);
    lob.asks.clear();
    lob.bids.clear();
    (socket, parsed, lob)
}

fn check_id_status(change_id: i64 ,parsed: &BookStreamWrapper ) -> i64 {
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

    let mut socket: WebSocket<AutoStream>;
    let msg = Arc::new(Mutex::new(String::from("no data yet")));
    let set_msg = msg.clone();
    
    (socket, _) = deribit_connect();
    socket = send_subscribe_msg(socket);
    (socket, _) = subscription_response(socket);

    std::thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_millis(1000));
            let to_print = msg.lock().unwrap();
            println!("{}\n", to_print);
            
        }
    });

    let mut lob = LimitOrderBook::new();
    let mut change_id: i64 = 0;
    loop {
        let mut parsed: BookStreamWrapper;
        (socket, parsed) = read_subscription(socket);
        change_id = check_id_status(change_id, &parsed);
        if change_id == -1  {
            (socket, parsed, lob) = reconnect(socket, lob);
        }
        lob = resolve_deltas(lob, &parsed);
        let mut set_msg = set_msg.lock().unwrap();
        *set_msg = format_best(&lob);
    }
}
