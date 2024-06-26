use crate::LightspeedClient;
use crate::models::Product;
use std::error::Error;

pub struct ProductResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> ProductResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, product: &Product) -> Result<Product, Box<dyn Error>> {
        self.client.create("products", product).await
    }

    pub async fn get(&self, id: &str) -> Result<Product, Box<dyn Error>> {
        self.client.read(&format!("products/{}", id), None).await
    }

    pub async fn update(&self, id: &str, product: &Product) -> Result<Product, Box<dyn Error>> {
        self.client.update(&format!("products/{}", id), product).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.client.delete(&format!("products/{}", id)).await
    }
}