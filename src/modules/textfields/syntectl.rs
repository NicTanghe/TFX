
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use syntect::highlighting::{ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use crate::modules::textfields::skrijf::Cblock;

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


// maybe ad erroring or maybe dont 

// Function to highlight code in HTML
 pub async fn highlight_synthax_to_html(code: &str, language_extension: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ss
        .find_syntax_by_extension(language_extension)
        .expect("Unknown language");

    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";

    let theme = &ts.themes["base16-ocean.dark"];
    let c = theme.settings.background.unwrap_or_else(|| syntect::highlighting::Color::WHITE);

    let html = highlighted_html_for_string(code, &ss, syntax, theme).unwrap();

    let output_html = format!(
        "<head><title>{}</title><style>{}</style></head>
         <body style=\"background-color:#{:02x}{:02x}{:02x};\">\n{}\n</body>",
        language_extension, style, c.r, c.g, c.b, html
    );

    output_html
}

// Function to process the Cblock and highlight the code if necessary
pub async fn process_cblock(mut cblock: Cblock) -> Cblock {
    match map_language(&cblock.lang) {
        LanguageStatus::Existing(mapped_lang) => {
            // Update the language in the Cblock
            cblock.lang = mapped_lang.clone();
            // Highlight the code with the corrected language and return HTML
            cblock.code = highlight_synthax_to_html(&cblock.code, &cblock.lang).await; // Return the HTML here
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

