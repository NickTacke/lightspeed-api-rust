use crate::LightspeedClient;
use std::error::Error;

pub struct ProductResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> ProductResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, product: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.create("products", product).await
    }

    pub async fn get(&self, id: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.read(&format!("products/{}", id), None).await
    }

    pub async fn update(&self, id: &str, product: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.update(&format!("products/{}", id), product).await
    }

    pub async fn delete(&self, id: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.delete(&format!("products/{}", id)).await
    }
}