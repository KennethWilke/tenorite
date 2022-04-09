use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::TenoriteRequest;

/// This trait specifies the worker end of the service. the `task()` method is
/// what implements the server side logic.
#[async_trait]
pub trait TenoriteWorker<Request, Response, Error, TaskConfig> {
    async fn task(
        mut receiver: Receiver<TenoriteRequest<Request, Response, Error>>,
        config: TaskConfig,
    );
}
