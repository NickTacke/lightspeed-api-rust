use crate::LightspeedClient;
use std::error::Error;

pub struct AccountResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> AccountResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.read("account", None).await
    }
}