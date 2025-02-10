use super::{config::Config, traits::ModelsAdapter, API};
use serde::Deserialize;
use std::error::Error;

// Supported models:
// Text
// - deepseek-r1-671b (Beta)
// - deepseek-r1-llama-70b (Beta)
// - dolphin-2.9.2-qwen2-72b (Most uncensored)
// - llama-3.1-405b (Most intelligent - Web search)
// - llama-3.2-3b (Fastest)
// - llama-3.3-70b (Default - Web search - Beta)

// Image
// - fluently-xl (Default)
// - flux-dev-uncensored
// - flux-dev (Highest quality)
// - lustify-sdxl
// - pony-realism
// - stable-diffusion-3.5 (Most artistic)

// Coding
// - qwen32b
// * deepseek-r1-671b (Beta)
// * deepseek-r1-llama-70b (Beta)
// * llama-3.1-405b (Most intelligent - Web search)
// * llama-3.3-70b (Default - Web search - Beta)

// Define the struct for the model specification
#[derive(Deserialize, Debug)]
pub struct ModelSpec {
    pub traits: Vec<String>,
    pub available_context_tokens: Option<u32>, // Optional field for availableContextTokens
}

// Define the struct for each model
#[derive(Deserialize, Debug)]
pub struct VeniceModel {
    pub id: String,
    pub r#type: String, // Use raw identifier for `type` because it's a reserved keyword
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub model_spec: ModelSpec,
}

// Define the struct for the response
#[derive(Deserialize, Debug)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<VeniceModel>,
}

impl<T: Config> API<T> {
    /// Returns a list of all models supported by the API.
    fn api_models() -> Result<ModelsResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&API::<T>::call("models")?)?)
    }

    /// Returns a lot of all model ids.
    fn model_ids() -> Result<Vec<String>, Box<dyn Error>> {
        let result = API::<T>::api_models()?;
        Ok(result.data.iter().map(|f| f.id.clone()).collect())
    }

    /// Returns true if the provided model id is supported by the API.
    fn has_model(model_id: &'_ str) -> Result<bool, Box<dyn Error>> {
        let models = API::<T>::model_ids()?;
        Ok(models.contains(&model_id.to_string()))
    }
}

impl<T: Config> ModelsAdapter for API<T> {
    fn api_models() -> Result<ModelsResponse, Box<dyn Error>> {
        API::<T>::api_models()
    }

    fn model_ids() -> Result<Vec<String>, Box<dyn Error>> {
        API::<T>::model_ids()
    }

    fn has_model(model_id: &'_ str) -> Result<bool, Box<dyn Error>> {
        API::<T>::has_model(model_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::config::Config;
    use synthonyx_kit::env_param;

    use super::*;

    #[test]
    fn api_models_works() {
        struct Settings;
        env_param!(ApiKey, String, "TEST_API_KEY");
        impl Config for Settings {
            type ApiKey = ApiKey;
        }

        assert!(API::<Settings>::has_model("llama-3.1-405b")
            .expect("Expected to be able to get models from API"));
    }

    // #[test]
    // fn models_works() {
    //     // Load environment variables from .env file
    //     dotenv().ok();

    //     // Get the API key from the environment variable
    //     let api_key = env::var("TEST_API_KEY").expect("TEST_API_KEY must be set");
    //     let client = API::new(api_key);

    //     assert!(client
    //         .has_model("llama-3.3-70b")
    //         .expect("Expected to be able to get models from API"));

    //     let models = client.models().expect("Expected to be able to get a list");

    //     assert!(models.len() >= 7);
    //     assert!(models.contains(&Model::Llama31_405B));
    // }

    // #[test]
    // fn has_model_with_model_input_works() {
    //     // Load environment variables from .env file
    //     dotenv().ok();

    //     // Get the API key from the environment variable
    //     let api_key = env::var("TEST_API_KEY").expect("TEST_API_KEY must be set");
    //     let client = API::new(api_key);

    //     assert!(client
    //         .has_model(&Model::Llama31_405B.to_string())
    //         .expect("Expected to be able to get models from API"));
    // }
}
