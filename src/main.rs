use tungstenite::client::AutoStream;
use std::{thread, time};
use std::sync::{Mutex, Arc};
use tungstenite::WebSocket;
mod models;
mod util;
mod book;
mod connection;
use crate::models::BookStreamWrapper;
use crate::models::LimitOrderBook;
use crate::util::{format_best, check_id_status};
use crate::book::resolve_deltas;
use crate::connection::{deribit_connect,
    send_subscribe_msg,
    subscription_response,
    reconnect,
    read_subscription};




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
