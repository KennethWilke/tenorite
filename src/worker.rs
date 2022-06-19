use async_trait::async_trait;
use std::fmt::Debug;
use tokio::sync::mpsc::Receiver;

use crate::request::TenoriteRequest;

/// This trait specifies the worker end of the service. the `task()` method is
/// what implements the server side logic.
#[async_trait]
pub trait TenoriteWorker<Request, Response, Error, TaskConfig, TaskResult = ()>
where
    Request: Debug,
    Response: Debug,
    Error: Debug,
{
    /// This task does the work, put your great stuff in here!
    async fn task(
        mut receiver: Receiver<TenoriteRequest<Request, Response, Error>>,
        config: TaskConfig,
    ) -> TaskResult;
}
