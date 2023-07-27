use std::time::Duration;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use uuid::{uuid, Uuid};

#[derive(Debug, Deserialize)]
pub struct Config {
    /// The Bluetooth service class UUIDs to listen for.
    /// Defaults to AudioSinkServiceClass.
    #[serde(default = "service_uuids_default")]
    pub service_uuids: Vec<Uuid>,
    #[serde(with = "serde_millis", default = "connect_timeout_default")]
    pub connect_timeout: Duration,
}

impl Config {
    pub fn load() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::raw())
            .extract()
    }
}

#[inline]
fn service_uuids_default() -> Vec<Uuid> {
    vec![uuid!("0000110b-0000-1000-8000-00805f9b34fb")]
}

#[inline]
fn connect_timeout_default() -> Duration {
    Duration::from_millis(10_000)
}
