use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::error::LightspeedApiError;

pub struct LightspeedClient {
    client: Client,
    api_key: String,
    api_secret: String,
    api_language: String,
    api_server: String
}

impl LightspeedClient {
    pub fn new(api_server: &str, api_key: &str, api_secret: &str, api_language: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            api_language: api_language.to_string(),
            api_server: api_server.to_string(),
        }
    }

    fn get_url(&self, resource_url: &str) -> String {
        let api_host = match self.api_server.as_str() {
            "live" => "https://api.webshopapp.com/",
            "local" => "https://api.webshopapp.dev/",
            "eu1" => "https://api.webshopapp.com/",
            "us1" => "https://api.shoplightspeed.com/",
            _ => panic!("Invalid API server"),
        };

        format!(
            "{}{}{}{}",
            api_host, self.api_language, "/", resource_url
        )
    }

    pub async fn create<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        payload: &T,
    ) -> Result<R, Box<dyn Error>> {
        let url = self.get_url(url);
        let response = self
            .client
            .post(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .json(payload)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn read<R: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        params: Option<Vec<(&str, &str)>>,
    ) -> Result<R, Box<dyn Error>> {
        let url = self.get_url(url);
        let mut request = self
            .client
            .get(url)
            .basic_auth(&self.api_key, Some(&self.api_secret));

        if let Some(params) = params {
            request = request.query(&params);
        }

        let response = request.send().await?;

        self.handle_response(response).await
    }

    pub async fn update<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        payload: &T,
    ) -> Result<R, Box<dyn Error>> {
        let url = self.get_url(url);
        let response = self
            .client
            .put(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .json(payload)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn delete<R: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<R, Box<dyn Error>> {
        let url = self.get_url(url);
        let response = self
            .client
            .delete(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<R: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> Result<R, Box<dyn Error>> {
        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            Ok(serde_json::from_str(&body)?)
        } else {
            Err(Box::new(LightspeedApiError {
                message: body,
                status_code: status.as_u16(),
            }))
        }
    }
}