use actix_web::{web, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    services::server::run(|| App::new().service(web::resource("/sub").to(sub)), "8081").await?;
    Ok(())
}

async fn sub(values: web::Json<Vec<i64>>) -> web::Json<i64> {
    // telemetry::init_tracer().expect("Failed to initialize telemetry");
    let mut total = values[0];
    for v in values.iter().skip(1) {
        total -= v;
    }

    web::Json(total)
}
