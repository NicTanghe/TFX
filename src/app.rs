use leptos::*;
use leptos::ev::SubmitEvent;
use leptos_router::*;


use crate::modules::{

    blog_posts::{
        blog_compo::*,
        blog_fn::*,
    },

    people::{
        people_comms::*,
        people_main::*,
    },

    textfields::skrijf::*,

    auth_fc::{
        cookie,
        auth_ta::get_access_token,
    }
};

use leptos_meta::*;

use tracing::debug;

use std::time::Duration;

use serde::{Serialize,Deserialize};

use rand::seq::SliceRandom; // For random selection from slices


use gloo_timers::callback::Timeout;
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


// you need to set a global loggin attempt failure somewhere not cookie based. or you could encrypt
// the cookie
#[component]
fn UncontrolledComponent(set_user_l3: WriteSignal<ActiveUser>) -> impl IntoView {
    use leptos::html::Input;

    let (userName, set_userName) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    // Separate NodeRefs for each input element
    let username_input: NodeRef<Input> = create_node_ref();
    let password_input: NodeRef<Input> = create_node_ref();

    // Fires when the form `submit` event happens
    let on_submit = move |ev: SubmitEvent| {
        // Stop the page from reloading
        ev.prevent_default();

        // Extract the values from both input fields
        if let (Some(username_input_element), Some(password_input_element)) = (
            username_input(), // Access the username input element
            password_input(), // Access the password input element
        ) {
            // Update the signals with the input values
            set_userName(username_input_element.value());
            set_password(password_input_element.value());
            set_user_l3.update(|user| {
                debug!("Previous user state: {:?}", user);

                // Clone the necessary data from the user
                let mut user_clone = user.clone();

                // Asynchronously fetch the access token
                leptos::spawn_local(async move {
                    match get_access_token(username_input_element.value().to_string(), password_input_element.value().to_string()).await {
                        Ok(token) => {
                            // Token retrieved successfully, update the user's token
                            user_clone.token = token;
                            user_clone.name = username_input_element.value().to_string();

                            debug!("Updated user token: {:?}", user_clone.token);
                        }
                        Err(err) => {
                            // Error handling, set the token to "Error"
                            user_clone.token = "Error".to_string();
                            debug!("Failed to get token, set to 'Error'. Error: {:?}", err);
                        }
                    }

                    set_user_l3.update(|user| {
                        *user = user_clone.clone(); // Update with the new token
                        debug!("User state updated: {:?}", user);

                        // Determine the duration based on whether the token contains "Error"
                        let cookie_duration = if user_clone.token.contains("Error") {
                            Duration::new(2, 0)  // Short duration for "Error" tokens
                        } else {
                            Duration::new(324890, 0)  // Default duration
                        };

                        // Handle cookie creation
                        cookie::cookieops::set(
                            &cookie::CookieKey::Other("user"),
                            &serde_json::to_string(&user_clone).expect("Failed to serialize user"),
                            cookie_duration
                        );
                    });
                });
            });
        }
    };

    view! {

        <div class="content">

            <form on:submit=on_submit>
                <input type="text"
                    value=userName
                    node_ref=username_input
                />
                <span class="span">
                    //<svg class="log_icon" style="enable-background:new 0 0 512 512" viewBox="0 0 512 512" height="20" width="50" xmlns="http://www.w3.org/2000/svg">
                    //    <g>
                    //        <path class="" data-original="#000000" fill="#595959" d="M256 0c-74.439 0-135 60.561-135 135s60.561 135 135 135 135-60.561 135-135S330.439 0 256 0zM423.966 358.195C387.006 320.667 338.009 300 286 300h-60c-52.008 0-101.006 20.667-137.966 58.195C51.255 395.539 31 444.833 31 497c0 8.284 6.716 15 15 15h420c8.284 0 15-6.716 15-15 0-52.167-20.255-101.461-57.034-138.805z"></path>
                    //    </g>
                    //</svg>
                </span>
                <input type="password"
                    value=password
                    node_ref=password_input
                />
                <span class="span">
                    //<svg class="log_icon" style="enable-background:new 0 0 512 512" viewBox="0 0 512 512" height="20" width="50" xmlns="http://www.w3.org/2000/svg">
                    //    <g>
                    //        <path class="" data-original="#000000" fill="#595959" d="M336 192h-16v-64C320 57.406 262.594 0 192 0S64 57.406 64 128v64H48c-26.453 0-48 21.523-48 48v224c0 26.477 21.547 48 48 48h288c26.453 0 48-21.523 48-48V240c0-26.477-21.547-48-48-48zm-229.332-64c0-47.063 38.27-85.332 85.332-85.332s85.332 38.27 85.332 85.332v64H106.668zm0 0"></path>
                    //    </g>
                    //</svg>
                </span>
                <button type="submit">Sign in</button>
            </form>
            //<label class="label">Password</label>
            //<div class="forgot-pass">
            //    <a href="#">Forgot Password?</a>
            //</div>
        </div>
        //<p>"Name is: " {userName}</p>
        //<p>"Name is: " {password}</p>
    }
}


#[component]
fn UserElement(
    user_l2: ReadSignal<ActiveUser>, 
    set_user_l2: WriteSignal<ActiveUser>, 
    is_hiding_l: ReadSignal<u8>
) -> impl IntoView {

    // ok maybe do make this an enum although its stil less space.
    let hide_class =move || match is_hiding_l.get() {
        0 => "hover-element from-hiding",
        1 => "hover-element showing",
        2 => "hover-element to-hiding",
        _ => "",
    };
    view! {
        <div class=hide_class style="position: absolute; top: 10px; right: 10px;">
            {
                move ||{
                    let user = user_l2.get(); // Reactively get the current user state

                    if user.token.contains("Error") {
                        view!{
                            <span>{format!("Wrong password or username ; {}", user.name)}</span>
                            // make it only clickable every 20 seconds;
                            // note they can still cycle the token itself so maybe put a limiter on

                            // but maybe still put an update on the effect to limit how fast it
                            // will update although you can probbably best just block ip`s from a
                            // firewal if a single user starts spamming requests with different
                            // tokens
                            //
                            <button on:click=move |_| {
                                debug!("Button clicked to set user to 'Bob'");

                                set_user_l2.update(|user| {
                                    debug!("Previous user state: {:?}", user);

                                    // Clear and reset fields directly
                                    user.name.clear(); // or set to a specific value, e.g., user.name = "Bob".to_string();
                                    user.token.clear(); // or set to a specific value
                                    user.roles.clear(); // This clears the vector of roles

                                    // Set the cookie after updating the fields
                                    cookie::cookieops::set(
                                        &cookie::CookieKey::Other("user"),
                                        &serde_json::to_string(&user).expect("Failed to serialize user"),
                                        Duration::new(0, 0)
                                    );

                                    debug!("Updated user state: {:?}", user);
                                });
                            }>"try_again"</button>
                        }.into_view()
                    }
                    else if user.name != "" {
                        // Show welcome message when user is logged in
                        view! { 
                            <span>{format!("Welcome back, {}", user.name)}</span>
                            
                            // make this into a widget since you use it twice

                            <button on:click=move |_| {
                                debug!("Button clicked to set user to 'Bob'");

                                set_user_l2.update(|user| {
                                    debug!("Previous user state: {:?}", user);

                                    // Clear and reset fields directly
                                    user.name.clear(); // or set to a specific value, e.g., user.name = "Bob".to_string();
                                    user.token.clear(); // or set to a specific value
                                    user.roles.clear(); // This clears the vector of roles

                                    // Set the cookie after updating the fields
                                    cookie::cookieops::set(
                                        &cookie::CookieKey::Other("user"),
                                        &serde_json::to_string(&user).expect("Failed to serialize user"),
                                        Duration::new(0, 0)
                                    );

                                    debug!("Updated user state: {:?}", user);
                                });
                            }>"Log Out"</button>
                        }.into_view()
                    } else {
                        // Show login message and button when no user is logged in
                        view! { 
                            <>
                    
                            <UncontrolledComponent set_user_l3=set_user_l2/> 

                            </>
                        }.into_view()
                    }
                }
            }
        </div>
        // Reactively display the user's name
    }
}





#[component]
fn NavBar(user_l1: ReadSignal<ActiveUser>,set_user_l1: WriteSignal<ActiveUser>) -> impl IntoView {
    // Get the current location (path)
    let location = use_location();

    let (show_card, set_show_card) = create_signal(false); //proly dont need
                                                           //
    //rename to hiding state
    let (is_hiding, set_hiding) =create_signal(2 as u8);
    

    // Helper function to check if the current path starts with the link's href
    let is_active = move |base: &str| location.pathname.get().starts_with(base);

    view! {

        //all the navlinks
        <div>
            <nav class="navbar">
                <div class = "bg">
                    <A class=move || format!("navlink{}", if is_active("/") && location.pathname.get() == "/" { " nb-active" } else { "" }) href="/">"Home"</A>
                    <A class=move || format!("navlink{}", if is_active("/contacts") { " nb-active" } else { "" }) href="/contacts">"Contacts"</A>
                    <A class=move || format!("navlink{}", if is_active("/people") { " nb-active" } else { "" }) href="/people">"people"</A>
                    <A class=move || format!("navlink{}", if is_active("/blog") { " nb-active" } else { "" }) href="/blog">"Blog"</A>
                    //<A class=move || format!("navlink{}", if is_active("/testing") { " nb-active" } else { "" }) href="/testing">"Testing"</A>
                    <button class= "toggle_userElelemet" on:click=move |_| {
                            match is_hiding.get() {
                        2 => { 
                            set_show_card.set(true); // start showing
                            set_hiding.set(0); // transition to "from hiding" state
                            Timeout::new(0_500,move || {
                                set_hiding.set(1)
                            }).forget();
                        },
                        1 => { 
                            set_hiding.set(2); // initiate hiding transition
                            // Delay hiding until animation completes
                            Timeout::new(0_500,move || {
                                set_show_card.set(false)
                            }).forget();

                        },
                        _ => {}
                    }}>
                       <svg class="log_icon" style="enable-background:new 0 0 512 512" viewBox="0 0 512 512" height="20" width="50" xmlns="http://www.w3.org/2000/svg">
                        <g>
                            <path class="" data-original="#000000" fill="#595959" d="M256 0c-74.439 0-135 60.561-135 135s60.561 135 135 135 135-60.561 135-135S330.439 0 256 0zM423.966 358.195C387.006 320.667 338.009 300 286 300h-60c-52.008 0-101.006 20.667-137.966 58.195C51.255 395.539 31 444.833 31 497c0 8.284 6.716 15 15 15h420c8.284 0 15-6.716 15-15 0-52.167-20.255-101.461-57.034-138.805z"></path> 
                        </g>
                    </svg>
                        
                    </button>

                    
                    // Conditionally render `UserElement` based on `show_card`
                    {move || if show_card.get() {
                        view! { <UserElement user_l2=user_l1 set_user_l2=set_user_l1 is_hiding_l=is_hiding /> }.into_view()
                    } else {
                        view! { <></> }.into_view() // Empty view when not shown
                    }}
                </div>
            </nav>
        </div>
    }
}

// note look at using a sagnal aswess for increased responsiveness


#[server(GetActiveUset, "/ActiveUser")]
pub async fn get_user_details() -> Result<Option<ActiveUser>, ServerFnError> {
    use  leptos_axum::extract;
    // Retrieve the stored JSON string from the cookie
    let headers: http::HeaderMap = extract().await?;
    
    // Define the cookie key
    let cookie_key = cookie::CookieKey::Other("user");

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



    let jwt: serde_json::Value = match serde_json::from_str(&get_user.get().token) {
        Ok(token) => token,
        Err(_) => {
            // Set to an invalid or default token value when deserialization fails
            serde_json::json!({"access_token": "NANNI !!"})

        }
    };

    let access_token = jwt["access_token"].as_str().expect("access_token not found").to_string();



    //_____________
    //__ posts ____
    //_____________



    let (posts, set_posts) = create_signal(vec![
        Post {
            post_id: 0 as i32,
            title: "server not talked to".to_string(),
            markdown: "# server unreachable".to_string(),
            tags:[{"error".to_string()}].to_vec(),
        },
    ]);

    // Create a resource that fetches posts from the API
    let async_data_posts = create_resource(
        move || (),  // Pass an empty tuple as a dependency to ensure it runs once
        move |_| async move {
            logging::log!("RESOURCE: loading data from API");
            get_post_vector(posts.get()).await
        },
    );


    // Update the posts signal when data is loaded
    create_effect(move |_| {
        if let Some(fetched_posts) = async_data_posts.get() {
            set_posts(fetched_posts);
        }
    });
   
    //_____________
    //__ people ___
    //_____________



    let (people, set_people) = create_signal(vec![
        Person {
            id: 0 as i32,
            name: Some("server not talked to".to_string()),
            description: Some("# server unreachable".to_string()),
            contact_info: None,
            profile_pic: None
        },
    ]);

    let async_data_people = create_resource(
        move || (), // Pass an empty tuple as a dependency to ensure it runs once
        move |_| {
            let atoken = access_token.clone();
            async move {
                logging::log!("RESOURCE: loading data from API");

                // Await the result of the API call and handle the response
                let people_vector = get_people_vector(people.get(), atoken.clone()).await;
                logging::log!("{}",atoken);
                people_vector // omg you did not return it you fucking idiot
            }
        }
    );

    //ok next up try it in the function itself don`t make no sence that it doesn`t work here.
    // Update the posts signal when data is loaded
    create_effect(move |_| {
        if let Some(fetched_people) = async_data_people.get() {
            set_people(fetched_people);
        }
    });


    //provide_meta_context();

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
                <Route path="/testing" view=move || view! { <ControlledWriting get_user/> } />  // Correctly self-closing
                <Route path="/people" view=move || view! { <PeopleList people /> }>
                </Route>

                // a bit wonky but not as spaghetty as passing get_user 3 times 
                // we have to pass it annyway to add the + conditionally
                <Route path="/blog" view=move || view! { <PostList posts={posts} /> }>   
                    <Route path="/newpost" view=move || view! { <ControlledWriting get_user/> } /> 
                </Route>

                // Uncomment when needed
                // <Route path="/blog" view=move || view! { <PostList posts={posts} /> } />
                {post_routes(posts, set_posts)}  // Ensure this is a valid expression
                                                 //
            </Routes>  // Closing the Routes component
        </Router>  // Closing the Router component
                   //
        <footer>
            <p>Copyright &copy; TFX 2024. All Rights Reserved.</p>
        </footer>

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
        <button class="glowy_large" on:click=on_click>"Click Me: " {count}</button>
    }
}
