use super::Client;
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
pub struct Model {
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
    pub data: Vec<Model>,
}

impl Client {
    pub fn models(&self) -> Result<ModelsResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.call("models")?)?)
    }

    pub fn model_ids(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let result = self.models()?;
        Ok(result.data.iter().map(|f| f.id.clone()).collect())
    }

    pub fn has_model_id(&self, model: &'static str) -> Result<bool, Box<dyn Error>> {
        let models = self.model_ids()?;
        Ok(models.contains(&model.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use super::*;

    #[test]
    fn models_works() {
        // Load environment variables from .env file
        dotenv().ok();

        // Get the API key from the environment variable
        let api_key = env::var("TEST_API_KEY").expect("TEST_API_KEY must be set");
        let client = Client::new(api_key);

        assert!(client
            .has_model_id("llama-3.3-70b")
            .expect("Expected to be able to get models from API"));
    }
}
