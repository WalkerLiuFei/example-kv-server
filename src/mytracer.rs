use opentelemetry::{global, trace};
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::trace::FutureExt;
use tracing::{Event, Id, info, span, Span, Subscriber};
use tracing::field::Field;
use tracing::span::Attributes;
use tracing_subscriber::{Layer, Registry};
use tracing_subscriber::layer::Context;
use tracing_subscriber::prelude::*;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use crate::config;

pub fn init_global_tracer() {
    // Initialize the global tracer
    //global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
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



#[derive(Default)]
struct CustomSubscriber<S> {
    inner: S,
}

impl<S> Layer<S> for CustomSubscriber<S>
    where
        S: tracing::Subscriber,
        S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(&self, _event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        println!("Got on_event! ctx={:?}", ctx.current_span());

        let span = ctx.current_span();
        match _event.parent() {
            None => {}
            Some(p) => {
                println!("parent:{:?}", p);
            }
        }
        //println!("Got on_new_span! {:?} ",tracing::);
        // global::get_text_map_propagator(|propagator| {
        //
        // });

        //assert_eq!(, Some(&ValueA("a")));
        //Span::current().context().get() //.get<trace::TraceId>();
        //let  trace_id = ;
        match Span::current().field("trace_id") {
            None => {}
            Some(field) => {
                println!("  trace id 1={:?}", field);
            }
        }
        match span.metadata() {
            None => {

            }
            Some(md) => {
                println!("  level={:?}", span.metadata().unwrap().level());
                println!("  target={:?}", span.metadata().unwrap().target());
                println!("  name={:?}", span.metadata().unwrap().name());
                println!("  fields={:?}", span.metadata().unwrap().fields());
                println!("  span id={:?}", span.id().unwrap());
                //println!(" trace id= {:?}", _event.fields().)
            }
        }

        //  span.id();
        // span.parent().map(|parent| {
        //     println!("  parent={:?}", parent.id());
        // });
    }
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        println!("Got on_new_span! {:?} ", id);
        attrs.fields().iter().for_each(|(k)| {
            println!("  {:?}", k);
        });
    }
    //
    // fn on_event(&self, _event: &Event<'_>, ctx: Context<S>) {

    // }
}