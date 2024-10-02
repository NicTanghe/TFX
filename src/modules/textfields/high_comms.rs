
use reqwest::Error;
use serde::Serialize;

use crate::modules::textfields::skrijf::Cblock;

use std::collections::HashMap;
// Struct for the request payload
#[derive(Serialize)]
struct CodeRequest {
    code: String,
    language_extension: String,
}
// Enum to represent the status of the language mapping
enum LanguageStatus {
    Existing(String),
    NotExisting(String),
}

// Function to map languages to their proper syntect language identifier
fn map_language(language: &str) -> LanguageStatus {
    let mut lang_map = HashMap::new();
    
    // Add mappings
    lang_map.insert("rust", "rs");
    lang_map.insert("Rust", "rs");
    lang_map.insert("rs", "rs");

    lang_map.insert("python", "py");
    lang_map.insert("Python", "py");
    lang_map.insert("py", "py");

    lang_map.insert("javascript", "js");
    lang_map.insert("JavaScript", "js");
    lang_map.insert("js", "js");

    lang_map.insert("typescript", "ts");
    lang_map.insert("TypeScript", "ts");
    lang_map.insert("ts", "ts");

    lang_map.insert("c++", "cpp");
    lang_map.insert("C++", "cpp");
    lang_map.insert("cpp", "cpp");
    
    lang_map.insert("Go", "go");
    lang_map.insert("go", "go");
    
    lang_map.insert("Java", "java");
    lang_map.insert("java", "java");
    
    lang_map.insert("html", "html");
    // Return mapped value or indicate if not found
    match lang_map.get(language) {
        Some(mapped) => LanguageStatus::Existing(mapped.to_string()),
        None => LanguageStatus::NotExisting(language.to_string()),
    }
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

// Function to process the Cblock and highlight the code if necessary
pub async fn process_cblock(mut cblock: Cblock) -> Cblock {
    match map_language(&cblock.lang) {
        LanguageStatus::Existing(mapped_lang) => {
            // Update the language in the Cblock
            cblock.lang = mapped_lang.clone();
            // Send the code for highlighting and get the HTML result
            match send_code_for_highlighting(&cblock.code, &cblock.lang).await {
                Ok(highlighted_code) => {
                    cblock.code = highlighted_code; // Assign the highlighted HTML code
                }
                Err(e) => {
                    println!("Error highlighting Cblock ID {}: {:?}", cblock.id, e);
                    // Optionally, handle the error by keeping the original code
                }
            }
            cblock
        }
        LanguageStatus::NotExisting(_) => {
            // If the language does not exist, do nothing for highlighting
            println!("Language not found, skipping highlight for Cblock ID: {}", cblock.id);
            // Return the original code or handle as needed
            cblock // Return original code if language is not found
        }
    }
}
