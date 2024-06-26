use crate::{LightspeedClient, Product, ProductWrapper, Products, Count};
use std::error::Error;

pub struct ProductResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> ProductResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn get_all(&self, params: Option<Vec<(&str, &str)>>) -> Result<Products, Box<dyn Error>> {
        self.client.read("products", params).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Product, Box<dyn Error>> {
        self.client.read(&format!("products/{}", id), None).await
    }

    pub async fn create(&self, product: Product) -> Result<Product, Box<dyn Error>> {
        let wrapper = ProductWrapper{product};
        let response: ProductWrapper = self.client.create("products", &wrapper).await?;
        Ok(response.product)
    }

    pub async fn update(&self, id: &str, product: Product) -> Result<Product, Box<dyn Error>> {
        let wrapper = ProductWrapper{product};
        let response: ProductWrapper = self.client.update(&format!("products/{}", id), &wrapper).await?;
        Ok(response.product)
    }

    pub async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.client.delete(&format!("products/{}", id)).await
    }

    pub async fn count(&self, params: Option<Vec<(&str, &str)>>) -> Result<Count, Box<dyn Error>> {
        self.client.read("products/count", params).await
    }
}