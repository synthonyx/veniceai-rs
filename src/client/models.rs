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
