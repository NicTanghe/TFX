use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
// If you still hit serde identity issues, uncomment the next line:
// #[serde(crate = "leptos::serde")]
pub struct PostData {
    #[serde(rename = "id")]
    pub post_id: i32,
    pub title: String,
    pub markdown: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub html: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(crate = "leptos::serde")]
pub struct CreatePostReq {
    pub title: String,
    pub tags: Vec<String>,
    pub markdown: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostReq {
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub markdown: Option<String>,
    pub html: Option<String>,
}
