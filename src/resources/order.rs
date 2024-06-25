use crate::LightspeedClient;
use std::error::Error;

pub struct OrderResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> OrderResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.read(&format!("orders/{}", id), None).await
    }

    pub async fn list(&self, params: Option<Vec<(&str, &str)>>) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.read("orders", params).await
    }

    pub async fn update(&self, id: &str, order: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.update(&format!("orders/{}", id), order).await
    }

    pub async fn count(&self, params: Option<Vec<(&str, &str)>>) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.read("orders/count", params).await
    }
}