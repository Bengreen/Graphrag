use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub database_url: String,
    pub server_port: u16,
}

impl AppConfig {
    #[allow(clippy::result_large_err)]
    pub fn load(config_path: Option<&str>) -> Result<Self, figment::Error> {
        let mut figment = Figment::new();

        if let Some(path) = config_path {
            figment = figment.merge(Yaml::file(path));
        }

        figment = figment.merge(Env::prefixed("GRAPHRAG_BE__").split("__"));

        figment.extract()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.database_url.trim().is_empty() {
            return Err("database_url must not be empty".to_string());
        }
        if self.server_port == 0 {
            return Err("server_port must be a valid port number".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_valid_config_from_yaml() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "database_url: postgres://yaml\nserver_port: 8081").unwrap();

        // Clear env vars that might affect the test since tests run in parallel
        unsafe {
            env::remove_var("GRAPHRAG_BE__DATABASE_URL");
            env::remove_var("GRAPHRAG_BE__SERVER_PORT");
        }

        let config = AppConfig::load(Some(file.path().to_str().unwrap())).unwrap();
        assert_eq!(config.database_url, "postgres://yaml");
        assert_eq!(config.server_port, 8081);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_override_from_env() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "database_url: postgres://yaml\nserver_port: 8081").unwrap();

        unsafe {
            env::set_var("GRAPHRAG_BE__DATABASE_URL", "postgres://env");
            env::set_var("GRAPHRAG_BE__SERVER_PORT", "9090");
        }

        let config = AppConfig::load(Some(file.path().to_str().unwrap())).unwrap();

        unsafe {
            env::remove_var("GRAPHRAG_BE__DATABASE_URL");
            env::remove_var("GRAPHRAG_BE__SERVER_PORT");
        }

        assert_eq!(config.database_url, "postgres://env");
        assert_eq!(config.server_port, 9090);
    }

    #[test]
    fn test_validate_empty_database_url() {
        let config = AppConfig {
            database_url: "   ".to_string(),
            server_port: 8080,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_zero_server_port() {
        let config = AppConfig {
            database_url: "postgres://test".to_string(),
            server_port: 0,
        };
        assert!(config.validate().is_err());
    }
}
