use std::fmt::Debug;

use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc;
use super::TenoriteError;
use super::TenoriteRequest;

#[derive(Clone)]
pub struct TenoriteClient<Request, Response, Error> {
    pub sender: mpsc::Sender<TenoriteRequest<Request, Response, Error>>,
}

impl <Request, Response, Error> TenoriteClient<Request, Response, Error>
where Error: Debug
{
    pub async fn send_request(
        &mut self,
        request: Request,
    ) -> Result<Response, TenoriteError<Error, SendError<TenoriteRequest<Request, Response, Error>>>> {
        let (service_request, receiver) = TenoriteRequest::new(request);
        match self.sender.send(service_request).await {
            Ok(_) => match receiver.await {
                Ok(response) => match response {
                    Ok(response) => {
                        Ok(response)
                    },
                    Err(error) => {
                        Err(TenoriteError::ServiceError(error))
                    }
                }
                Err(error) => {
                    Err(TenoriteError::FetchFailure(error))
                }
            },
            Err(error) => {
                Err(TenoriteError::SendFailure(error))
            }
        }
    }
}