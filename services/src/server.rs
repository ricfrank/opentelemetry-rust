use actix_http::body::MessageBody;
use actix_service::ServiceFactory;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    App, HttpServer,
};

use crate::telemetry;

pub async fn run<T, B>(app_builder: fn() -> App<T>, port: &str) -> std::io::Result<()>
where
    // B: 'static + MessageBody<Error = actix_http::Error>,
    B: 'static + MessageBody<Error = std::boxed::Box<(dyn std::error::Error + 'static)>>,
    T: 'static
        + ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<B>,
            Error = actix_web::Error,
            InitError = (),
        >,
{
    telemetry::init_tracer().expect("Failed to initialize telemetry");
    HttpServer::new(move || app_builder().wrap(actix_web_opentelemetry::RequestTracing::new()))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await?;

    Ok(())
}
