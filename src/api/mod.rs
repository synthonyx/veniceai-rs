use reqwest::blocking::Client as HttpClient;
use serde::Serialize;
use std::{error::Error, marker::PhantomData};
use synthonyx_kit::traits::Get;

pub mod chat;
pub mod config;
pub mod models;
pub mod traits;

pub use config::Config;

const BASE_URL: &str = "https://api.venice.ai/api/v1";

pub struct API<T: config::Config>(PhantomData<T>);

impl<T: config::Config> API<T> {
    fn get_url(to: &'static str) -> String {
        format!("{}/{}", BASE_URL, to)
    }

    fn get_http_client() -> reqwest::blocking::Client {
        HttpClient::new()
    }

    fn call(to: &'static str) -> Result<String, Box<dyn Error>> {
        let http_client = Self::get_http_client();

        let res = http_client
            .get(Self::get_url(to))
            .header("Authorization", format!("Bearer {}", T::ApiKey::get()))
            .send()?;

        // Ensure the request was successful
        if !res.status().is_success() {
            panic!("Request failed with status: {}", res.status());
        }

        // Return the body
        Ok(res.text()?)
    }

    fn call_with_body<S: Serialize>(to: &'static str, body: S) -> Result<String, Box<dyn Error>> {
        let http_client = Self::get_http_client();
        let res = http_client
            .post(Self::get_url(to))
            .header("Authorization", format!("Bearer {}", T::ApiKey::get()))
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

impl<T: config::Config> traits::API for API<T> {}

#[cfg(test)]
mod tests {
    use synthonyx_kit::{env_param, param};

    use super::*;

    struct Settings;

    param!(ApiKey, String, "");
    impl super::config::Config for Settings {
        type ApiKey = ApiKey;
    }

    #[test]
    fn url_composition_works() {
        assert_eq!(
            API::<Settings>::get_url("models"),
            "https://api.venice.ai/api/v1/models".to_string()
        );
        assert_eq!(
            API::<Settings>::get_url("chat/completions"),
            "https://api.venice.ai/api/v1/chat/completions".to_string()
        );
    }

    #[test]
    fn client_works() {
        struct Settings;
        env_param!(ApiKey, String, "TEST_API_KEY");
        impl super::config::Config for Settings {
            type ApiKey = ApiKey;
        }

        assert!(API::<Settings>::call("models").is_ok());
    }
}
