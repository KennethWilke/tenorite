use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use super::TenoriteCaller;
use super::TenoriteWorker;

/// Binds together the generic type components into the service
pub trait TenoriteService<Request, Response, Error, Task, TaskConfig>
where
    Request: Send + 'static,
    Response: Send + 'static,
    Error: Send + 'static,
    Task: TenoriteWorker<Request, Response, Error, TaskConfig>,
    TaskConfig: Send + 'static,
{
    /// Begins the task thread with the given configuration, returns the task
    /// and a client handle for communicating with the server
    fn start_task(
        &self,
        backlog_size: usize,
        config: TaskConfig,
    ) -> (JoinHandle<()>, TenoriteCaller<Request, Response, Error>) {
        let (sender, receiver) = mpsc::channel(backlog_size);
        let task = tokio::spawn(async move {
            <Task as TenoriteWorker<Request, Response, Error, TaskConfig>>::task(receiver, config)
                .await
        });
        (
            task,
            TenoriteCaller::<Request, Response, Error> { handle: sender },
        )
    }
}
