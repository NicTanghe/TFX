use leptos::{ev::SubmitEvent, *};
use markdown;

use crate::modules::textfields::high_comms::*;
use crate::modules::textfields::syntectl::*;
use regex::Regex;
use scraper::{Html, Selector};

use serde::{Serialize, Deserialize};

// Assuming Omark and Cblock are also serializable
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

#[component]
pub fn ControlledWriting() -> impl IntoView {
    // create a signal to hold the value of the textarea input_e
    let (plainstring, set_plainstring) = create_signal("Uncontrolled".to_string());
    let (code, set_code) = create_signal("fn main() { println!(\"Hello, world!\"); }".to_string());

    let (final_html, set_final_html) =   create_signal("Uncontrolled".to_string());

    // Create a resource to fetch the highlighted HTML asynchronously
    let highlighted_html = create_resource(
        move || code.get(), // the code dependency
        move |code| async move {
            highlight_synthax_to_html(&code, "html").await
        },
    );


// try make a signal c if that reduces the laggio

    
    // Create a resource for extracting code blocks and modified HTML
    let Allstat_resource = create_resource(plainstring, move |plainstring| {
        let html_code = markdown::to_html(&plainstring);
        
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
    let Final_resource = create_resource(plainstring, move |plainstring| {
        let html_code = markdown::to_html(&plainstring);
        
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

    // not sure why this doesnt work
    //let set_final_html_resource = create_resource(
    //  let all_stat = Allstat_resource.get();
    //
    //    async move{
    //       let final_html =  assemble_highlighted_content(all_stat).await
    //    }
    //    final_html
    //);


    // Create a signal to hold the vector of code blocks
    let (code_blocks_signal, set_code_blocks_signal) = create_signal(vec![
        Cblock {
            id: 0,
            code: "".to_string(),
            lang: "none".to_string(),
        },
    ]);



    view! {
        <textarea class="skrijver_in"
            // fire an event whenever the input changes
            on:input=move |ev| {
                // Update the signal with the current value
                //set_code(event_target_value(&ev));
                set_plainstring(event_target_value(&ev));
                set_code(markdown::to_html(&plainstring.get()));
                 // Extract code blocks and modified HTML using the `extract_code_blocks_from_html` function
                let (code_blocks, omark) = extract_code_blocks_from_html(&code.get());
                //ok do whole thing with signals instead

            }
            // Use prop:value to bind the current value to the textarea
            prop:value=plainstring
        />
        <div class="skrijver_out">
            //<p>"Code input:"</p>
            //<pre>{code}</pre>
            //
            <p>"marked down"</p>
            //<div inner_html=move || code.get()></div> 

            //<p>"Highlighted Output:"</p>
            //// Display the highlighted HTML once it's available
            //<div inner_html=move || highlighted_html.get().unwrap_or_default()></div>

            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }        
            >

                <p>"Highlighted Output:"</p>
                <div inner_html=move || {
                    // Get the AllStat from the resource
                    match Final_resource.get() {
                        Some(f_html) => {
                            // Access the first Cblock and print its code if it exists
                            f_html
                        },
                        None => "".to_string(), // Handle the case where the resource isn't ready yet
                    }
                }></div>
            </Suspense>
        </div>
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

