use std::thread;

use crate::telemetry;
use actix_http::body::MessageBody;
use actix_service::ServiceFactory;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    App, HttpServer,
};
use actix_web_prom::PrometheusMetricsBuilder;
use prometheus::Gauge;
use systemstat::{Platform, System, Duration};

pub async fn run<T, B>(
    app_builder: fn() -> App<T>,
    port: &str,
    service_name: &str,
) -> std::io::Result<()>
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
    let sys = System::new();
    telemetry::init_tracer(service_name).expect("Failed to initialize telemetry");
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    let cpu_usage = Gauge::new("cpu_usage", "Current CPU usage in percent").unwrap();
    let mem_usage = Gauge::new("mem_usage", "Current memory usage in percent").unwrap();

    prometheus
        .registry
        .register(Box::new(cpu_usage.clone()))
        .unwrap();

    prometheus
        .registry
        .register(Box::new(mem_usage.clone()))
        .unwrap();

    thread::spawn(move || loop {
        match sys.cpu_load_aggregate() {
            Ok(cpu) => {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                cpu_usage.set(f64::trunc(
                    ((cpu.system * 100.0) + (cpu.user * 100.0)).into(),
                ));
            }
            Err(x) => println!("\nCPU load: error: {}", x),
        }
        match sys.memory() {
            Ok(mem) => {
                let memory_used = mem.total.0 - mem.free.0;
                let pourcentage_used = (memory_used as f64 / mem.total.0 as f64) * 100.0;
                mem_usage.set(f64::trunc(pourcentage_used));
            }
            Err(x) => println!("\nMemory: error: {}", x),
        }
    });

    HttpServer::new(move || {
        app_builder()
            .wrap(prometheus.clone())
            .wrap(actix_web_opentelemetry::RequestTracing::new())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
