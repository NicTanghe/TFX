use reqwest::Error;
use serde::Serialize;


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

    lang_map.insert("scss", "scss");
    lang_map.insert("css", "css");
    
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


//----------------------------------------
#[derive(Serialize, Deserialize, Clone)]
pub struct Cblock {
   pub id: i8,
   pub code: String,
   pub lang: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Omark {
    amount: i8,
    ohtml: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct AllStat {
    orig: Omark,
    code: Vec<Cblock>,
}

//splits yo string
pub fn split_tags(tags: &str) -> Vec<String> {
    // Trim leading and trailing whitespace from the entire input string
    tags.trim()
        .split(',')
        .map(|tag| tag.trim().to_string()) // Trim whitespace from each tag and convert to String
        .filter(|tag| !tag.is_empty()) // Remove empty tags if any
        .collect()
}

// Function to highlight code blocks asynchronously
async fn highlight_code_blocks(code_blocks: Vec<Cblock>) -> Vec<Cblock> {
    let mut highlighted_blocks = Vec::new();

    for block in code_blocks {

        // Await the highlighting of the code block
        let processed_block = process_cblock(block).await;

        // Create a new Cblock with the highlighted code
        highlighted_blocks.push(
            processed_block
        );
    }

    highlighted_blocks
}


async fn assemble_highlighted_content(allstat: AllStat) -> String {
    let mut final_html = allstat.orig.ohtml.clone();

    // Iterate over the code blocks and insert them in the correct position
    for (i, block) in allstat.code.iter().enumerate() {
        // Create the placeholder string (e.g., [Code Block 1])
        let code_block_marker = format!("[Code Block {}]", i + 1);

        // Replace the placeholder with the highlighted code
        final_html = final_html.replace(&code_block_marker, &block.code);
    }

    final_html
}

fn escape_html(input: &str) -> String {
    input.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&apos;")
}


// Function to extract code blocks from HTML-formatted markdown and return both Cblock and Omark
fn extract_code_blocks_from_html(html: &str) -> (Vec<Cblock>, Omark) {
    let mut code_blocks: Vec<Cblock> = Vec::new();
    let fragment = Html::parse_fragment(html);

    // Selector to find the <code> elements
    let code_selector = Selector::parse("code").unwrap();

    // Regex to capture the language from the class attribute (e.g., "language-rust")
    let lang_regex = Regex::new(r"language-(\w+)").unwrap();

    let mut id_counter = 1;
    let mut modified_html = html.to_string();

    // Iterate over each <code> element in the HTML

    for element in fragment.select(&code_selector) {
        // Extract the class attribute to identify the language
        if let Some(class_attr) = element.value().attr("class") {
            if let Some(captures) = lang_regex.captures(class_attr) {
                let language = captures[1].to_string(); // Extract language
                
                // Extract the code block content (text inside the <code> tag)
                let code_content = element.text().collect::<Vec<_>>().join("\n");

                // Escape HTML characters in the code content
                let escaped_code_content = escape_html(&code_content);

                // Create a new Cblock
                let block = Cblock {
                    id: id_counter,
                    code: code_content.clone(),
                    lang: language.clone(),
                };

                code_blocks.push(block);

                // Replace the code block content in the HTML
                let code_html = format!(r#"<code class="language-{}">{}</code>"#, language, escaped_code_content);
                let placeholder = format!(r#"<code text-decoration= none; class="language-{}">[Code Block {}]</code>"#, language, id_counter);
                modified_html = modified_html.replace(&code_html, &placeholder);

                id_counter += 1;
            }
        }
    }

    // Create Omark struct
    let omark = Omark {
        amount: (id_counter - 1) as i8,  // Amount is the number of code blocks found
        ohtml: modified_html,
    };

    (code_blocks, omark)  // Return both the Cblock vector and mark
}


