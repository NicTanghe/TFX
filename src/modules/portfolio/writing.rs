use leptos::*;
use leptos::prelude::*; 
use leptos_router::components::Outlet;
#[component]
pub fn Portf_writing() -> impl IntoView {
    // Define categories directly as a Vec<String>

    // Insert `category_links` into the final view
    view! {
        <div class="big_void"></div>
        <h1>"Writing ! is the sourcecode of storytelling"</h1>
        <Outlet/>
    }
}
