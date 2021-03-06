use opentelemetry::exporter::trace::stdout;
use opentelemetry::{
    api::{Provider, TracerGenerics},
    global, sdk,
};

fn main() {
    let exporter = stdout::Builder::default().init();
    let provider = sdk::Provider::builder()
        .with_simple_exporter(exporter)
        .with_config(sdk::Config {
            default_sampler: Box::new(sdk::Sampler::Always),
            ..Default::default()
        })
        .build();
    global::set_provider(provider);

    global::trace_provider()
        .get_tracer("component-main")
        .with_span("operation", move |_span| {});
}
