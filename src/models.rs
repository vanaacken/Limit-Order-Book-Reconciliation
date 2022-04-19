use serde::{Deserialize,Deserializer, Serialize, Serializer};
use serde::de;
use std::collections::BTreeMap;

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
    pub fn new(price: f64, size: f64) -> Self {
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
    pub asks: BTreeMap<i64, Order>,
    pub bids: BTreeMap<i64, Order>,
    pub best_ask: Order,
    pub best_bid: Order,
}