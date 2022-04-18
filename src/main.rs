
// use std::net::TcpListener;
// use std::thread::spawn;
use tungstenite::{accept, Message, connect};
use url::Url;
use serde::{Deserialize,Deserializer, Serialize, Serializer};
use serde::de;
use std::collections::LinkedList;
use serde_json::{Value, from_str, json};
use float_cmp::ApproxEq;
use std::cmp::PartialEq;
/*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
                            STATICS 

**|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
// lazy_static! {
//     static ref RE: Regex = Regex::new(r#""jsonrpc":"2.0","id":(\d+),"#).unwrap();
// }

// type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

// pub static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
pub static DERIBIT_WS_API: &str = "wss://www.deribit.com/ws/api/v2";
pub static DERIBIT_WS_API_TESTNET: &str = "wss://test.deribit.com/ws/api/v2";

/***********************************************************************/

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Delta {
    New(String),
    Change(String),
    Delete(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderBookDelta(pub String, pub f64, pub f64);


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct BookData {
    pub asks: Vec<OrderBookDelta>,
    pub bids: Vec<OrderBookDelta>,
    pub change_id: i64,
    pub instrument_name: String,
    pub prev_change_id: Option<i64>,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct BookParams {
    pub data: BookData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct BookStreamWrapper {
    pub params: BookParams,
    pub method: String,
    pub jsonrpc: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub price: f64,
    pub size: f64,
}


impl Order {
    fn new(price: f64, size: f64) -> Self {
        Order {price, size}
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

#[derive(Debug, Clone)]
pub struct LimitOrderBook {
    pub asks: LinkedList<Order>,
    pub bids: LinkedList<Order>,
    pub best_ask: Order,
    pub best_bid: Order,
}

// impl LimitOrderBook {
//     fn delta(&mut Self, data: BookData) -> Self {
//         for i in 0..data.asks.len() {
//             LimitOrderBook::deltaAsks(data.asks[i].clone());
//         };
//         // for i in 0..data.bids.len() {
//         //     LimitOrderBook::deltaMatch(self.bids, data.bids[i]);
//         // }
//     }
    
//     fn deltaAsks(Self, delta: OrderBookDelta) -> Self {
//         match delta.0.as_str() {
//             "new" => {
                
//             },
//             "delete" => {

//             }
//             "change" => {

//             }
//             _ => ()
//         };
//     }
// }











fn main() {

    let ws_url = DERIBIT_WS_API_TESTNET;
    let (mut socket, _) = connect(Url::parse(ws_url).unwrap()).expect("Can't connect");
   

    let mut ask_arr = Vec::new();
    // let mut bid_arr = Vec::new();


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
            let delta_method = String::from(&parsed.params.data.asks[i].0);
            let price_level: f64 = parsed.params.data.asks[i].1;
            let quanitity: f64 = parsed.params.data.asks[i].1;
            let order = Order::new(price_level, quanitity);
            match delta_method.as_str() {
                "new" => {
                    ask_arr.push(order.clone())
                }
                "change" => {
                    if let Some(index) = ask_arr.iter().position(|value| *value == order) {
                        ask_arr[index] = order.clone();
                    }
                }
                "delete" => {
                    if let Some(index) = ask_arr.iter().position(|value| *value == order) {
                        ask_arr.swap_remove(index);
                    }
                }
                _ => ()
            }
        }

        let mut best_ask = ask_arr[0];
        if let Some(index) = ask_arr.iter().position(|value| *value < best_ask) {
            best_ask = index.clone();
        }


        for index in ask_arr.into_iter() {
                if (index <= best_ask)

        }

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
        // println!("{}", parsed.params.data.timestamp);
    }
}
