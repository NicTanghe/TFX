use reqwest::Error;
use crate::modules::blog_posts::blog_compo::Post;
use leptos::{
    server,
    logging,
    ServerFnError
};
use serde::{Serialize,Deserialize};



use crate::modules::statics::WHERETO;

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct CreatePostReq {
 pub title: String,
 pub tags: Vec<String>,
 pub markdown: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<Post>,
}



#[server(GetPosts, "/post/get")]
pub async fn get_posts_from_api() -> Result<Vec<Post>, ServerFnError> {
    let url = WHERETO.full_url(3030,"/posts");
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



#[server(DelPost, "/post/del")]
pub async fn delete_post_from_api(post_id: i32) -> Result<(), ServerFnError> {
    
    let formatted_path = format!("/posts/{}", post_id);
    let url = WHERETO.full_url(3030, &formatted_path);

    logging::log!("Sending DELETE request to {}", url);
 
    let response = match reqwest::Client::new().delete(&url).send().await {
        Ok(resp) => resp,
        Err(err) => {
            logging::log!("Failed to delete post {}: {}", post_id, err);
            return Err(ServerFnError::Args(err.to_string())); // Wrap it in `Err`
        }
    };

    if !response.status().is_success() {
        logging::log!("DELETE request failed with status: {}", response.status());
    } else {
        logging::log!("Post with ID {} deleted successfully.", post_id);
    }

    Ok(())
}



// Function to create a new post via API
#[server(SetPost, "/post/set")]
pub async fn create_post_to_blog_api(new_post: CreatePostReq, jwt: String) -> Result<(), ServerFnError> {
    let url = WHERETO.full_url(4000,"/posts");
    logging::log!("Sending POST request to {}", url);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", jwt))
        .json(&new_post) // Automatically serialize the struct to JSON
        .send()
        .await;


    match response {
        Ok(resp) => {
            let status = resp.status(); // Store the status
            let error_body = resp.text().await.unwrap_or_else(|_| "No response body".to_string()); // Move the response here

            if !status.is_success() {
                logging::log!("POST request failed with status: {}", status);
                logging::log!("Response body: {}", error_body);

                return Err(ServerFnError::ServerError(format!(
                    "POST request failed with status: {} and body: {}",
                    status,
                    error_body
                )));
            } else {
                logging::log!("Successfully created post.");
            }
        }
        Err(err) => {
            logging::log!("Failed to send POST request: {}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    };

    Ok(())
}



