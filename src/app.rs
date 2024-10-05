use leptos::*;
use leptos_router::*;

use crate::contacts::*;


use crate::modules::blog_posts::blog_compo::*;
use crate::modules::blog_posts::blog_fn::*;

use crate::modules::textfields::skrijf::*;

use leptos_meta::*;

/// Function to create the contact list signal
pub fn create_contact_signal() -> (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>){
    create_signal(vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Steve".to_string(),
        "Diana".to_string(),
        "Eve".to_string(),
        "Fred".to_string(),
        "Camille".to_string(),
        "Tron".to_string()
    ])
}

/// Function to create the post list signal

#[component]
fn NavBar() -> impl IntoView {
    // Get the current location (path)
    let location = use_location();

    // Helper function to check if the current path starts with the link's href
    let is_active = move |base: &str| location.pathname.get().starts_with(base);

    view! {
        <nav class="navbar">
            <A class=move || format!("navlink{}", if is_active("/") && location.pathname.get() == "/" { " nb-active" } else { "" }) href="/">"Home"</A>
            <A class=move || format!("navlink{}", if is_active("/contacts") { " nb-active" } else { "" }) href="/contacts">"Contacts"</A>
            <A class=move || format!("navlink{}", if is_active("/blog") { " nb-active" } else { "" }) href="/blog">"Blog"</A>
            <A class=move || format!("navlink{}", if is_active("/testing") { " nb-active" } else { "" }) href="/testing">"Testing"</A>
        </nav>
    }
}

// note look at using a sagnal aswess for increased responsiveness

#[component]
pub fn App() -> impl IntoView {
 

    let (contacts, _set_contacts) = create_contact_signal();
    let (posts, set_posts) = create_signal(vec![
        Post {
            post_id: 0 as i32,
            title: "server not talked to".to_string(),
            markdown: "# server unreachable".to_string(),
            tags:[{"error".to_string()}].to_vec(),
        },
    ]);

    // Create a resource that fetches posts from the API
    let async_data = create_resource(
        move || (),  // Pass an empty tuple as a dependency to ensure it runs once
        move |_| async move {
            logging::log!("RESOURCE: loading data from API");
            get_post_vector(posts.get()).await
        },
    );

    // Update the posts signal when data is loaded
    create_effect(move |_| {
        if let Some(fetched_posts) = async_data.get() {
            set_posts(fetched_posts);
        }
    });

    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/werk.css" />
        <Title text="Welcome to Leptos"/>
        <Html
            lang="eng"
            dir="ltr"
            attr:data-theme="dark"
        />

        <Router>
            <NavBar />
            <Routes>
                <Route path="/" view=HomePage />  // Home route
                <Route path="/testing" view=move || view! { <ControlledWriting/> } />  // Correctly self-closing

                <Route path="/contacts" view=move || view! { <ContactList contacts /> }>
                    <Route path="" view=|| view! { 
                        <p>"Select a contact to view more info."</p> 
                    } />  // Correctly self-closing
                    <Route path=":id" view=move || view! { <ContactInfo contacts /> }>
                        <Route path="" view=|| view! { 
                            <p>"Select a contact to view more info."</p> 
                        } />  // Correctly self-closing
                    </Route>
                </Route>

                // Uncomment when needed
                // <Route path="/blog" view=move || view! { <PostList posts={posts} /> } />
                {post_routes(posts, set_posts)}  // Ensure this is a valid expression
                                                 //
            </Routes>  // Closing the Routes component
        </Router>  // Closing the Router component
    }

}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
