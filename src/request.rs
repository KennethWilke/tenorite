use tokio::sync::oneshot;

/// [`oneshot::Sender`] the service uses to reply to a client
pub type Responder<Response, Error> = oneshot::Sender<Result<Response, Error>>;
/// [`oneshot::Receiver`] the client receives replies through
pub type Respondee<Response, Error> = oneshot::Receiver<Result<Response, Error>>;

/// Encapsulation for requests from client to service
pub struct TenoriteRequest<Request, Response, Error> {
    /// Request from the client
    pub request: Request,
    /// Handle used by service to send response
    pub client: Responder<Response, Error>,
}

impl<Request, Response, Error> TenoriteRequest<Request, Response, Error> {
    /// Generates [`oneshot`] channel pair, effectively encoding a callback
    /// handle into the request
    pub fn new(request: Request) -> (Self, Respondee<Response, Error>) {
        let (sender, receiver) = make_responder_channel();
        (
            TenoriteRequest {
                request,
                client: sender,
            },
            receiver,
        )
    }
}

/// Generates the [`oneshot`] channel pair
fn make_responder_channel<Response, Error>(
) -> (Responder<Response, Error>, Respondee<Response, Error>) {
    oneshot::channel()
}
