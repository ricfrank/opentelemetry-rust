use actix_web::{get, post, web, App, HttpResponse, Responder, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;

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
    // let prometheus = PrometheusMetricsBuilder::new("api")
    //     .endpoint("/metrics")
    //     .build()
    //     .unwrap();
    // HttpServer::new(move || {
    //     App::new()
    //         .wrap(prometheus.clone())
    //         .service(hello)
    // })
    // .bind(("127.0.0.1", 4000))?
    //     .run()
    //     .await
    services::server::run(
        || {
            App::new()
                // .wrap(prometheus.clone())
                .service(web::resource("/add").to(add))
        },
        "8080",
        "add",
    )
    .await?;
    Ok(())
}

async fn add(values: web::Json<Vec<i64>>) -> web::Json<i64> {
    web::Json(values.iter().sum())
}
