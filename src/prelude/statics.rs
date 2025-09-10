
use std::env;
use once_cell::sync::Lazy;

use leptos::{
    server,
    server_fn::ServerFnError,
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
        // Base, trimmed and without trailing slash
        let mut base = self.base_url.trim().trim_end_matches('/').to_string();

        // NEW: ensure we have a scheme (default to http://)
        if !base.starts_with("http://") && !base.starts_with("https://") {
            base = format!("http://{}", base);
        }

        // Keep your existing "has port?" check
        let has_port = base
            .rsplit_once(':')
            .map(|(_, suffix)| suffix.parse::<u16>().is_ok())
            .unwrap_or(false);

        let base_with_port = if has_port {
            println!("Base URL already contains a port: {}", base);
            base
        } else {
            println!("Appending port {} to base URL {}", port, base);
            format!("{}:{}", base, port)
        };

        // Clean path and join
        let p = path.trim();
        if p.is_empty() {
            base_with_port
        } else if p.starts_with('/') {
            format!("{}{}", base_with_port, p)
        } else {
            format!("{}/{}", base_with_port, p)
        }
    }
}
    
// Lazily initialize the two different configurations
pub static WHERETO: Lazy<ServerConfig> = Lazy::new(|| ServerConfig::init("WHERETO_URL"));
pub static THELIGHT: Lazy<ServerConfig> = Lazy::new(|| ServerConfig::init("THELIGHT_URL"));
pub static CDN: Lazy<ServerConfig> = Lazy::new(|| ServerConfig::init("CDN_URL"));


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
