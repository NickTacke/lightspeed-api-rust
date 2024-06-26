use crate::{LightspeedClient, Customer, CustomerWrapper, Customers, Count};
use std::error::Error;

pub struct CustomerResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> CustomerResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn get_all(&self, params: Option<Vec<(&str, &str)>>) -> Result<Customers, Box<dyn Error>> {
        self.client.read("customers", params).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Customer, Box<dyn Error>> {
        let order: Option<CustomerWrapper> = self.client.read(&format!("customers/{}", id), None).await?;
        
        Ok(order.and_then(| data | Some(data.customer)).ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No order found with given id!"
        ))?)
    }

    pub async fn create(&self, customer: Customer) -> Result<Customer, Box<dyn Error>> {
        let wrapper = CustomerWrapper { customer };
        let response: CustomerWrapper = self.client.create("customers", &wrapper).await?;
        Ok(response.customer)
    }

    pub async fn update(&self, id: &str, customer: Customer) -> Result<Customer, Box<dyn Error>> {
        let wrapper = CustomerWrapper{customer};
        let response: CustomerWrapper = self.client.update(&format!("customers/{}", id), &wrapper).await?;
        Ok(response.customer)
    }

    pub async fn delete(&self, id: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.client.delete(&format!("customers/{}", id)).await
    }

    pub async fn count(&self, params: Option<Vec<(&str, &str)>>) -> Result<Count, Box<dyn Error>> {
        self.client.read("customers/count", params).await
    }
}