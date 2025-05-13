use serde::Serialize;
use dapr::client::{Client, TonicClient};
use std::collections::HashMap;

#[derive(Clone)]
pub struct DaprNatsClient {
    pubsub_component: String,
    topic: String,
    dapr: Client<TonicClient>,
}

const DAPR_ADDR: &str = "https://127.0.0.1";

impl DaprNatsClient {
    pub async fn new(pubsub_component: &str, topic: &str) -> Result<Self, anyhow::Error> {

        // Trying to connect to Dapr runtime
        let dapr = Client::<TonicClient>::connect(DAPR_ADDR.to_string())
            .await
            .map_err(|e| anyhow::anyhow!("❌ Error connecting to Dapr  {}", e))?;

        Ok(Self {
            pubsub_component: pubsub_component.to_string(),
            topic: topic.to_string(),
            dapr,
        })
    }



    pub async fn publish<T: Serialize + Sync>(&mut self, payload: T) -> Result<(), anyhow::Error>
    {
        let json = serde_json::to_vec(&payload)?;
        let metadata = None::<HashMap<String, String>>;
        let content_type = "application/json".to_string();

        self.dapr
            .publish_event(
                &self.pubsub_component,
                &self.topic,
                &content_type, 
                json,
                metadata,
            )
            .await?;

        Ok(())
    }
}