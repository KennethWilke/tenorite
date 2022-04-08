use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::TenoriteRequest;

// Some kind of function pointer or closure may be better for this?
//  Not sure how to do that for async cleanly yet though
#[async_trait]
pub trait TenoriteTask<Request, Response, Error, TaskConfig> {
    async fn task(
        mut receiver: Receiver<TenoriteRequest<Request, Response, Error>>,
        config: TaskConfig,
    );
}
