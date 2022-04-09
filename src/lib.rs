//! Welcome to the Tenorite docs! My documentation is sparse at the moment, so
//! I recommend taking a look at my [example repo](https://github.com/KennethWilke/tenorite-example)
//! to get an idea for how everything fits together.

/// The [`TenoriteCaller`] struct, which provides the generic request/reply pattern
pub mod caller;

/// The internal [`TenoriteError`] enumeration
pub mod error;

/// The [`TenoriteRequest`] request encapsulation structure
pub mod request;

/// The [`TenoriteService`] trait to glue together a custom service
pub mod service;

/// The [`TenoriteWorker`] trait, which requires implementation of the task to be started by [`TenoriteService`]
pub mod worker;

pub use caller::TenoriteCaller;
pub use error::TenoriteError;
pub use request::TenoriteRequest;
pub use service::TenoriteService;
pub use worker::TenoriteWorker;

// Re-exports
pub use async_trait::async_trait;
pub use tokio::sync::mpsc::Receiver;
