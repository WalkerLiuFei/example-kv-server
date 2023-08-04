use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

use crate::config;

pub fn init_global_tracer() {
    // Initialize the global tracer
    //global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(config::CONFIG.jaeger_endpoint.clone().expect("jaeger_endpoint is not set"))
        .with_service_name(config::CONFIG.name.clone())
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("failed to install_batch");

    // Initialize tracing_subscriber next
    let subscriber = Registry::default()
        .with(
           // CustomSubscriber::default()
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
        ).with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_subscriber::EnvFilter::from_default_env());

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global default subscriber");

    // Now you can use tracing and env_logger
    info!("This is an INFO message");
}
