use std::fmt::Debug;
use tokio::sync::{mpsc, oneshot};

use crate::request::TenoriteRequest;

/// This is the error type returned by [`super::TenoriteCaller::send_request`]
#[derive(thiserror::Error, Debug)]
pub enum TenoriteError<Request, Response, Error>
where
    Request: Debug,
    Response: Debug,
    Error: Debug,
{
    /// Returned when there is an error sending the request to the service
    #[error("failed to send request to service: {0}")]
    SendFailure(mpsc::error::SendError<TenoriteRequest<Request, Response, Error>>),
    /// Returned when there is an error fetching the response from the service
    #[error("failed to fetch response from service: {0}")]
    FetchFailure(oneshot::error::RecvError),
    /// Returned when the service bubbles up an error
    #[error("service error: {0}")]
    ServiceError(Error),
}
