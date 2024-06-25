use crate::LightspeedClient;
use std::error::Error;

pub struct CustomerResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> CustomerResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, customer: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.create("customers", customer).await
    }

    pub async fn get(&self, id: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.read(&format!("customers/{}", id), None).await
    }

    pub async fn list(&self, params: Option<Vec<(&str, &str)>>) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.read("customers", params).await
    }

    pub async fn update(&self, id: &str, customer: &serde_json::Value) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.update(&format!("customers/{}", id), customer).await
    }

    pub async fn delete(&self, id: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.delete(&format!("customers/{}", id)).await
    }

    pub async fn count(&self, params: Option<Vec<(&str, &str)>>) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.read("customers/count", params).await
    }
}