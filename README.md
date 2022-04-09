# Tenorite

Tenorite aims to simplify building concurrent systems with Rust. By building
simple abstractions over the solid foundation offered by Rust and Tokio,
Tenorite helps builds asynchronous workers that can service requests from other
threads using a client/server model.

## Example repository

Check out [the example repo](https://github.com/KennethWilke/tenorite-example)
to get an idea for how a `TenoriteService` can be built and used.

# Service Design

Tenorite services are created by feeding custom types into the generic type
parameters of the Tenorite core components. The general design flow involves
building a 4 data types and a worker, which are then bound together into an
easy to use service by implementing the `TenoriteSerivce` trait.

```
Service
  - Request
  - Response
  - Error
  - Worker
  - Config
```

## Request, Response, Error and Config

These structures are defined for each service. All of these types must meet
the `Send + 'static` trait bounds. `Request`, `Response` and `Error` must also
implement the `Clone` trait so that `TenoriteCaller` can be cloned to share
handles to the service.

Here's the example set of these types based on [the example repo](https://github.com/KennethWilke/tenorite-example):

```rust
#[derive(Debug, Clone)]
pub enum ExampleRequest {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

#[derive(Debug, Clone)]
pub enum ExampleResponse {
    EmptyResponse,
    StringResponse(String),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ExampleError {
    #[error("Invalid key!")]
    InvalidKey(String),
    #[error("Unexpected error!")]
    Unexpected,
}

pub struct ExampleConfig {
    pub data: std::collections::HashMap<String, String>,
}
```

## Worker type

The `TenoriteWorker` trait is fulfilled to provide the service/worker
implementation that will run within it's own tokio task. In the case of the
example project, it fully implements the "HashMap-as-a-Service" worker:

```rust
pub struct ExampleWorker {}

#[async_trait]
impl TenoriteWorker<ExampleRequest, ExampleResponse, ExampleError, ExampleConfig>
    for ExampleWorker
{
    async fn task(
        mut receiver: tenorite::Receiver<
            TenoriteRequest<ExampleRequest, ExampleResponse, ExampleError>,
        >,
        mut config: ExampleConfig,
    ) {
        while let Some(request) = receiver.recv().await {
            println!("[ExampleTask] Received Request: {:?}", request.request);

            use ExampleRequest::*;
            use ExampleResponse::*;
            let response = match request.request {
                Set { key, value } => {
                    config.data.insert(key, value);
                    Ok(EmptyResponse)
                }
                Get { key } => match config.data.get(&key) {
                    Some(value) => Ok(StringResponse(value.to_string())),
                    None => Err(ExampleError::InvalidKey(key)),
                },
                Delete { key } => match config.data.remove(&key) {
                    Some(_) => Ok(EmptyResponse),
                    None => Err(ExampleError::InvalidKey(key))
                }
            };

            match request.client.send(response) {
                Err(_result) => {
                    panic!("Error!!!!!")
                }
                _ => {}
            }
        }
    }
}
```

## Service type

The `Service` doesn't require much effort to build, simply include the needed
types!

```rust
pub struct ExampleService {}

impl TenoriteService<ExampleRequest, ExampleResponse, ExampleError, ExampleWorker, ExampleConfig>
    for ExampleService
{
}
```

# Service Usage

Using a service built with this pattern is even easier than building the
services! Instantiate instances of the custom `Service` and `Config` structs,
then start the worker task providing a queue size (32 here) and the `Config`
structure. This function returns a `JoinHandle` for the worker thread and a
`TenoriteCaller` structure that is used to make requests to the worker. This
caller handle can be cloned to share with other threads.

Using

```rust
let service = ExampleService {};
let config = ExampleConfig {
    data: HashMap::new(),
};
let (task, caller) = service.start_task(32, config);
```

The calling pattern is simple, modeled to flow as if you were calling an
ordinary async function in Rust. In this case, an async function that takes a
single `ExampleRequest` parameter and returns a `Result<ExampleResponse, TenoriteError>`
enumeration. If the service bubbles up the `ExampleError` structure, the error
will be returned within the `ServiceError(Error)` variant.

```rust
let key = "test".to_string();
let value = "weeee".to_string();
let request = ExampleRequest::Set { key, value };
let response = caller.send_request(request).await;
```

Lastly, when all of the `caller` handles fall out of scope and are dropped, the
`Worker` thread will terminate.

```rust
task.await;
```