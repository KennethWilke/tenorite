use tokio::sync::oneshot;

pub enum TenoriteError<Error, SendError> {
    SendFailure(SendError),
    FetchFailure(oneshot::error::RecvError),
    ServiceError(Error),
}
