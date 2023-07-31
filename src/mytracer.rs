use opentelemetry::global;
use tracing::{info, Subscriber};
use tracing::instrument::WithSubscriber;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_global_tracer() {
    // Initialize the global tracer
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("grpc-server")
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("failed to install_batch");

     // Initialize tracing_subscriber next
    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer()
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_span_events(FmtSpan::ACTIVE)
        )
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .event(|event: &tracing::Event| {
            info!(parent: event.parent(), "tracing event: {}", event.name());
        })
        .with(tracing_subscriber::EnvFilter::from_default_env());

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global default subscriber");

    // Now you can use tracing and env_logger
    info!("This is an INFO message");
}
