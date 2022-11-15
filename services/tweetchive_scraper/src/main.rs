use opentelemetry::sdk::export::trace::stdout;
use tikv_jemallocator::Jemalloc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

static GLOBAL: Jemalloc = Jemalloc;

mod config;
mod user;

#[tokio::main]
async fn main() {
    let tracer = stdout::new_pipeline().install_simple();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
