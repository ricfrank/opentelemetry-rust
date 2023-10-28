use anyhow::Result;
use opentelemetry::global;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;

pub fn init_tracer() -> Result<()> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("otpl-rs")
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let env_filter = EnvFilter::new("INFO");
    let formatting_layer = BunyanFormattingLayer::new("otpl-rs".into(), std::io::stdout);

    let suscriber = tracing_subscriber::Registry::default()
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(env_filter);

    tracing::subscriber::set_global_default(suscriber).expect("setting default subscriber failed");

    Ok(())
}
