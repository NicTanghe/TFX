use leptos::*; 
use leptos_router::A;

use std::{
    rc::Rc
};

/// Renders the home page of your application.
#[component]
fn HomePage_old() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="big_void"></div>
        <h1>"Welcome to Leptos!"</h1>
        <button class="glowy_large" on:click=on_click>"Click Me: " {count}</button>
    }
}



#[component]
pub fn HomePage() -> impl IntoView {
    // Define categories directly as a Vec<String>
    let categories = vec![
        "modelling".to_string(),
        "compositing".to_string(),
        "pipeline".to_string(),
        "editing".to_string(),
    ];

    // Generate the links by cloning each category string as needed
    let category_links = categories.into_iter().map(|cat| {
        let href = format!("/home/{}", cat);
        view! {
            <A href={href}>{cat.clone()}</A>
        }
    }).collect_view();

    // Insert `category_links` into the final view
    view! {
        <div class="big_void"></div>
        <h1>"Welcome to TFX!"</h1>
        <div> "This portal is designed to streamline customer and collaborator interaction"</div>    
        <div> "It also serves as our portfolio" </div>
        {category_links}
    }
}

