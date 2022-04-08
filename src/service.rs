use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use super::TenoriteClient;
use super::TenoriteTask;

pub trait TenoriteService<Request, Response, Error, Task, TaskConfig>
where
    Request: Send + 'static,
    Response: Send + 'static,
    Error: Send + 'static,
    Task: TenoriteTask<Request, Response, Error, TaskConfig>,
    TaskConfig: Send + 'static,
{
    fn start_task(
        &self,
        backlog_size: usize,
        config: TaskConfig,
    ) -> (JoinHandle<()>, TenoriteClient<Request, Response, Error>) {
        let (sender, receiver) = mpsc::channel(backlog_size);
        let task = tokio::spawn(async move {
            <Task as TenoriteTask<Request, Response, Error, TaskConfig>>::task(receiver, config)
                .await
        });
        (task, TenoriteClient::<Request, Response, Error> { sender })
    }
}
