use tokio::sync::{mpsc, oneshot};

use crate::request::TenoriteRequest;

/// This is the error type returned by [`super::TenoriteCaller::send_request`]
pub enum TenoriteError<Request, Response, Error> {
    /// Returned when there is an error sending the request to the service
    SendFailure(mpsc::error::SendError<TenoriteRequest<Request, Response, Error>>),
    /// Returned when there is an error fetching the response from the service
    FetchFailure(oneshot::error::RecvError),
    /// Returned when the service bubbles up an error
    ServiceError(Error),
}
