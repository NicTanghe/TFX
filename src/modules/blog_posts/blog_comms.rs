use reqwest::Error;
use crate::modules::blog_posts::blog_compo::Post;
use leptos::logging;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<Post>,
}


pub async fn get_posts_from_api() -> Result<Vec<Post>, Error> {
    let url = "http://localhost:3030/posts";
    logging::log!("Sending request to {}", url);

    // Send request
    let response = match reqwest::get(url).await {
        Ok(resp) => resp,
        Err(err) => {
            logging::log!("Failed to fetch posts: {}", err);
            // Return default posts on error
            return Ok(vec![
                Post {
                    post_id: 505,
                    title: "ERROR!".to_string(),
                    markdown: "Something went wrong".to_string(),
                    tags:[{"error".to_string()}].to_vec()
                }
            ]);
        }
    };

    // Check if the response status is OK
    if !response.status().is_success() {
        logging::log!("Request failed with status: {}", response.status());
        // Return default posts on HTTP error
        return Ok(vec![
            Post {
                post_id: response.status().as_u16() as i32,
                title: "HTTP Error".to_string(),
                markdown: "Request failed".to_string(),
                tags:[{"error".to_string()}].to_vec(),
            }
        ]);
    }

    // Deserialize the response body into ApiResponse
    let api_response: ApiResponse = match response.json().await {
        Ok(data) => data,
        Err(err) => {
            logging::log!("Failed to deserialize response: {}", err);
            // Return default posts on deserialization error
            return Ok(vec![
                Post {
                    post_id:0,
                    title: "Deserialization Error".to_string(),
                    markdown:"#this shall be serialised propperly".to_string(),
                    tags:[{"error".to_string()}].to_vec(),
                }
            ]);
        }
    };

    // Log success status
    logging::log!("Successfully received response from API.");
    //logging::log!("COMMS: \n Loaded data from API: {:?}", api_response.data);

    // Return the posts from the data field
    Ok(api_response.data)
}


// DELETE request to delete a post by ID
pub async fn delete_post_from_api(post_id: i32) -> Result<(), Error> {
    let url = format!("http://localhost:3030/posts/{}", post_id);
    logging::log!("Sending DELETE request to {}", url);

    let response = match reqwest::Client::new().delete(&url).send().await {
        Ok(resp) => resp,
        Err(err) => {
            logging::log!("Failed to delete post {}: {}", post_id, err);
            return Err(err);
        }
    };

    if !response.status().is_success() {
        logging::log!("DELETE request failed with status: {}", response.status());
    } else {
        logging::log!("Post with ID {} deleted successfully.", post_id);
    }

    Ok(())
}