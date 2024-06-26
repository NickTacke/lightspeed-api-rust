use crate::LightspeedClient;
use crate::models::Account;
use std::error::Error;

pub struct AccountResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> AccountResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<Account, Box<dyn Error>> {
        self.client.read("account", None).await
    }
}