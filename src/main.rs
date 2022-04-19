
use tungstenite::{accept, Message, connect};
use url::Url;
use std::collections::BTreeMap;
use serde_json::{Value, from_str, json};
use float_cmp::ApproxEq;
use std::cmp::PartialEq;
mod models;
use crate::models::Order;
use crate::models::BookStreamWrapper;
// use lobr::{print_map_values, price_key};

pub static DERIBIT_WS_API: &str = "wss://www.deribit.com/ws/api/v2";
pub static DERIBIT_WS_API_TESTNET: &str = "wss://test.deribit.com/ws/api/v2";



// change map key to fixed point
pub fn price_key(f_price: f64) -> i64 {
    (f_price * 100.0) as i64
 }
 
pub fn print_map_values(map: &BTreeMap<i64, Order>) {
     println!(
         "Map size: {}, lowest ask: --, Quantitiy: --",
         map.len()
     );
 }



fn deribit_connect() -> Result {
    
}




fn main() {

    let ws_url = DERIBIT_WS_API_TESTNET;
    let (mut socket, _) = connect(Url::parse(ws_url).unwrap()).expect("Can't connect");
   


    let mut ask_tree = BTreeMap::new();
    

    let json_data = json!({
        "jsonrpc": "2.0",
            "method": "public/subscribe",
            "id": "42",
            "params": {
                "channels": [
                    "book.BTC-PERPETUAL.100ms"]}
    });
    
    // let req: Result<String> = serde_json::from_str(json_req);
    // let msg = socket.read_message().expect("Error reading message");
    let msg = tungstenite::Message::Text(json_data.to_string());

    let _ = socket.write_message(msg);
    let received = socket.read_message().expect("Error reading message");
    println!("received {:?}", received);
    loop {

        let msg_recv = socket.read_message().expect("Error reading message");
        let msg_recv = match msg_recv {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };
        let parsed: BookStreamWrapper = serde_json::from_str(&msg_recv).expect("Can't parse");
        for i in 0..parsed.params.data.asks.len() {
            let f_price = parsed.params.data.asks[i].1;
            let f_quantity = parsed.params.data.asks[i].2;
            match String::from(&parsed.params.data.asks[i].0).as_str() {
                "new" => {
                    ask_tree.insert(price_key(f_price), Order::new(f_price, f_quantity));
                }
                "change" => {
                    if let Some(order) = ask_tree.get_mut(&price_key(f_price)) {
                        *order = Order::new(f_price, f_quantity);
                    }
                }
                "delete" => {
                    ask_tree.remove(&price_key(f_price));
                }
                _ => ()
            }
        }
        println!("{}", parsed.params.data.timestamp);
        print_map_values(&ask_tree);
        // for i in 0..parsed.params.data.asks.len() {
        //     println!(
        //         "Delta: {}. ask: {}, size {}",
        //         parsed.params.data.asks[i].0,
        //         parsed.params.data.asks[i].1,
        //         parsed.params.data.asks[i].2,
        //     )
        // }
        // for i in 0..parsed.params.data.bids.len() {
        //     println!(
        //         "Delta: {}. bids: {}, size {}",
        //         parsed.params.data.bids[i].0,
        //         parsed.params.data.bids[i].1,
        //         parsed.params.data.bids[i].2,
        //     )
        // }   
    }
}
