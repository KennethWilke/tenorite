use tokio::sync::oneshot;

pub type Responder<Response, Error> = oneshot::Sender<Result<Response, Error>>;
pub type Respondee<Response, Error> = oneshot::Receiver<Result<Response, Error>>;

pub struct TenoriteRequest<Request, Response, Error> {
    pub request: Request,
    pub client: Responder<Response, Error>
}

impl <Request, Response, Error> TenoriteRequest<Request, Response, Error> {
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

fn make_responder_channel<Response, Error>() -> (Responder<Response, Error>, Respondee<Response, Error>) {
    oneshot::channel()
}