use opentelemetry::global;
use opentelemetry::propagation::{Extractor, Injector};
use tonic::{Request, Status};
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::service::Interceptor;
use tracing_attributes::instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct MyInterceptor;

impl Interceptor for MyInterceptor {
    #[instrument(name = "interceptor", skip(self, request) )]
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {

        // use x-request-id as trace id
        match request.metadata().get("x-request-id") {
            None => {
                let request_id = Uuid::new_v4().to_string();
                let request_id = MetadataValue::try_from(&request_id).unwrap();
                request.metadata_mut().insert("x-request-id", request_id);
            }
            Some(_) => {}
        }



        if !request.metadata().contains_key("traceparent") {
            global::get_text_map_propagator(|propagator| {
                propagator.inject_context(
                    &tracing::Span::current().context(),
                    &mut MutMetadataMap(request.metadata_mut()),
                )
            });
        }

        Ok(request)
    }
}



pub struct MyExtractor<'a, T> (pub &'a Request<T>);

impl<'a, T> Extractor for MyExtractor<'a, T> {
    fn get(&self, key: &str) -> Option<&str> {
        match self.0.metadata().get(key) {
            Some(v) => v.to_str().ok(),
            None => None
        }
    }

    fn keys(&self) -> Vec<&str> {
        self.0.metadata().keys().map(|key| match key {
            tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
            tonic::metadata::KeyRef::Binary(v) => v.as_str(),
        }).collect::<Vec<_>>()
    }
}


pub struct MutMetadataMap<'a>(pub &'a mut MetadataMap);

impl<'a> Injector for MutMetadataMap<'a> {
    /// Set a key and value in the MetadataMap.  Does nothing if the key or value are not valid inputs
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(val) = tonic::metadata::MetadataValue::try_from(&value) {
                self.0.insert(key, val);
            }
        }
    }
}
