#![allow(unused_imports)]

use axum_extra::extract::CookieJar;
use http::header::AUTHORIZATION;


use crate::prelude::statics::WHERETO;
use crate::prelude::{
        auth_fc::cookie::extract_token_from_cookies,
        pages::posts::posts_types::{
            PostData,UpdatePostReq
    }
};

use leptos::logging::log;
use leptos::context::use_context;
use leptos::{
    server, prelude::ServerFnError::{
        self,ServerError
    },
};
use serde::Deserialize;


use reqwest::Url;
// This is the payload your API returns
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<PostData>,
}


#[server(GetPosts, "/post/get")]
pub async fn get_posts_from_api() -> Result<Vec<PostData>, ServerFnError> {
    // Build and sanitize the URL string
    let url_str = WHERETO.full_url(4000, "/posts");
    let url_str_trimmed = url_str.trim(); // kill accidental spaces/newlines
    log!("Posts URL (raw)   = {:?}", url_str);
    log!("Posts URL (trimmed)= {:?}", url_str_trimmed);

    // Catch invalid URLs up-front
    let url = match Url::parse(url_str_trimmed) {
        Ok(u) => u,
        Err(e) => {
            log!("Bad URL for posts: {}", e);
            return Ok(vec![PostData {
                post_id: 0,
                title: "ERROR!".to_string(),
                markdown: format!("Bad URL: {e}"),
                html: None,
                tags: vec!["error".to_string()],
            }]);
        }
    };

    log!("Sending request to {}", url);

    let resp = match reqwest::get(url).await {
        Ok(r) => r,
        Err(err) => {
            // This is where you'd otherwise see the vague "builder error"
            log!("Failed to fetch posts (network/build): {:#}", err);
            return Ok(vec![PostData {
                post_id: 0,
                title: "ERROR!".to_string(),
                markdown: "Something went wrong".to_string(),
                html: None,
                tags: vec!["error".to_string()],
            }]);
        }
    };

    if !resp.status().is_success() {
        let code = resp.status();
        log!("Non-200 response: {}", code);
        return Ok(vec![PostData {
            post_id: 0,
            title: format!("HTTP {}", code),
            markdown: "Request failed".to_string(),
            html: None,
            tags: vec!["error".to_string()],
        }]);
    }

    let api = match resp.json::<ApiResponse>().await {
        Ok(a) => a,
        Err(err) => {
            log!("Failed to parse JSON: {}", err);
            return Ok(vec![PostData {
                post_id: 0,
                title: "Bad JSON".to_string(),
                markdown: "Could not parse API response".to_string(),
                html: None,
                tags: vec!["error".to_string()],
            }]);
        }
    };

    Ok(api.data)
}

type MyServerError = leptos::server_fn::ServerFnError<String>;



#[server(UpdatePost, "/post/update")]
pub async fn update_post_api(
    id: i32,
    post: UpdatePostReq,
    jwt: String,   // ✅ pass extracted JWT directly
) -> Result<(), leptos::server_fn::ServerFnError<String>> {
    let url_str = WHERETO.full_url(4000, &format!("/posts/{id}"));
    let url = Url::parse(url_str.trim())
        .map_err(|e| ServerError(format!("Bad URL: {e}")))?;

    let client = reqwest::Client::new();
    let mut req = client.patch(url).json(&post);

    if !jwt.is_empty() {
        leptos::logging::log!("➡️ Attaching Authorization header: Bearer {}", jwt);
        req = req.header(AUTHORIZATION, format!("Bearer {jwt}"));
    }

    let resp = req.send().await.map_err(|err| {
        ServerError(format!("Failed request: {err}"))
    })?;

    if !resp.status().is_success() {
        let code = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(ServerError(format!(
            "Update failed: HTTP {code} body={body}"
        )));
    }

    Ok(())
}


#[server(DeletePost, "/post/delete")]
pub async fn delete_post_api(id: i32) -> Result<(), ServerFnError> {
    let url_str = WHERETO.full_url(3033, &format!("/posts/{id}"));
    let url_str_trimmed = url_str.trim();
    log!("Delete URL (trimmed) = {:?}", url_str_trimmed);

    // Catch invalid URL
    let url = match Url::parse(url_str_trimmed) {
        Ok(u) => u,
        Err(e) => {
            log!("Bad URL for delete: {}", e);
            return Err(ServerFnError::ServerError(format!("Bad URL: {e}")));
        }
    };

    // Send DELETE
    let client = reqwest::Client::new();
    let resp = match client.delete(url).send().await {
        Ok(r) => r,
        Err(err) => {
            log!("Failed to send delete request: {:#}", err);
            return Err(ServerFnError::ServerError(format!("Failed request: {err}")));
        }
    };

    if !resp.status().is_success() {
        let code = resp.status();
        let body = resp.text().await.unwrap_or_default();
        log!("Delete failed: HTTP {} body={}", code, body);
        return Err(ServerFnError::ServerError(format!("Delete failed: HTTP {code} body={body}")));
    }

    Ok(())
}
