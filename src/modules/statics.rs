use std::env;
use once_cell::sync::Lazy;

use leptos::{
    server,
    ServerFnError
};

#[derive(Debug)]
pub struct ServerConfig {
    base_url: String,
}

impl ServerConfig {
    // Initialize ServerConfig from an environment variable
    pub fn init(env_var: &str) -> Self {
        let base_url = env::var(env_var).unwrap_or_else(|_| "127.0.0.1".to_owned());
        ServerConfig { base_url }
    }

    // Get the base URL
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the full URL with a port and optional path
    pub fn full_url(&self, port: u16, path: &str) -> String {
        let base_with_port = if self.base_url.rfind(':').map_or(false, |i| {
        // Check if `:` is followed by digits (indicating a port)
            self.base_url[i + 1..].chars().all(|c| c.is_digit(10))
        }) {
            println!("Base URL already contains a port: {}", self.base_url);
            self.base_url.clone() // Use as is if it already contains a port
        } else {
            println!("Appending port {} to base URL {}", port, self.base_url);
            format!("{}:{}", self.base_url, port) // Append the provided port
        };

        if path.starts_with('/') {
            format!("{}{}", base_with_port, path)  // Directly append path
        } else {
            format!("{}/{}", base_with_port, path)  // Add "/" before appending path
        }
    }
}
    
// Lazily initialize the two different configurations
pub static WHERETO: Lazy<ServerConfig> = Lazy::new(|| ServerConfig::init("WHERETO_URL"));
pub static THELIGHT: Lazy<ServerConfig> = Lazy::new(|| ServerConfig::init("THELIGHT_URL"));
pub static CDN: Lazy<ServerConfig> = Lazy::new(|| ServerConfig::init("CDN_URL"));


// weird and roundabout way but it makes sence when you think about it to use serverfunctions
// weird and roundabout way but it makes sence when you think about it to use serverfunctions


#[server(GetWhereto, "/statics/get")]
pub async fn get_whereto(port: u16, path: String) -> Result<String, ServerFnError> {
    Ok(WHERETO.full_url(port, &path))
}

#[server(GetTheLight, "/statics/get")]
pub async fn get_thelight(port: u16, path: String) -> Result<String, ServerFnError> {
    Ok(THELIGHT.full_url(port, &path))
}

#[server(GetCDN, "/statics/get")]
pub async fn get_cdn(port: u16, path: String) -> Result<String, ServerFnError> {
    Ok(CDN.full_url(port, &path))
}

