use std::fmt::Debug;

use serde::{Deserialize, Serialize};

mod bytes;
mod echo;
pub mod value;

pub use bytes::*;
pub use echo::*;
pub use value::*;

pub fn timestamp_nano() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}

pub fn timestamp_nano_f64() -> f64 {
    timestamp_nano() as f64 / 1_000_000_000.0
}

#[cfg(feature = "impl-obc")]
pub fn new_uuid() -> String {
    uuid::Uuid::from_u128(timestamp_nano()).to_string()
}

pub trait SelfId: Sized {
    fn self_id(&self) -> String;
}

#[async_trait::async_trait]
pub trait SelfIds {
    async fn self_ids(&self) -> Vec<String>;
}

#[doc(hidden)]
pub trait ProtocolItem:
    Serialize + for<'de> Deserialize<'de> + Debug + Send + Sync + 'static
{
    fn json_encode(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn json_decode(s: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
    fn rmp_encode(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }
    fn rmp_decode(v: &[u8]) -> Result<Self, String>
    where
        Self: Sized,
    {
        rmp_serde::from_slice(v).map_err(|e| e.to_string())
    }
    #[cfg(feature = "http")]
    fn to_body(self, content_type: &ContentType) -> hyper::Body {
        match content_type {
            ContentType::Json => hyper::Body::from(self.json_encode()),
            ContentType::MsgPack => hyper::Body::from(self.rmp_encode()),
        }
    }
    #[cfg(feature = "websocket")]
    fn to_ws_msg(self, content_type: &ContentType) -> tokio_tungstenite::tungstenite::Message {
        match content_type {
            ContentType::Json => tokio_tungstenite::tungstenite::Message::Text(self.json_encode()),
            ContentType::MsgPack => {
                tokio_tungstenite::tungstenite::Message::Binary(self.rmp_encode())
            }
        }
    }
}

impl<T> ProtocolItem for T where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Send + Sync + 'static
{
}

/// Onebot ?????????????????????????????????
///
/// Json or MessagePack
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Json,
    MsgPack,
}

impl ContentType {
    #[allow(dead_code)]
    pub fn new(s: &str) -> Option<Self> {
        match s {
            "application/json" => Some(Self::Json),
            "application/msgpack" => Some(Self::MsgPack),
            _ => None,
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "application/json"),
            Self::MsgPack => write!(f, "application/msgpack"),
        }
    }
}

pub(crate) trait AuthReqHeaderExt {
    fn header_auth_token(self, token: &Option<String>) -> Self;
}

#[cfg(feature = "http")]
use hyper::http::{header::AUTHORIZATION, request::Builder};
#[cfg(all(feature = "websocket", not(feature = "http")))]
use tokio_tungstenite::tungstenite::http::{header::AUTHORIZATION, request::Builder};

#[cfg(any(feature = "websocket", feature = "http"))]
impl AuthReqHeaderExt for Builder {
    fn header_auth_token(self, token: &Option<String>) -> Self {
        if let Some(token) = token {
            self.header(AUTHORIZATION, format!("Bearer {}", token))
        } else {
            self
        }
    }
}
