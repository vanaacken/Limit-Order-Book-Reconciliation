// /*
// all in one file first

// setup client

// setup connection

// setup poll
// setup serde json

// setup collection

// printer

// */



// /*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
//                             DEPENDENCIES 

// **|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/

// use fehler::throw;
// use serde::{
//     de::{Error, Unexpected},
//     Deserialize, Deserializer, Serialize, Serializer,
// };


// use anyhow::Error;
// use dotenv::dotenv;
// use env_logger::init;
// use fehler::throws;

// use derive_builder::Builder;
// use fehler::throws;
// use futures::channel::{mpsc, oneshot};
// use futures::{select, FutureExt, SinkExt, Stream, StreamExt, TryStreamExt};
// use lazy_static::lazy_static;
// use log::warn;
// use log::{info, trace};
// use regex::Regex;
// use std::{collections::HashMap, time::Duration};
// use tokio::net::TcpStream;
// use tokio::time::timeout;
// use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
// use tungstenite::Message;
// use url::Url;
// use fehler::throw;


// use crate::errors::Result;
// use crate::models::SubscriptionMessage;
// use futures::channel::mpsc;
// use futures::task::{Context, Poll};
// use futures::Stream;
// use log::warn;
// use pin_project::pin_project;
// use serde::de::DeserializeOwned;
// use serde_json::from_str;
// use std::marker::PhantomData;
// use std::pin::Pin;

// use thiserror::Error;
// use futures::{
//     channel::{mpsc, oneshot},
//     stream::SplitSink,
//     task::{Context, Poll},
//     {Future, SinkExt},}

// /***********************************************************************/

// // pub type Result<T> = anyhow::Result<T>;
// // #[derive(Error, Debug)]
// // pub enum DeribitError {
// //     #[error("Deribit remote error {{code: {code}, message: {message}}}")]
// //     RemoteError { code: i64, message: String },
// //     #[error("The background servo pulling message exited")]
// //     ServoExited,
// //     #[error("Unknown currency {0}")]
// //     UnknownCurrency(String),
// //     #[error("Unknown asset kind {0}")]
// //     UnknownAssetKind(String),
// //     #[error("Websocket disconnected")]
// //     WebsocketDisconnected,
// //     #[error("Request timed out")]
// //     RequestTimeout,
// //     // #[error("oneshot channel canceled on the other side: {0}")]
// //     // CanceledError(#[from] Canceled),
// //     // #[error("cannot parse url: {0}")]
// //     // ParseError(#[from] ParseError),
// //     // #[error("underlying websocket reported an error: {0}")]
// //     // WebsocketError(#[from] WebsocketError),
// //     // #[error("cannot send message to channel: {0}")]
// //     // SendError(#[from] SendError),
// //     // #[error("JSON serialization error: {0}")]
// //     // JsonError(#[from] JsonError),
// //     // #[error("IO error: {0}")]
// //     // IOError(#[from] std::io::Error)
// // }




// /*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
//                             STATICS 

// **|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
// lazy_static! {
//     static ref RE: Regex = Regex::new(r#""jsonrpc":"2.0","id":(\d+),"#).unwrap();
// }

// type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

// // pub static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
// pub static DERIBIT_WS_API: &str = "wss://www.deribit.com/ws/api/v2";
// pub static DERIBIT_WS_API_TESTNET: &str = "wss://test.deribit.com/ws/api/v2";

// /***********************************************************************/









// /*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
//                             BEGIN 
//                             BOOK MODULE
//                     DELTA - BOOKDATA - BOOKCHANNEL

// **|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/
// #[derive(Deserialize, Serialize, Debug, Clone, Copy)]
// #[serde(rename_all = "lowercase")]
// pub enum Delta {
//     New,
//     Change,
//     Delete,
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct OrderBookDelta(pub Delta, pub f64, pub f64);

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct BookData {
//     pub asks: Vec<OrderBookDelta>,
//     pub bids: Vec<OrderBookDelta>,
//     pub change_id: i64,
//     pub instrument_name: String,
//     pub prev_change_id: Option<i64>,
//     pub timestamp: u64,
// }


// #[derive(Debug, Clone)]
// pub struct BookChannel(pub String, pub String);
// impl<'de> Deserialize<'de> for BookChannel {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
//         let segments: Vec<_> = s.split(".").collect();
//         match segments.as_slice(){
//             ["book", instrument_name, interval] => Ok(BookChannel(
//                 instrument_name.to_string(),
//                 interval.to_string(),
//             )),
//             _ => throw!(D::Error::invalid_value(
//                 Unexpected::Str(s),
//                 &"book.{instrument_name}.{interval}"
//             )),
//         }
//     }
// }

// impl Serialize for BookChannel {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(&self.to_string())
//     }
// }

// impl std::fmt::Display for BookChannel{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->std::fmt::Result {
//         write!(f, "book.{}.{}", self.0, self.1)
//     }
// }
// /*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
//                             END 
//                             BOOK MODULE

// **|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/





// /*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
//                             BEGIN 
//                         SUBSCRIPTION MODULE

// **|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/


// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct PublicSubscribeRequest {
//     pub channels: Vec<String>,
// }

// impl PublicSubscribeRequest {
//     pub fn new(channels: &[String]) -> Self {
//         Self {
//             channels: channels.to_vec(),
//         }
//     }
// }
// pub trait Request {
//     const METHOD: &'static str;
//     const HAS_PAYLOAD: bool = true;
//     type Response;

//     fn no_payload(&self) -> bool {
//         !Self::HAS_PAYLOAD
//     }
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct SubscribeResponse(pub Vec<String>);

// impl Request for PublicSubscribeRequest {
//     const METHOD: &'static str = "public/subscribe";
//     type Response = SubscribeResponse;
// }

// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct PublicUnsubscribeRequest {
//     pub channels: Vec<String>,
// }

// impl PublicUnsubscribeRequest {
//     pub fn new(channels: &[String]) -> Self {
//         Self {
//             channels: channels.to_vec(),
//         }
//     }
// }

// #[derive(Deserialize, Serialize, Debug, Clone)] 
// pub struct UnsubscribeResponse(pub Vec<String>);

// impl Request for PublicUnsubscribeRequest {
//     const METHOD: &'static str = "public/unsubscribe";
//     type Response = UnsubscribeResponse;
// }


// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct WithChannel<C, D> {
//     pub channel: C,
//     pub data: D,
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(untagged)]
// pub enum SubscriptionData {
//     Book(WithChannel<BookChannel, BookData>),
// }

// #[derive(Deserialize, Serialize, Clone, Debug, Copy)]
// pub enum JSONRPCVersion {
//     #[serde(rename = "2.0")]
//     V2,
// }

// #[derive(Deserialize, Serialize, Clone, Debug, Copy)]
// #[serde(rename_all = "lowercase")]
// pub enum SubscriptionMethod {
//     Subscription,
//     Heartbeat,
// }


// #[derive(Deserialize, Serialize, Clone, Debug)]
// #[serde(untagged)]
// pub enum SubscriptionParams<D = SubscriptionData> {
//     Subscription(D),
//     Heartbeat { r#type: HeartbeatType },
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// #[serde(rename_all = "snake_case")]
// pub enum HeartbeatType {
//     Heartbeat,
//     TestRequest,
// }

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct SubscriptionMessage<D = SubscriptionData> {
//     pub jsonrpc: JSONRPCVersion,
//     pub method: SubscriptionMethod,
//     pub params: SubscriptionParams<D>,
// }




// pub struct SubscriptionClient {
//     rx: mpsc::Receiver<String>
// }

// impl SubscriptionClient {
//     pub(crate) fn new(rx: mpsc::Receiver<String>) -> SubscriptionClient {
//         SubscriptionClient { rx }
//     }
// }


// impl Stream for SubscriptionClient {
//     type Item = Result<SubscriptionMessage>;

//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
//         let pin = Pin::new(&mut self.rx);
//         // check if match is essential
//         match pin.poll_next(cx) {
//             Poll::Ready(Some(v)) => {
//                 let data = from_str::<SubscriptionMessage>(&v).map_err(From::from);
//                 if let Err(_) = data.as_ref() {
//                     warn!(
//                         "[Subscription Client] Cannot deserialize subscription message {}",
//                         v
//                     );
//                 }
//                 Poll::Ready(Some(data))
//             }
//             Poll::Ready(None) => Poll::Ready(None),
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }

// /*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
//                             END 
//                         SUBSCRIPTION MODULE

// **|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/












// /*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
//                             BEGIN 
//                         CLIENT MODULE

// **|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/


// #[derive(Default, Builder, Debug)]
// #[builder(setter(into))]
// pub struct Deribit {
//     #[builder(default)]
//     testnet: bool,
//     #[builder(default = "10")]
//     subscription_buffer_size: usize,
//     #[builder(setter(into, strip_option), default)]
//     timeout: Option<Duration>,
// }

// impl Deribit {
//     pub fn new() -> Deribit {
//         DeribitBuilder::default().build().unwrap()
//     }

//     pub fn builder() -> DeribitBuilder {
//         DeribitBuilder::default()
//     }


//     pub async fn connect(self) -> (DeribitAPIClient, SubscriptionClient) {
//         let ws_url = if self.testnet {DERIBIT_WS_API_TESTNET} else {DERIBIT_WS_API};
//         println!("Connecting");
//         let ( ws, _) = connect_async(Url::parse(ws_url)).await;

//         let (wstx, wsrx) = ws.split();

//         let (stx, srx) = mpsc::channel(self.subscription_buffer_size);
        
//         (
//             DeribitAPIClient::new(
//                 wstx,
//                 wsrx,
//                 self.timeout.unwrap_or(Duration::from_secs(3600)),
//         ),
//             SubscriptionClient::new(srx)        
//         )
//     }
// }



// pub struct DeribitAPIClient {
//     wstx: SplitSink<WSStream, Message>,
//     waiter_tx: mpsc::Sender<(i64, oneshot::Sender<String>)>,
//     timeout: Duration,
//     id: i64,
// }

// impl DeribitAPIClient {
//     pub(crate) fn new(
//         wstx: SplitSink<WSStream, Message>,
//         waiter_tx: mpsc::Sender<(i64, oneshot::Sender<String>)>,
//         timeout: Duration,
//     ) -> DeribitAPIClient {
//         DeribitAPIClient {
//             wstx: wstx,
//             waiter_tx: waiter_tx,
//             timeout: timeout,
//             id: 0,
//         }
//     }

//     // #[throws(Error)]
//     pub async fn call_raw<'a, R>(&'a mut self, request: R) -> DeribitAPICallRawResult<R::Response>
//     where
//         R: Request + Serialize + 'a,
//     {
//         let (waiter_tx, waiter_rx) = oneshot::channel();
//         let req = JSONRPCRequest {
//             id: self.id,
//             method: R::METHOD.into(),
//             params: request,
//         };
//         self.id += 1;

//         let payload = to_string(&req)?;
//         trace!("[API Client] Request: {}", payload);
//         self.wstx.send(Message::Text(payload)).await?;
//         self.waiter_tx.send((req.id, waiter_tx)).await?;
//         DeribitAPICallRawResult::new(waiter_rx, self.timeout)
//     }

//     #[throws(Error)]
//     pub async fn call<'a, R>(&'a mut self, request: R) -> DeribitAPICallResult<R::Response>
//     where
//         R: Request + Serialize + 'a,
//     {
//         let resp: DeribitAPICallRawResult<R::Response> = self.call_raw(request).await?;
//         DeribitAPICallResult::new(resp)
//     }
// }

// #[pin_project]
// pub struct DeribitAPICallRawResult<R> {
//     #[pin]
//     rx: Timeout<oneshot::Receiver<String>>,
//     _ty: PhantomData<R>,
// }

// impl<R> DeribitAPICallRawResult<R> {
//     pub(crate) fn new(rx: oneshot::Receiver<String>, expiry: Duration) -> Self {
//         DeribitAPICallRawResult {
//             rx: timeout(expiry, rx),
//             _ty: PhantomData,
//         }
//     }
// }

// impl<R> Future for DeribitAPICallRawResult<R>
// where
//     R: DeserializeOwned,
// {
//     type Output = Result<JSONRPCResponse<R>>;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<JSONRPCResponse<R>>> {
//         let this = self.project();
//         match this.rx.poll(cx) {
//             Poll::Ready(Ok(ret)) => Poll::Ready(match ret {
//                 Ok(resp) => {
//                     let result: StdResult<JSONRPCResponse<R>, _> = from_str(&resp);
//                     if let Err(_) = result.as_ref() {
//                         error!("[API Client] Cannot deserialize RPC response: {}", resp);
//                     }
//                     result.map_err(Into::into)
//                 }
//                 Err(err) => Err(err.into()),
//             }),
//             Poll::Ready(Err(Elapsed { .. })) => {
//                 Poll::Ready(Err(DeribitError::RequestTimeout.into()))
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }
// #[pin_project]
// pub struct DeribitAPICallResult<R> {
//     #[pin]
//     inner: DeribitAPICallRawResult<R>,
// }

// impl<R> DeribitAPICallResult<R> {
//     pub(crate) fn new(inner: DeribitAPICallRawResult<R>) -> Self {
//         DeribitAPICallResult { inner: inner }
//     }
// }

// impl<R> Future for DeribitAPICallResult<R>
// where
//     R: DeserializeOwned,
// {
//     type Output = Result<R>;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<R>> {
//         let this = self.project();
//         match this.inner.poll(cx) {
//             Poll::Ready(Ok(resp)) => Poll::Ready(resp.result.left_result().map_err(|e| {
//                 DeribitError::RemoteError {
//                     code: e.code,
//                     message: e.message,
//                 }
//                 .into()
//             })),
//             Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }





// /*|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||**
                        
//                             END 
//                         CLIENT MODULE

// **|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||*/








// #[throws(Error)]
// #[tokio::main]
// fn main() {

//     let _ = dotenv();
//     init();

//     let drb = DeribitBuilder::default()
//     .subscription_buffer_size(100000usize)
//     .build()
//     .unwrap();

//     let mut subscription = drb.connect();

//     let req = PublicSubscribeRequest::new(&["book.BTC-PERPETUAL.100ms".into()]);

//     let _ = client.call(req);

//     client.call(SetHeartBeatRequest::with_interval(30));

//     while let Some(m) = subscription.next() {
//         if let SubscriptionParams::Heartbeat {
//             r#type: HeartBeatType::TestRequest,
//         } = m.params 
//         {
//             client.call(TestRequest::default());
//         }
//     }
// }