use opentelemetry::global;
use opentelemetry::trace::TraceError;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;

pub fn init_tracer(service_name: &str) -> Result<(), TraceError> {
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(service_name)
        .install_batch(opentelemetry::runtime::Tokio)?;

    // global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    global::set_text_map_propagator(TraceContextPropagator::new());

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
