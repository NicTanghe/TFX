use leptos::{ev::SubmitEvent, *};
use markdown;

use crate::modules::textfields::high_comms::*;

use regex::Regex;
use scraper::{Html, Selector};

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

// Assuming Omark and Cblock are also serializable
#[derive(Serialize, Deserialize, Clone)]
struct Cblock {
    id: i8,
    code: String,
    lang: String,
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
}// Function to highlight code blocks asynchronously
 //
 // Function to asynchronously fetch and highlight code
async fn highlight_first_block(all_stat: AllStat) -> String {
    if let Some(first_block) = all_stat.code.get(0) {
        let code_l = first_block.code.clone();
        let lang_l = first_block.lang.clone();
        
        // Perform the async task
        match send_code_for_highlighting(&code_l, &lang_l).await {
            Ok(highlighted) => highlighted,
            Err(_) => "".to_string(), // Handle errors, return an empty string
        }
    } else {
        "".to_string() // Handle the case where no code blocks exist
    }
}
async fn highlight_code_blocks(code_blocks: Vec<Cblock>) -> Vec<Cblock> {
    let mut highlighted_blocks = Vec::new();

    for block in code_blocks {
        let code_clone = block.code.clone();
        let lang_clone = block.lang.clone();

        // Await the highlighting of the code block
        let highlighted_code = send_code_for_highlighting(&code_clone, &lang_clone).await.unwrap_or_else(|_| "Error highlighting code".to_string());

        // Create a new Cblock with the highlighted code
        highlighted_blocks.push(Cblock {
            id: block.id, // Retain the original ID
            code: highlighted_code, // Use the highlighted code
            lang: block.lang, // Retain the original language
        });
    }
    return highlighted_blocks;
}

// Function to map languages to their proper syntect language identifier
fn map_language(language: &str) -> String {
    let mut lang_map = HashMap::new();
    
    // Add mappings
    lang_map.insert("rust", "rs");
    lang_map.insert("Rust", "rs");
    lang_map.insert("python", "py");
    lang_map.insert("Python", "py");
    lang_map.insert("javascript", "js");
    lang_map.insert("JavaScript", "js");
    lang_map.insert("typescript", "ts");
    lang_map.insert("TypeScript", "ts");
    lang_map.insert("c++", "cpp");
    lang_map.insert("C++", "cpp");
    lang_map.insert("Go", "go");
    lang_map.insert("Java", "java");

    // Return mapped value or the original language if not found
    lang_map.get(language).unwrap_or(&language).to_string()
    // its probably best to do this in the api instead of here. !!!!!!!!!!!!!
    //
}

// Function to extract code blocks from HTML-formatted markdown and return both Cblock and Omark
fn extract_code_blocks_from_html(html: &str) -> (Vec<Cblock>, Omark) {
    let mut code_blocks: Vec<Cblock> = Vec::new();
    let fragment = Html::parse_fragment(html);

    // Selector to find the <code> elements
    let code_selector = Selector::parse("code").unwrap();

    // Regex to capture the language from the class attribute (e.g., "//language-rust")
    let lang_regex = Regex::new(r"language-(\w+)").unwrap();

    let mut id_counter = 1;
    let mut modified_html = html.to_string();

    // Iterate over each <code> element in the HTML
    for element in fragment.select(&code_selector) {
        // Extract the class attribute to identify the language
        if let Some(class_attr) = element.value().attr("class") {
            if let Some(captures) = lang_regex.captures(class_attr) {
                let raw_language = captures[1].to_string(); // Extract the raw language
                let language = map_language(&raw_language); // Convert the language to its proper format                                                        //
                
                // Extract the code block content (text inside the <code> tag)
                let code_content = element.text().collect::<Vec<_>>().join("\n");

                // Create a new Cblock
                let block = Cblock {
                    id:id_counter,
                    code: code_content.clone(),
                    lang: language.clone(),
                };

                code_blocks.push(block);

                // Replace the code block content in the HTML
                let code_html = format!(r#"<code class="language-{}">{}</code>"#, language, code_content);
                let placeholder = format!(r#"<code class="language-{}">[Code Block {}]</code>"#, language, id_counter);
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
//hooraaaay vex synthax highlighting incomming


async fn assemble_highlighted_content(allstat: AllStat) -> String {
    let mut final_html = allstat.orig.ohtml.clone();
    let mut insertion_point = 0; // Track the insertion point

    // Iterate over the code blocks, send each one for syntax highlighting
    for block in &allstat.code {
        if let Ok(highlighted) = send_code_for_highlighting(&block.code, &block.lang).await {
            // Find the position of the next code block marker in the original HTML
            let code_block_marker = format!("<code id=\"{}\">", block.id);
            if let Some(pos) = final_html[insertion_point..].find(&code_block_marker) {
                // Adjust position to the full string
                let insert_pos = insertion_point + pos + code_block_marker.len();

                // Inject the highlighted code at the correct position
                final_html.insert_str(insert_pos, &highlighted);

                // Update the insertion point for further blocks
                insertion_point = insert_pos + highlighted.len();
            }
        }
    }

    final_html
}

#[component]
pub fn ControlledWriting() -> impl IntoView {
    // create a signal to hold the value of the textarea input
    let (plainstring, set_plainstring) = create_signal("Uncontrolled".to_string());
    let (code, set_code) = create_signal("fn main() { println!(\"Hello, world!\"); }".to_string());

    let (highlighted_code,set_highlighted_code) = create_signal("".to_string());

    let (allstat,set_allstat) = create_signal(AllStat{
        orig: Omark{
            amount:0,
            ohtml:"".to_string()
        },
        code: vec![
            Cblock {
                id: 0,
                code: "".to_string(),
                lang: "Error".to_string(),
            }
        ]});
    


    // Create a resource to fetch the highlighted HTML asynchronously
    let highlighted_html = create_resource(
        move || code.get(), // the code dependency
        move |code| async move {
            send_code_for_highlighting(&code, "html").await.unwrap_or_else(|_| "Error highlighting code".to_string())
        },
    );
    // Create a signal to hold the modified HTML
    let (modified_html, set_modified_html) = create_signal(Omark {
        amount:0,
        ohtml:"code removal error".to_string(),
    });



    
    // Create a resource for extracting code blocks and modified HTML
    let Allstat_resource = create_resource(code, move |code| {
        
        // Run this in an async block
        async move {
            let (code_blocks, omark) = extract_code_blocks_from_html(&code);
            
            // Create and return AllStat
            AllStat {
                orig: omark,
                code: code_blocks,
            }
        }
    });


    // Create a signal to hold the vector of code blocks
    let (code_blocks_signal, set_code_blocks_signal) = create_signal(vec![
        Cblock {
            id: 0,
            code: "".to_string(),
            lang: "none".to_string(),
        },
    ]);



    
    view! {
        <textarea
            // Fire an event whenever the input changes
            on:input=move |ev| {
                // Update the signal with the current value
                set_plainstring(event_target_value(&ev));

                // Convert the plainstring to HTML
                let html_code = markdown::to_html(&plainstring.get());
                set_code(html_code.clone());

                // Additional logic can be added here if needed for further processing
            }
            // Bind the current value to the textarea
            prop:value=plainstring
        />

        //<p>"Code input:"</p>
        //<pre>{code}</pre>

        //<p>"Marked down"</p>
        //<div inner_html=move || code.get()></div>

        <p>"Highlighted Output:"</p>
        // Display the highlighted HTML once it's available
        <div inner_html=move || highlighted_html.get().unwrap_or_default()></div>

        <p>"Highlighted Output from AllStat:"</p>


        <Await
            future=move || {
                // Unwrap the Option, panic if it's None
                async move {
                    let all_stat = Allstat_resource.get().expect("Allstat resource not available");
                    highlight_first_block(all_stat)
                }
            }
            let:highlighted_code
        >
            // Use the highlighted code in the view
            <div inner_html=highlighted_code />
        </Await>


        
    }
}
#[component]
pub fn UnControlledWriting() -> impl IntoView {
    // Use NodeRef for Textarea
    use leptos::html::Textarea;

    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<Textarea> = create_node_ref();

    // Fires when the form `submit` event happens
    let on_submit = move |ev: SubmitEvent| {
        // Stop the page from reloading!
        ev.prevent_default();

        // Extract the value from the textarea
        let value = input_element()
            .expect("<textarea> to exist")
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <textarea
                // Set the initial value of the textarea
                value=name

                // Store a reference to this textarea in `input_element`
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
        <p>"Markdown Input:"</p>
        <div inner_html=move || markdown::to_html(&name.get())></div> 
    }
}

