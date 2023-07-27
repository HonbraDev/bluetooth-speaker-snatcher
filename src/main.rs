mod config;
mod tracing;

use ::tracing::{debug, info};
use bluez_async::{BluetoothEvent, BluetoothSession, DeviceEvent, DiscoveryFilter};
use eyre::Context;
use futures_util::StreamExt;

use crate::{config::Config, tracing::set_up_tracing};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    set_up_tracing("bluetooth_speaker_snatcher").context("Failed to set up tracing")?;

    let Config {
        service_uuids,
        connect_timeout,
    } = Config::load().context("Failed to load config")?;

    let (_, session) = BluetoothSession::new()
        .await
        .context("Failed to open Bluetooth session")?;

    info!("Created BlueZ session");

    let filter = DiscoveryFilter {
        service_uuids,
        ..Default::default()
    };

    session.start_discovery_with_filter(&filter).await?;

    let mut stream = session.event_stream().await?;

    while let Some(event) = stream.next().await {
        match event {
            BluetoothEvent::Device { id, event } => match event {
                DeviceEvent::Discovered => {
                    info!("{id} discovered, connecting");
                    match session.connect_with_timeout(&id, connect_timeout).await {
                        Ok(_) => info!("{id} connected successfully"),
                        Err(err) => info!("{id} failed to connect: {err}"),
                    }
                }
                event => debug!("Other device event: {event:?}"),
            },
            event => debug!("Other event: {event:?}"),
        }
    }

    Ok(())
}
