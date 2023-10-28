use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;

mod telemetry;

#[get("/")]
#[tracing::instrument]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    telemetry::init_tracer().expect("Failed to initialize telemetry");

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
