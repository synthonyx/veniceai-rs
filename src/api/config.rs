use synthonyx_kit::traits::Get;

pub trait Config {
    type ApiKey: Get<String>;
}
