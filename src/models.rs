use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

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
    pub r#type: String,
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

// #[derive(Debug, Clone, Copy)]
// pub struct Key {
//     pub key: OrderedFloat<f64>,
// }

// impl Key {
//     pub fn new(price: f64) -> Self {
//         let key = OrderedFloat(price)
//         Key {key}
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub price: f64,
    pub qty: f64,
}

impl Order {
    pub fn new(price: f64, qty: f64) -> Self {
        Order {price, qty}
    }
}

#[derive(Debug, Clone)]
pub struct LimitOrderBook {
    pub asks: BTreeMap<OrderedFloat<f64>, Order>,
    pub bids: BTreeMap<OrderedFloat<f64>, Order>,
    pub best_ask: Order,
    pub best_bid: Order,
}

impl LimitOrderBook {
    pub fn new() -> Self {
        LimitOrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            best_ask: Order::new(0.0, 0.0),
            best_bid: Order::new(0.0, 0.0),
        }
    }


    pub fn asks_new (mut self, order: Order) -> Self {
        self.asks.insert(OrderedFloat(order.price), order);
        self
    }

    pub fn asks_delete(mut self, order: Order) -> Self {
        self.asks.remove(&OrderedFloat(order.price));
        self
    }

    pub fn asks_change(mut self, order: Order) -> Self {
        if let Some(item) = self.asks.get_mut(&OrderedFloat(order.price)) {
            *item = order;
        }
        self
    }

    pub fn bids_new (mut self, order: Order) -> Self {
        self.bids.insert(OrderedFloat(order.price), order);
        self
    }

    pub fn bids_delete(mut self, order: Order) -> Self {
        self.bids.remove(&OrderedFloat(order.price));
        self
    }

    pub fn bids_change(mut self, order: Order) -> Self {
        if let Some(item) = self.bids.get_mut(&OrderedFloat(order.price)) {
            *item = order;
        }
        self
    }
}

// #[derive(Debug, Copy, Clone)]
// pub struct ChangeId{
//     pub id: i64,
//     pub prev_id: i64,
// }

// impl ChangeId {
//     pub fn new(id: i64, )-> Self {
//         let prev_id = id;
//         ChangeId{id, prev_id}
//     }

//     pub 

// }
















