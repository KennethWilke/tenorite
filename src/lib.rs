#![doc = include_str!("../README.md")]

/// The [`TenoriteCaller`] struct, which provides the generic request/reply pattern
pub mod caller;

/// The [`TenoriteError`] enumeration
pub mod error;

pub mod macros;

/// The [`TenoriteRequest`] request encapsulation structure
pub mod request;

/// The [`TenoriteService`] trait to glue together a custom service
pub mod service;

/// The [`TenoriteWorker`] trait, which is required for the task to be started by [`TenoriteService`]
pub mod worker;

pub use caller::TenoriteCaller;
pub use error::TenoriteError;
pub use request::TenoriteRequest;
pub use service::TenoriteService;
pub use worker::TenoriteWorker;

/// Re-exported from [`mod@async_trait`]
pub use async_trait::async_trait;

/// Re-export of [`tokio::sync::mpsc::Receiver`], used to define [`TenoriteWorker::task`]
pub use tokio::sync::mpsc::Receiver;
