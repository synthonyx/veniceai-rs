use crate::api::traits::API;

/// Configuration is passed as associated types.
pub trait Config {
    type API: API;
}

#[cfg(test)]
mod tests {
    use crate::api::traits::ModelsAdapter;
    use crate::api::API;
    use synthonyx_kit::env_param;

    use super::*;

    #[test]
    fn config_works() {
        pub struct Client;
        env_param!(ApiKey, String, "TEST_API_KEY");
        impl crate::api::config::Config for Client {
            type ApiKey = ApiKey;
        }
        impl Config for Client {
            type API = API<Self>;
        }

        assert!(
            <<Client as Config>::API as ModelsAdapter>::has_model("llama-3.1-405b")
                .expect("Expected to be able to get models from API")
        );
    }
}
