use std::fmt::Debug;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use super::caller::TenoriteCaller;
use super::worker::TenoriteWorker;

/// Binds together the generic type components into the service
pub trait TenoriteService<Request, Response, Error, Task, TaskConfig, TaskResult>
where
    Request: Send + Debug + 'static,
    Response: Send + Debug + 'static,
    Error: Send + Debug + 'static,
    Task: TenoriteWorker<Request, Response, Error, TaskConfig, TaskResult>,
    TaskConfig: Send + 'static,
    TaskResult: Send + 'static
{
    /// Begins the task thread with the given configuration, returns the task
    /// and a client handle for communicating with the server
    fn start_task(
        &self,
        backlog_size: usize,
        config: TaskConfig,
    ) -> (JoinHandle<TaskResult>, TenoriteCaller<Request, Response, Error>) {
        let (sender, receiver) = mpsc::channel(backlog_size);
        let task = tokio::spawn(async move {
            <Task as TenoriteWorker<Request, Response, Error, TaskConfig, TaskResult>>::task(receiver, config)
                .await
        });
        (
            task,
            TenoriteCaller::<Request, Response, Error> { handle: sender },
        )
    }
}
