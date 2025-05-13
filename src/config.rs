use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_port: u16,
    pub pubsub_component: String,
    pub topic: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .set_default("SERVER_PORT", 3000)?
            .set_default("PUBSUB_COMPONENT", "nats-pubsub")?
            .set_default("TOPIC", "my-topic")?;

        builder.build()?.try_deserialize()
    }
}