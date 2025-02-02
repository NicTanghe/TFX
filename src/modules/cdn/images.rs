use leptos::*;
use leptos::html::body;

use reqwest::{Client,Response,Body};
use std::sync::Arc;

//switch to using
use crate::modules::statics::CDN;
use tokio_util::io::StreamReader;
/// Server function to fetch a single image from the local LAN server.
/// Accepts the image path as input and returns the binary image data.
#[server(fetch_image_from_cdn_base64, "/apimem")]
pub async fn fetch_image_from_cdn_mem(
    image_path: String,
) -> Result<Vec<u8>, ServerFnError> {
    let client = reqwest::Client::new();
    let url = format!("http://192.168.2.13:5434{}", image_path);


    match client.get(&url).send().await {
        Ok(response) if response.status().is_success() => {
            match response.bytes().await {
                Ok(bytes) => Ok(bytes.to_vec()), // Return binary data
                Err(_) => Err(ServerFnError::ServerError(
                    "Failed to read image bytes".into(),
                )),
            }
        }
        Ok(_) => Err(ServerFnError::ServerError(
                "didn`t find jack shit".into(),
        )),
        Err(_) => Err(ServerFnError::ServerError(
            "Failed to fetch image from backend".into(),
        )),
    }
}


#[server(fetch_image_from_cdn_forward, "/api")]
pub async fn fetch_image_from_cdn(
    image_path: String,
) -> Result<(), ServerFnError> {
    // Construct the URL for the backend server
    let backend_url = format!("http://192.168.2.13:5434{}", image_path);

    // Send the request to the backend
    let client = Client::new();
    let response = client.get(&backend_url).send().await.map_err(|_| {
        ServerFnError::ServerError("Failed to fetch image from backend".into())
    })?;

    // Check for a successful response
    if !response.status().is_success() {
        return Err(ServerFnError::ServerError(
            "Image not found or backend error".into(),
        ));
    }

    // Forward the response headers and body to the client
    let status = response.status();
    let headers = response.headers().clone();
    let stream = StreamReader::new(response.bytes_stream());

    let mut forward_response = Response::builder()
            .status(status)
            .header("Content-Type", "image/gif") // Set the content type to image/gif
            .body(Body::wrap_stream(stream))
            .map_err(|err| ServerFnError::ServerError(format!("Failed to build response: {}", err)))?;
        // Copy headers from the original response (to preserve things like Content-Type)
    for (key, value) in headers.iter() {
        forward_response.headers_mut().insert(key, value.clone());
    }

    // Return the forward response to the client
    Ok(forward_response)
}
