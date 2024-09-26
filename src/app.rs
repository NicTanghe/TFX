use leptos::*;
use leptos_router::*;

use crate::contacts::*;


use crate::modules::blog_posts::blog_compo::*;
use crate::modules::blog_posts::blog_fn::*;



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



    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/contacts">"Contacts"</A>
                <A href="/blog">"blog"</A>   // Added post navigation
            </nav>
            <Routes>
                <Route path="/" view=HomePage/>
                
                <Route path="/contacts" view=move || view!{ <ContactList contacts />}>
                    <Route path="" view=|| view! {
                        <p>"Select a contact to view more info."</p>
                    }/>
                    <Route path=":id" view=move || view!{ <ContactInfo contacts />}>
                        <Route path="" view=|| view! {
                        <p>"this is a test or a conditional footer  "</p>
                        }/>
                    </Route>
                </Route>

                //{post_routes_test()}
                {post_routes(posts,set_posts)}
            </Routes>
        </Router>
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
