use reqwest::blocking::Client as HttpClient;
use serde::Serialize;
use std::error::Error;

const BASE_URL: &'static str = "https://api.venice.ai/api/v1";

pub mod chat;
pub mod models;

pub struct Client {
    http_client: HttpClient,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        let http_client = HttpClient::new();

        Self {
            http_client,
            api_key,
        }
    }

    fn get_url(to: &'static str) -> String {
        format!("{}/{}", BASE_URL, to)
    }

    fn call(&self, to: &'static str) -> Result<String, Box<dyn Error>> {
        let res = self
            .http_client
            .get(Client::get_url(to))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()?;

        // Ensure the request was successful
        if !res.status().is_success() {
            panic!("Request failed with status: {}", res.status());
        }

        // Return the body
        Ok(res.text()?)
    }

    fn call_with_body<T: Serialize>(
        &self,
        to: &'static str,
        body: T,
    ) -> Result<String, Box<dyn Error>> {
        let res = self
            .http_client
            .post(Client::get_url(to))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()?;

        // Ensure the request was successful
        if !res.status().is_success() {
            panic!("Request failed with status: {}", res.status());
        }

        // Return the body
        Ok(res.text()?)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use super::*;

    #[test]
    fn url_composition_works() {
        assert_eq!(
            Client::get_url("models"),
            "https://api.venice.ai/api/v1/models".to_string()
        );
        assert_eq!(
            Client::get_url("chat/completions"),
            "https://api.venice.ai/api/v1/chat/completions".to_string()
        );
    }

    #[test]
    fn client_works() {
        // Load environment variables from .env file
        dotenv().ok();

        // Get the API key from the environment variable
        let api_key = env::var("TEST_API_KEY").expect("TEST_API_KEY must be set");

        let client = Client::new(api_key);
        assert!(client.call("models").is_ok());
    }
}
