use leptos::{ev::SubmitEvent, *};
use markdown;

use crate::modules::textfields::high_comms::*;

#[component]
pub fn ControlledWriting() -> impl IntoView {
    // create a signal to hold the value of the textarea input
    let (code, set_code) = create_signal("fn main() { println!(\"Hello, world!\"); }".to_string());

    // Create a resource to fetch the highlighted HTML asynchronously
    let highlighted_html = create_resource(
        move || code.get(), // the code dependency
        move |code| async move {
            send_code_for_highlighting(&code, "rs").await.unwrap_or_else(|_| "Error highlighting code".to_string())
        },
    );

    view! {
        <textarea
            // fire an event whenever the input changes
            on:input=move |ev| {
                // Update the signal with the current value
                set_code(event_target_value(&ev));
            }
            // Use prop:value to bind the current value to the textarea
            prop:value=code
        />
        <p>"Code input:"</p>
        <pre>"{code}"</pre>
        <p>"Highlighted Output:"</p>
        // Display the highlighted HTML once it's available
        <div inner_html=move || highlighted_html.get().unwrap_or_default()></div>
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

