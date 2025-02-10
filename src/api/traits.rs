use super::{
    chat::{ChatRequest, ChatResponse},
    models::ModelsResponse,
};
use std::error::Error;

pub trait ModelsAdapter {
    fn api_models() -> Result<ModelsResponse, Box<dyn Error>>;
    fn model_ids() -> Result<Vec<String>, Box<dyn Error>>;
    fn has_model(model_id: &'_ str) -> Result<bool, Box<dyn Error>>;
}

pub trait ChatAdapter {
    fn chat(req: ChatRequest) -> Result<ChatResponse, Box<dyn Error>>;
}

pub trait API: ModelsAdapter + ChatAdapter {}
