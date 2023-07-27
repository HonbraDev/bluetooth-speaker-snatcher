use eyre::{Context, Result};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn set_up_tracing(crate_name: &str) -> Result<()> {
    let self_filter = format!("{crate_name}=info")
        .parse()
        .context("failed to create default filter")?;

    let env_filter = EnvFilter::builder()
        .with_default_directive(self_filter)
        .from_env()
        .context("failed to parse env filter")?;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();

    Ok(())
}
