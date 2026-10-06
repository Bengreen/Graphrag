use crate::config::LlmConfig;
use rig_core::providers::openai::Client;

#[derive(Clone, Debug)]
pub struct LlmClient {
    pub client: Client,
    pub model_name: String,
}

impl LlmClient {
    pub fn new(config: &LlmConfig) -> Result<Self, String> {
        let api_key = config.api_key.as_deref().unwrap_or("sk-no-key-required");
        let mut client_builder = rig_core::providers::openai::Client::builder().api_key(api_key);

        let url = config.base_url.trim();
        if !url.is_empty() {
            client_builder = client_builder.base_url(url);
        }

        let rig_client = client_builder.build().map_err(|e| e.to_string())?;

        Ok(Self {
            client: rig_client,
            model_name: config.model.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_client_initialization() {
        let config = LlmConfig {
            provider: "ollama".to_string(),
            base_url: "http://localhost:11434/v1".to_string(),
            api_key: None,
            model: "llama3".to_string(),
            timeout_seconds: 60,
        };

        let client = LlmClient::new(&config);
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.model_name, "llama3");
    }
}
