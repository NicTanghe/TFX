use leptos::{
    server,
    logging,
    ServerFnError
};
use serde::{Serialize,Deserialize};

use serde_json::Value;

#[derive(PartialEq, Clone,Serialize, Deserialize,Debug)] // Ensure Post is Cloneable
pub struct Person {
  pub id: i32,  // Use Option<i32> if id can be null
  pub name: Option<String>,  // Use Option<String> if name can be null
  pub contact_info: Option<Value>,  // Use Option for nullable arrays or complex types
  pub profile_pic: Option<Vec<u8>>,  // This is already an Option
  pub description: Option<String>,  // This is already an Option
}


#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Vec<Person>,
}

// ok so this thing seems to just return an empt vector.
// so then why are there 8 unknowns wtf.


// this updates the current vector and returns the old value if there is an error.
pub async fn get_people_vector(value: Vec<Person>, jwt_l1:String) -> Vec<Person> {
    match get_people_from_api(jwt_l1).await {
        Ok(fetched_people) => {
            logging::log!("Fetched people:\n{:#?}", fetched_people);
            fetched_people
        }
        Err(err) => {
            logging::log!("Error fetching posts: {:?}", err);
            value // Returns old vector in case of error
        }
    }
}


#[server(GetPosts, "/people/get")]
pub async fn get_people_from_api(jwt_l2: String) -> Result<Vec<Person>, ServerFnError> {
    let url = "http://localhost:4000/people";
    logging::log!("Sending request to {}", url);

    // Initialize reqwest client
    let client = reqwest::Client::new();

    // Send request with Authorization header
    let response = match client
        .get(url)
        .header("Content-Type", "application/json")

        .header("Authorization", format!("Bearer {}", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0aW5nIiwiY29tcGFueSI6IkV4YW1wbGUgQ29ycCIsImV4cCI6MTczMjIyODQ5NSwicm9sZXMiOlsidXNlciJdfQ.-Q-r1JD8j3xXLnBAbfC6PNTs9O2PGPpJddGSwRUHztk"))


        .send()
        .await 
    {
        Ok(resp) => resp,
        Err(err) => {
            logging::log!("Failed to fetch posts: {}", err);
            return Ok(vec![
                Person {
                    id: 505,
                    name: Some("ERROR!".to_string()),
                    description: Some("Something went wrong".to_string()),
                    contact_info: None,
                    profile_pic: None
                }
            ]);
        }
    };

    // Check if the response status is OK
    if !response.status().is_success() {
        logging::log!("Request failed with status: {}", response.status());
        return Ok(vec![
            Person {
                id: response.status().as_u16() as i32,
                name: Some("HTTP Error".to_string()),
                description: Some("Request failed".to_string()),
                contact_info: None,
                profile_pic: None
            }
        ]);
    }

    // Deserialize the response body into ApiResponse
    let api_response: ApiResponse = match response.json().await {
        Ok(data) => data,
        Err(err) => {
            logging::log!("Failed to deserialize response: {}", err);
            return Ok(vec![
                Person {
                    id: 505,
                    name: Some("Deserialization Error".to_string()),
                    description: None,
                    contact_info: None,
                    profile_pic: None
                }
            ]);
        }
    };

    // Log success status
    logging::log!("Successfully received response from API.");
    logging::log!("COMMS: \n Loaded data from API: {:?}", api_response.data);

    Ok(api_response.data)
}

