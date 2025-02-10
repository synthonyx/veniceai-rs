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

/// An enum representing all the models supported by Venice AI.
#[derive(Debug, PartialEq, Clone)]
pub enum Model {
    DeepSeekR1_671B,
    DeepSeekR1Llama70B,
    Dolphin292Qwen2_72B,
    Llama31_405B,
    Llama32_3b,
    Llama33_70B,
    Qwen32B,
    FluentlyXL,
    FluxDevUncensored,
    FluxDev,
    LustifySDXL,
    PonyRealism,
    StableDiffusion35,
}

impl<'a> From<&'a str> for Model {
    fn from(value: &'a str) -> Self {
        match value {
            "deepseek-r1-671b" => Model::DeepSeekR1_671B,
            "deepseek-r1-llama-70b" => Model::DeepSeekR1Llama70B,
            "dolphin-2.9.2-qwen2-72b" => Model::Dolphin292Qwen2_72B,
            "llama-3.1-405b" => Model::Llama31_405B,
            "llama-3.2-3b" => Model::Llama32_3b,
            "llama-3.3-70b" => Model::Llama33_70B,
            "qwen32b" => Model::Qwen32B,
            "fluently-xl" => Model::FluentlyXL,
            "flux-dev-uncensored" => Model::FluxDevUncensored,
            "flux-dev" => Model::FluxDev,
            "lustify-sdxl" => Model::LustifySDXL,
            "pony-realism" => Model::PonyRealism,
            "stable-diffusion-3.5" => Model::StableDiffusion35,
            model => panic!("Unknown model: {}", model),
        }
    }
}

impl From<String> for Model {
    fn from(value: String) -> Self {
        Model::from(value.as_str())
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Model::DeepSeekR1_671B => write!(f, "deepseek-r1-671b"),
            Model::DeepSeekR1Llama70B => write!(f, "deepseek-r1-llama-70b"),
            Model::Dolphin292Qwen2_72B => write!(f, "dolphin-2.9.2-qwen2-72b"),
            Model::Llama31_405B => write!(f, "llama-3.1-405b"),
            Model::Llama32_3b => write!(f, "llama-3.2-3b"),
            Model::Llama33_70B => write!(f, "llama-3.3-70b"),
            Model::Qwen32B => write!(f, "qwen32b"),
            Model::FluentlyXL => write!(f, "fluently-xl"),
            Model::FluxDevUncensored => write!(f, "flux-dev-uncensored"),
            Model::FluxDev => write!(f, "flux-dev"),
            Model::LustifySDXL => write!(f, "lustify-sdxl"),
            Model::PonyRealism => write!(f, "pony-realism"),
            Model::StableDiffusion35 => write!(f, "stable-diffusion-3.5"),
        }
    }
}

impl Model {
    /// Returns true if the particular model is suitable for text queries.
    pub fn for_text(&self) -> bool {
        matches!(
            self,
            Model::DeepSeekR1_671B
                | Model::DeepSeekR1Llama70B
                | Model::Dolphin292Qwen2_72B
                | Model::Llama31_405B
                | Model::Llama32_3b
                | Model::Llama33_70B
                | Model::Qwen32B
        )
    }

    /// Returns true if the particular model is suitable for coding.
    pub fn for_coding(&self) -> bool {
        matches!(self, Model::Llama31_405B | Model::Qwen32B)
    }

    /// Returns true if the particular model is suitable for image generation.
    ///
    /// Will always return false for Other.
    pub fn for_images(&self) -> bool {
        matches!(
            self,
            Model::FluentlyXL
                | Model::FluxDevUncensored
                | Model::FluxDev
                | Model::LustifySDXL
                | Model::PonyRealism
                | Model::StableDiffusion35
        )
    }
}

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

impl Client {
    /// Returns a list of all models supported by the API.
    pub fn api_models(&self) -> Result<ModelsResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.call("models")?)?)
    }

    /// Returns a lot of all model ids.
    pub fn model_ids(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let result = self.api_models()?;
        Ok(result.data.iter().map(|f| f.id.clone()).collect())
    }

    /// Returns true if the provided model id is supported by the API.
    pub fn has_model(&self, model_id: &'_ str) -> Result<bool, Box<dyn Error>> {
        let models = self.model_ids()?;
        Ok(models.contains(&model_id.to_string()))
    }

    /// Returns a list of all available models.
    pub fn models(&self) -> Result<Vec<Model>, Box<dyn Error>> {
        let list: Vec<Model> = self
            .api_models()?
            .data
            .iter()
            .map(|f| Model::from(f.id.clone()))
            .collect();

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use super::*;

    #[test]
    fn api_models_works() {
        // Load environment variables from .env file
        dotenv().ok();

        // Get the API key from the environment variable
        let api_key = env::var("TEST_API_KEY").expect("TEST_API_KEY must be set");
        let client = Client::new(api_key);

        assert!(client
            .has_model("llama-3.1-405b")
            .expect("Expected to be able to get models from API"));
    }

    #[test]
    fn models_works() {
        // Load environment variables from .env file
        dotenv().ok();

        // Get the API key from the environment variable
        let api_key = env::var("TEST_API_KEY").expect("TEST_API_KEY must be set");
        let client = Client::new(api_key);

        assert!(client
            .has_model("llama-3.3-70b")
            .expect("Expected to be able to get models from API"));

        let models = client.models().expect("Expected to be able to get a list");

        assert!(models.len() >= 7);
        assert!(models.contains(&Model::Llama31_405B));
    }

    #[test]
    fn has_model_with_model_input_works() {
        // Load environment variables from .env file
        dotenv().ok();

        // Get the API key from the environment variable
        let api_key = env::var("TEST_API_KEY").expect("TEST_API_KEY must be set");
        let client = Client::new(api_key);

        assert!(client
            .has_model(&Model::Llama31_405B.to_string())
            .expect("Expected to be able to get models from API"));
    }
}
