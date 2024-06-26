use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    pub count: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountWrapper {
    pub account: Account
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub company: String,
    pub cluster: String,
    pub language: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    pub products: Option<Vec<Product>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductWrapper {
    pub product: Product
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub price: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orders {
    pub orders: Option<Vec<Order>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderWrapper {
    pub order: Order
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: i64,
    pub number: String,
    pub status: String,
    pub total: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customers {
    pub customers: Option<Vec<Customer>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerWrapper {
    pub customer: Customer
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}