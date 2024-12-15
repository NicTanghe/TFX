
use leptos::*;
use leptos::prelude::*;
use leptos::prelude::server_fn::ServerFnError;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthResponse {
    pub token: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub client_id: String,
    pub client_secret: String,
}

// Function to map reqwest::StatusCode to ServerFnError
fn map_status_to_error(status: reqwest::StatusCode) -> ServerFnError {
    match status {
        reqwest::StatusCode::UNAUTHORIZED => ServerFnError::ServerError("Unauthorized".into()),
        reqwest::StatusCode::BAD_REQUEST => ServerFnError::ServerError("Bad Request".into()),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => ServerFnError::ServerError("Internal Server Error".into()),
        _ => ServerFnError::ServerError("Unknown error".into()),
    }
}


#[server(Authoreise, "/authorize")]
pub async fn get_access_token(client_id: String, client_secret: String) -> Result<String, ServerFnError> {
    let url = "http://localhost:4000/authorize";
    //let client_id = "foo";
    //let client_secret = "bar";
    
    logging::log!("Sending POST request to {}", url);

    // Prepare the body of the request
    let request_body = serde_json::json!({
        "client_id": client_id,
        "client_secret": client_secret,
    });

    // Send POST request with the specified headers
    let client = reqwest::Client::new();
    let response = match client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(err) => {
            logging::log!("Failed to send request: {}", err);
            // Return error token
            return Ok(serde_json::json!({
                "access_token": "",
                "error": "Failed to send request"
            }).to_string());
        }
    };

    // Check if the response status is OK
    if !response.status().is_success() {
        logging::log!("Request failed with status: {}", response.status());
        // Return error token on HTTP error
        return Ok(serde_json::json!({
            "access_token": "",
            "error": format!("HTTP Error: {}", response.status())
        }).to_string());
    }

    // Try to parse the JSON response
    let json_response: serde_json::Value = match response.json().await {
        Ok(data) => data,
        Err(err) => {
            logging::log!("Failed to deserialize response: {}", err);
            // Return error token on deserialization error
            return Ok(serde_json::json!({
                "access_token": "",
                "error": "Deserialization Error"
            }).to_string());
        }
    };

        let access_token = json_response["access_token"].as_str().expect("access_token not found").to_string();
     //Return the access token from the response
    Ok(access_token.to_string())
}
