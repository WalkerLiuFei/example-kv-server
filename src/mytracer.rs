use std::time::Duration;

use opentelemetry::global;
use tracing_subscriber::Registry;
use tracing_subscriber::prelude::*;

pub fn init_global_tracer() {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("grpc-server")
        .install_batch(opentelemetry::runtime::Tokio).expect("failed to install_batch");

    Registry::default()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init().expect("failed to init tracing_subscriber");


 }


