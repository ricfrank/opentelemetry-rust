use actix_web::{get, post, web, App, HttpResponse, Responder};

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
    // telemetry::init_tracer().expect("Failed to initialize telemetry");
    services::server::run(|| App::new().service(web::resource("/add").to(add)), "8080", "add").await?;
    Ok(())
}

async fn add(values: web::Json<Vec<i64>>) -> web::Json<i64> {
    web::Json(values.iter().sum())
}
