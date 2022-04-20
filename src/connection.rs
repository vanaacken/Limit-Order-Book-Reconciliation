use tungstenite::connect;
use url::Url;
use serde_json::{Value, json};
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::{WebSocket, Message};
use tungstenite::protocol::CloseFrame;
use tungstenite::protocol::frame::coding::CloseCode;
use crate::models::BookStreamWrapper;
use crate::models::LimitOrderBook;

pub static DERIBIT_WS_API: &str = "wss://www.deribit.com/ws/api/v2";
// pub static DERIBIT_WS_API_TESTNET: &str = "wss://test.deribit.com/ws/api/v2";
 
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

pub fn deribit_connect() -> (WebSocket<AutoStream>, Response) {
    let ws_url = DERIBIT_WS_API;
    connect(Url::parse(ws_url).unwrap()).expect("Can't connect")
}

pub fn send_subscribe_msg(mut socket: WebSocket<AutoStream>) -> WebSocket<AutoStream>{
    let json_data = parse_json_data();
    let msg = tungstenite::Message::Text(json_data.to_string());
    let _ = socket.write_message(msg);
    socket
}

pub fn reconnect(mut socket: WebSocket<AutoStream>, mut lob: LimitOrderBook) -> (WebSocket<AutoStream>,BookStreamWrapper,LimitOrderBook) {
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

pub fn subscription_response(mut socket: WebSocket<AutoStream>) -> (WebSocket<AutoStream>, Message) {
    let recv = socket.read_message().expect("Error reading message");
    (socket, recv)
}

pub fn read_subscription(mut socket: WebSocket<AutoStream>) -> (WebSocket<AutoStream>, BookStreamWrapper) {
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
