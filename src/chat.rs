use super::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    // TODO: tools
    // TODO: venice_parameters
}

// TODO: Builder pattern.

impl ChatRequest {
    pub fn new(model: String, messages: Vec<Message>) -> ChatRequest {
        ChatRequest {
            model,
            messages,
            frequency_penalty: None,
            max_completion_tokens: None,
            presence_penalty: None,
            prompt: None,
            stop: None,
            stream: false,
            temperature: None,
            top_p: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

impl Client {
    pub fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn Error>> {
        let res = serde_json::from_str(&self.call_with_body("chat/completions", req)?)?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use super::*;

    #[test]
    fn chat_works() {
        // Load environment variables from .env file
        dotenv().ok();

        // Get the API key from the environment variable
        let api_key = env::var("TEST_API_KEY").expect("TEST_API_KEY must be set");
        let client = Client::new(api_key);

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "What is the capital of Cyprus?".to_string(),
            },
        ];

        let request = ChatRequest::new("llama-3.1-405b".to_string(), messages);
        let result = client.chat(request).unwrap();
        assert_eq!(result.choices.len(), 1);
        assert!(result
            .choices
            .first()
            .unwrap()
            .message
            .content
            .contains("Nicosia"));
    }
}
