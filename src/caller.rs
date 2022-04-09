use super::TenoriteError;
use super::TenoriteRequest;
use tokio::sync::mpsc;

/// Holds the client side handle of the communication
#[derive(Clone)]
pub struct TenoriteCaller<Request, Response, Error> {
    pub handle: mpsc::Sender<TenoriteRequest<Request, Response, Error>>,
}

impl<Request, Response, Error> TenoriteCaller<Request, Response, Error> {
    /// Sends the request through the underlying caller handle and waits for
    /// the response from the service
    pub async fn send_request(
        &mut self,
        request: Request,
    ) -> Result<Response, TenoriteError<Request, Response, Error>> {
        let (service_request, receiver) = TenoriteRequest::new(request);
        match self.handle.send(service_request).await {
            Ok(_) => match receiver.await {
                Ok(response) => match response {
                    Ok(response) => Ok(response),
                    Err(error) => Err(TenoriteError::ServiceError(error)),
                },
                Err(error) => Err(TenoriteError::FetchFailure(error)),
            },
            Err(error) => Err(TenoriteError::SendFailure(error)),
        }
    }
}
