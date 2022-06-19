#[macro_export]
macro_rules! tenorite_service {
    ($new_service_type:ident, $request:ty, $response:ty, $error:ty, $worker:ty, $config:ty) => {
        pub struct $new_service_type {}

        impl tenorite::TenoriteService<$request, $response, $error, $worker, $config, ()>
            for $new_service_type
        {
        }
    };
}
