use leptos::*;
use leptos_router::*;



use crate::contacts::*;


use crate::modules::blog_posts::blog_compo::*;
use crate::modules::blog_posts::blog_fn::*;

use crate::modules::textfields::skrijf::*;

use crate::modules::cookies::cookie;
use crate::modules::cookies::cookie::CookieKey;



use leptos_meta::*;

use tracing::debug;

use std::time::Duration;

use serde::{Serialize,Deserialize};

use rand::seq::SliceRandom; // For random selection from slices

use wasm_cookies::cookies;


#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ActiveUser {
    pub name: String,
    pub token: String,
    pub roles: Vec<String>,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserSession(Option<String>);




#[component]
pub fn FormExample() -> impl IntoView {
    // reactive access to URL query
    let query = use_query_map();
    let name = move || query().get("name").cloned().unwrap_or_default();
    let number = move || query().get("number").cloned().unwrap_or_default();

    view! {
        // read out the URL query strings
        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{name()}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number()}</td>
            </tr>
        </table>

        // <Form/> will navigate whenever submitted
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="name" value=name()/>
            <input type="number" name="number" value=number()/>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
    }
}


fn generate_random_user() -> ActiveUser {
    let names = ["Alice", "Bob", "Charlie", "David", "Eve"];
    let tokens = ["token1", "token2", "token3", "token4", "token5"];
    let roles = vec!["admin", "editor", "viewer","horntoad"];

    let mut rng = rand::thread_rng();
    
    ActiveUser {
        name: names.choose(&mut rng).unwrap().to_string(),
        token: tokens.choose(&mut rng).unwrap().to_string(),
        roles: roles.choose_multiple(&mut rng, 2).map(|&s| s.to_string()).collect(), // Choose 2 random roles
    }
}


#[component]
fn UserElement(user_l2: ReadSignal<ActiveUser>, set_user_l2: WriteSignal<ActiveUser>) -> impl IntoView {
    view! {
        <div class="hover-element" style="position: absolute; top: 10px; right: 10px;">
            {
                move || {
                    let user = user_l2.get(); // Reactively get the current user state
                    if user.name != "" {
                        // Show welcome message when user is logged in
                        view! { 
                            <span>{format!("Welcome back, {}", user.name)}</span>
                        }.into_view()
                    } else {
                        // Show login message and button when no user is logged in
                        view! { 
                            <>
                                <span>"Please log in "</span>
                                <button on:click=move |_| {
                                    // Logging the click event
                                    debug!("Button clicked to set user to 'Bob'");

                                    set_user_l2.update(|user| {
                                        // Log before updating the fields
                                        debug!("Previous user state: {:?}", user);

                                        // Update user details
                                        *user = generate_random_user();
                                        cookie::cookieops::set(
                                            &cookie::CookieKey::Other("user"),
                                            &serde_json::to_string(user).expect("Failed to serialize user"),
                                            Duration::new(523457, 0)
                                            );

                                        // Log after updating
                                        debug!("Updated user state: {:?}", user);
                                    });
                                }>"Click Me"</button>
                            </>
                        }.into_view()
                    }
                }
            }
        </div>
        // Reactively display the user's name
        <div>{move || user_l2.get().name.clone()}</div>
    }
}





#[component]
fn NavBar(user_l1: ReadSignal<ActiveUser>,set_user_l1: WriteSignal<ActiveUser>) -> impl IntoView {
    // Get the current location (path)
    let location = use_location();

    // Helper function to check if the current path starts with the link's href
    let is_active = move |base: &str| location.pathname.get().starts_with(base);

    view! {

        //all the navlinks
        
        <nav class="navbar">
            <A class=move || format!("navlink{}", if is_active("/") && location.pathname.get() == "/" { " nb-active" } else { "" }) href="/">"Home"</A>
            <A class=move || format!("navlink{}", if is_active("/contacts") { " nb-active" } else { "" }) href="/contacts">"Contacts"</A>
            <A class=move || format!("navlink{}", if is_active("/blog") { " nb-active" } else { "" }) href="/blog">"Blog"</A>
            <A class=move || format!("navlink{}", if is_active("/testing") { " nb-active" } else { "" }) href="/testing">"Testing"</A>

            <UserElement user_l2=user_l1 set_user_l2=set_user_l1/>

        </nav>

        //the user login pane;
        

    }
}

// note look at using a sagnal aswess for increased responsiveness


#[server(GetActiveUset, "/ActiveUser")]
pub async fn get_user_details() -> Result<Option<ActiveUser>, ServerFnError> {
    use  leptos_axum::extract;
    // Retrieve the stored JSON string from the cookie
    let headers: http::HeaderMap = extract().await?;
    
    // Define the cookie key
    let cookie_key = CookieKey::Other("user");

    // Check for user cookie using the key
    if let Ok(Some(user_str)) = cookie::cookieops::get(&cookie_key, &headers) {
        // Deserialize and return the ActiveUser
        if let Ok(loaded_user) = serde_json::from_str::<ActiveUser>(&user_str) {
            return Ok(Some(loaded_user));
        }
    }

    // Default return if no cookie or deserialization fails
    Ok(None)
}


#[component]
pub fn App() -> impl IntoView {
   


    let (get_user,set_user) = create_signal(
       ActiveUser{
           name:"".to_string(),
           token:"".to_string(),
           roles:["".to_string()].to_vec()
       }
        );

        let async_UserCookieData = create_resource(
        move || (),  // Pass an empty tuple as a dependency to ensure it runs once
        move |_| async move {
            debug!("RESOURCE: loading data from user Cookies");
            get_user_details().await
        },
    );


    // Update user details
    create_effect(move |_| {
        if let Some(fetched_usercookie) = async_UserCookieData.get() {
            // Properly handle the Result<Option<ActiveUser>, leptos::ServerFnError>
            match fetched_usercookie {
                Ok(Some(active_user)) => {
                    // Proceed with ActiveUser
                    set_user(active_user);
                }
                Ok(None) => {
                    // Handle the case where there is no active user
                    eprintln!("No active user found");
                }
                Err(e) => {
                    // Handle the error case from leptos::ServerFnError
                    eprintln!("Error fetching user cookie: {:?}", e);
                }
            }
        }
    });

    //match get_user_details() {
    //    Some(loaded_user) => {
    //        // If the user details are successfully loaded, update the signal
    //        set_user(loaded_user);
    //        println!("User updated successfully!");
    //
    //    }
    //    None => {
    //        println!("Failed to load user details.");
    //    }
    //}


    //Ok these don`t trow errors but cause a thread panic 


    //if let Some(storage) = window().unwrap().local_storage().unwrap() {
    //    if let Ok(Some(user_data)) = storage.get_item("user_data") {
    //        // Deserialize the JSON string into the ActiveUser struct
    //        if let Ok(user) = serde_json::from_str::<ActiveUser>(&user_data) {
    //            set_user.set(user); // Restore the user state
    //        }
    //    }
    //}

    //create_effect(move |_| {
    //  // immediately prints "Value: 0" and subscribes to `a`
    //  if let Some(storage) = window().unwrap().local_storage().unwrap() {
    //    let user = get_user.get();
    //    let user_data = serde_json::to_string(&user).unwrap();
    //    storage.set_item("user_data", &user_data).unwrap();
    //    }
    //});
    //




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
    

    //add_cookie("presousing".to_string());

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
            <NavBar user_l1=get_user set_user_l1=set_user/>
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
        <div class="big_void"></div>
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
