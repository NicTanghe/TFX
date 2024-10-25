use leptos::{ev::SubmitEvent, *};
use markdown;

use regex::Regex;
use scraper::{Html, Selector};

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::modules::textfields::high_comms::*;

use crate::modules::blog_posts::blog_comms::{create_post_to_blog_api,CreatePostReq};
// Assuming Omark and Cblock are also serializable

use crate::app::ActiveUser;


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

//hooraaaay vex synthax highlighting incomming
   




    
// change the button submit thing so that ControlledWriting takes a function as input for the submit button.
#[component]
pub fn ControlledWriting(get_user: ReadSignal<ActiveUser>) -> impl IntoView {
    
    // blog title, tags
    let (area_title,set_area_title)= create_signal("enter title".to_string());
    // maybe change this to an additive dropdown in the future maybe also add a change to closest match
    
    let (area_tags,set_area_tags)= create_signal("".to_string());


    // blog content variables    
    let (content_string, set_content_string) = create_signal("*enter content*".to_string());
    
    let (code, set_code) = create_signal("fn main() { println!(\"Hello, world!\"); }".to_string());

    let (final_html, set_final_html) =   create_signal("".to_string());





    // Create a resource to fetch the highlighted HTML asynchronously
    let highlighted_html = create_resource(
        move || code.get(), // the code dependency
        move |code| async move {
            match send_code_for_highlighting(&code, "html").await {
                Ok(highlighted_code) => highlighted_code,  // Return the highlighted HTML
                Err(e) => {
                    println!("Error highlighting code: {:?}", e);
                    String::new() // Return an empty string or handle the error accordingly
                }
            }
        },
    );

// try make a signal c if that reduces the laggio

    
    // Create a resource for extracting code blocks and modified HTML
    let Allstat_resource = create_resource(content_string, move |content_string| {
        let html_code = markdown::to_html(&content_string);
        
        // Run this in an async block
        async move {
            let (code_blocks, omark) = extract_code_blocks_from_html(&html_code);
            let syndicated_blocks = highlight_code_blocks(code_blocks).await;
            // Create and return AllStat
            AllStat {
                orig: omark,
                code: syndicated_blocks,
            }
        }
    });

    

     // Create a resource for extracting code blocks and modified HTML
    let Final_resource = create_resource(content_string, move |content_string| {
        let html_code = markdown::to_html(&content_string);
        
        // Run this in an async block
        async move {
            let (code_blocks, omark) = extract_code_blocks_from_html(&html_code);
            let syndicated_blocks = highlight_code_blocks(code_blocks).await;

            // Create AllStat
            let tempstat = AllStat {
                orig: omark,
                code: syndicated_blocks,
            };
            assemble_highlighted_content(tempstat).await
        }
    });   



    view! {
        <div class= "big_void"></div>

        <div class= "text_section">
            <div class= "skrijver_in">
                <textarea class="title" rows=1 style="width:75%"
                    on:input=move |ev_title| {
                        // Update the signal with the current value
                        set_area_title(event_target_value(&ev_title));
                        // Extract code blocks and modified HTML using the `extract_code_blocks_from_html` function
                    }
                    // Use prop:value to bind the current value to the textarea
                    prop:value=area_title
                />
                <textarea class="tags" rows=1 style="width:50% height:1em"
                    on:input=move |ev_tags| {
                        set_area_tags(event_target_value(&ev_tags));
                    
                    }
                    prop:value=area_tags
                />
                <textarea class="blog_area" rows=40 style="width: 100%; max-width: 100%;"
                    // fire an event whenever the input changes
                    on:input=move |ev| {
                        // Update the signal with the current value
                        set_content_string(event_target_value(&ev));
                        set_code(markdown::to_html(&content_string.get()));
                        // Extract code blocks and modified HTML using the `extract_code_blocks_from_html` function
                    }
                    // Use prop:value to bind the current value to the textarea
                    prop:value=content_string
                />
            </div>
            <div class="skrijver_out">
                <Suspense
                    fallback=move || view! { <div inner_html ={final_html}></div> }        
                >

                    <div inner_html=move || {
                        // Get the AllStat from the resource
                        match Final_resource.get() {
                            Some(f_html) => {
                                set_final_html(f_html.clone());
                                // Access the first Cblock and print its code if it exists
                                f_html
                            },
                            None => "".to_string(), // Handle the case where the resource isn't ready yet
                        }
                    }></div>
                </Suspense>
            </div>
        </div>
       <button on:click=move |_| {
            // Assume token is a string

            // Parse the string as JSON
            let jwt: Value = serde_json::from_str(get_user.get().token.as_str()).expect("Failed to parse token");

            // Extract the access_token
            let access_token = jwt["access_token"].as_str().expect("access_token not found").to_string();

            logging::log!("it has pressed my presous");
            spawn_local(async move {
                let _ =create_post_to_blog_api(
                    CreatePostReq {
                        title: area_title.get(),
                        tags: split_tags(&area_tags.get()),
                        markdown: content_string.get(),
                    },

                   access_token 
                ).await; // Awaiting the async function inside the async block
            });
        }>
            "submit!"
        </button>
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

