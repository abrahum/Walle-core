#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const WALLE_CORE: &str = "Walle-core";

pub mod action;
#[cfg(feature = "alt")]
pub mod alt;
pub mod config;
pub mod error;
pub mod event;
pub mod resp;
pub mod segment;
pub mod structs;
pub mod util;

mod ah;
pub use ah::{ActionHandler, GetStatus};
mod eh;
pub use eh::EventHandler;

#[cfg(any(feature = "impl-obc", feature = "app-obc"))]
pub mod obc;
#[cfg(test)]
mod test;

pub mod prelude {
    pub use crate::resp::{resp_error, Resp};
    pub use crate::segment::{IntoMessage, MessageExt, MessageSegment, Segments};

    pub use super::*;
    pub use crate::error::{WalleError, WalleResult};
    pub use crate::util::{Echo, OneBotBytes, SelfId, Value, ValueMap, ValueMapExt};
    pub use crate::{value, value_map, value_vec};
    pub use async_trait::async_trait;
    pub use walle_macro::{OneBot, PushToValueMap};
}

/// AH: EventConstructor + ActionHandler
/// EH: EventHandler + ActionConstructor
pub struct OneBot<AH, EH> {
    pub action_handler: AH,
    pub event_handler: EH,
    // Some for running, None for stopped
    signal: std::sync::Mutex<Option<tokio::sync::broadcast::Sender<()>>>,
}

use std::sync::Arc;

use crate::error::{WalleError, WalleResult};

impl<AH, EH> OneBot<AH, EH> {
    pub fn new(action_handler: AH, event_handler: EH) -> Self {
        Self {
            action_handler,
            event_handler,
            signal: std::sync::Mutex::new(None),
        }
    }
    pub async fn start<E, A, R>(
        self: &Arc<Self>,
        ah_config: AH::Config,
        eh_config: EH::Config,
        ah_first: bool,
    ) -> WalleResult<Vec<tokio::task::JoinHandle<()>>>
    where
        E: Send + Sync + 'static,
        A: Send + Sync + 'static,
        R: Send + Sync + 'static,
        AH: ActionHandler<E, A, R> + Send + Sync + 'static,
        EH: EventHandler<E, A, R> + Send + Sync + 'static,
    {
        let mut signal = self.signal.lock().unwrap();
        if signal.is_none() {
            let (tx, _) = tokio::sync::broadcast::channel(1);
            *signal = Some(tx);
        } else {
            return Err(WalleError::AlreadyStarted);
        }
        drop(signal);
        let mut tasks = vec![];
        if ah_first {
            tasks.extend(
                self.action_handler
                    .start(self, ah_config)
                    .await?
                    .into_iter(),
            );
            tasks.extend(self.event_handler.start(self, eh_config).await?.into_iter());
        } else {
            tasks.extend(self.event_handler.start(self, eh_config).await?.into_iter());
            tasks.extend(
                self.action_handler
                    .start(self, ah_config)
                    .await?
                    .into_iter(),
            );
        }
        Ok(tasks)
    }
    pub fn started(&self) -> bool {
        self.signal.lock().unwrap().is_some()
    }
    pub fn get_signal_rx(&self) -> WalleResult<tokio::sync::broadcast::Receiver<()>> {
        Ok(self
            .signal
            .lock()
            .unwrap()
            .as_ref()
            .ok_or(WalleError::NotStarted)?
            .subscribe())
    }
    pub async fn shutdown<E, A, R>(&self) -> WalleResult<()>
    where
        E: Send + Sync + 'static,
        A: Send + Sync + 'static,
        R: Send + Sync + 'static,
        AH: ActionHandler<E, A, R> + Send + Sync + 'static,
        EH: EventHandler<E, A, R> + Send + Sync + 'static,
    {
        let tx = self
            .signal
            .lock()
            .unwrap()
            .take()
            .ok_or(WalleError::NotStarted)?;
        tx.send(()).ok();
        self.action_handler.shutdown().await;
        self.event_handler.shutdown().await;
        Ok(())
    }
    pub async fn handle_event<E, A, R>(self: &Arc<Self>, event: E) -> WalleResult<()>
    where
        AH: ActionHandler<E, A, R> + Send + Sync + 'static,
        EH: EventHandler<E, A, R> + Send + Sync + 'static,
        E: Send + 'static,
    {
        self.event_handler
            .call(self.action_handler.before_call_event(event).await?)
            .await?;
        self.action_handler.after_call_event().await
    }
    pub async fn handle_action<E, A, R>(self: &Arc<Self>, action: A) -> WalleResult<R>
    where
        AH: ActionHandler<E, A, R> + Send + Sync + 'static,
        EH: EventHandler<E, A, R> + Send + Sync + 'static,
        A: Send + 'static,
        R: Send + 'static,
    {
        self.event_handler
            .after_call_action(
                self.action_handler
                    .call(self.event_handler.before_call_action(action).await?)
                    .await?,
            )
            .await
    }
}
