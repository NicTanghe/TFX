use leptos::*; 
use leptos_router::Outlet;
#[component]
pub fn Portf_simulations() -> impl IntoView {
    // Define categories directly as a Vec<String>

    // Insert `category_links` into the final view
    view! {
        <div class="big_void"></div>
        <h1>"Simulations ! is storytelling"</h1>
        <Outlet/>
    }
}
