use tracing::subscriber::SetGlobalDefault;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    LogTracer::init()?;
    
    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let formatting_layer = BunyanFormattingLayer::new("rust-systems-project-0".into(), std::io::stdout);
    
    let subscriber = Registry::default()
        .with(filter_layer)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    
    SetGlobalDefault::set_global_default(subscriber)?;
    
    Ok(())
}
