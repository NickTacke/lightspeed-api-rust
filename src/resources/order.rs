use crate::{LightspeedClient, Order, OrderWrapper, Orders, Count};
use std::error::Error;

pub struct OrderResource<'a> {
    client: &'a LightspeedClient,
}

impl<'a> OrderResource<'a> {
    pub fn new(client: &'a LightspeedClient) -> Self {
        Self { client }
    }

    pub async fn get_all(&self, params: Option<Vec<(&str, &str)>>) -> Result<Orders, Box<dyn Error>> {
        self.client.read("orders", params).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Order, Box<dyn Error>> {
        let order: Option<OrderWrapper> = self.client.read(&format!("orders/{}", id), None).await?;
        
        Ok(order.and_then(| data | Some(data.order)).ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No order found with given id!"
        ))?)
    }

    pub async fn get_by_number(&self, number: &str) -> Result<Order, Box<dyn Error>> {
        let orders: Option<Vec<Order>> = self.client.read("orders", Some(vec![("number", number)])).await?;
    
        Ok(orders
            .and_then(|vec| vec.into_iter().next())
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No order found with given number!"
            ))?)
    }

    pub async fn update(&self, id: &str, order: Order) -> Result<Order, Box<dyn Error>> {
        let wrapper = OrderWrapper{order};
        let response: OrderWrapper = self.client.update(&format!("orders/{}", id), &wrapper).await?;
        Ok(response.order)
    }

    pub async fn count(&self, params: Option<Vec<(&str, &str)>>) -> Result<Count, Box<dyn Error>> {
        self.client.read("orders/count", params).await
    }
}