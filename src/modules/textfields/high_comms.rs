
use reqwest::Error;
use serde::Serialize;

// Struct for the request payload
#[derive(Serialize)]
struct CodeRequest {
    code: String,
    language_extension: String,
}

// Function to send code for highlighting
pub async fn send_code_for_highlighting(code: &str, language_extension: &str) -> Result<String, Error> {
    let url = "http://127.0.0.1:3037/highlight";

    // Create a CodeRequest instance
    let payload = CodeRequest {
        code: code.to_string(),
        language_extension: language_extension.to_string(),
    };

    // Send the request using reqwest
    let response = reqwest::Client::new()
        .post(url)
        .json(&payload) // This will serialize the payload to JSON
        .send()
        .await?;

    // Check if the response was successful
    if response.status().is_success() {
        response.text().await
    } else {
        Ok("Error: Failed to get a successful response".to_string())
    }
}

