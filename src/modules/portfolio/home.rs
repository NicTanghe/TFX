use leptos::prelude::*; 
use leptos_router::{
    NavigateOptions,
    hooks::use_navigate
};
use leptos_router::components::Outlet;
use crate::modules::blog_posts::blog_compo::Post;

/// Renders the home page of your application.
#[component]
fn HomePage_old() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="big_void"></div>
        <h1>"Welcome to Leptos!"</h1>
        <button class="glowy_large" on:click=on_click>"Click Me: " {count}</button>
    }
}

fn gotolink(href: &str) {
    // Use Leptos `navigate` function to navigate to the link
    let navigate = use_navigate();
    navigate(href, NavigateOptions::default());
}

#[component]
pub fn HomePage() -> impl IntoView {
    // Define categories as a Vec<String>
    //
    let gotolink = use_navigate();

    let categories = vec![
        "modelling".to_string(),
        "compositing".to_string(),
        "pipeline".to_string(),
        "editing".to_string(),
        "programming".to_string(),
    ];
    
    // this may be stupid but so are you.
    // Function to get the GIF URL based on category
    let filter_gif = |category: &str| -> Option<String> {
        match category {
            "programming" => Some("http://127.0.0.1:5434/programming.gif".to_string()),
            "modelling" => Some("http://127.0.0.1:5434/programming.gif".to_string()),
            "compositing" => Some("http://127.0.0.1:5434/programming.gif".to_string()),
            "pipeline" => Some("http://127.0.0.1:5434/programming.gif".to_string()),
            "editing" => Some("http://127.0.0.1:5434/programming.gif".to_string()),
            _ => None,
        }
    };

    // Precompute links and GIF URLs to avoid moving `cat` multiple times
    let category_links: Vec<_> = categories
        .into_iter()
        .map(|cat| {
            let href = format!("/home/{}", cat);
            let gif_url = filter_gif(&cat); // Pass `cat` as `&str`
            (href, gif_url, cat.clone(), cat) // Return a tuple with two copies of `cat`
        })
        .collect();

    // Signal to store the last visited link
    let (last_link, set_last_link) = signal(None::<String>);
    
    let precomputed_links: Vec<_> = category_links
        .into_iter()
        .map(|(href, gif_url, cat_for_a, cat_for_img)| {
            let (href_signal,_href_signal_set) = signal(href.clone());
            let cat_for_a_cloned = cat_for_a.clone();
            let cat_for_img_cloned = cat_for_img.clone();
            let gotolink_cloned = gotolink.clone(); // Clone `gotolink` to prevent move issues
            
            let last_link_clone = last_link.clone();
            // Create the necessary components for each category
            


            view! {
                
                <div 
                    class=move || {
                        if last_link_clone.with(|last| last.as_ref() == Some(&href.clone())) {
                            "active".to_string() // Active class
                        } else {
                            "".to_string() // Default class
                        }
                    }
                    style=format!("background: url({}) no-repeat 100% / cover;", gif_url.clone().unwrap_or_default())
                    on:mouseover=move |_| {
                        // Simple navigation logic
                        if last_link.with(|last| last.as_ref() != Some(&href_signal.get())) {
                            gotolink_cloned(&href_signal.get(), NavigateOptions::default());
                            set_last_link(Some(href_signal.get()));
                        }
                    }
                >
                    <div 
                        class=move || {
                            if last_link_clone.with(|last| last.as_ref() == Some(&href_signal.get())) {
                                "content active".to_string() // Active class
                            } else {
                                "content".to_string() // Default class
                            }
                        }
                    > 
                        <h2>{cat_for_a_cloned}</h2>
                        <span>"Lorem Ipsum Fixum Later"</span>
                    </div>
                </div>
            }
        })
        .collect();


        view! {
            <div class= "portf_container">
                {precomputed_links}
            </div>
            <Outlet/>
        }
    }

            
//<div class="container">
//  <div>
//    <div class="content">
//      <h2>Jane Doe</h2>
//      <span>UI & UX Designer</span>
//    </div>
//  </div>
//  <div>
//    <div class="content">
//      <h2>Alex Smith</h2>
//      <span>CEO Expert</span>
//    </div>
//  </div>
//  <div>
//    <div class="content">
//      <h2>Emily New</h2>
//      <span>Web Designer</span>
//    </div>
//  </div>
//  <div>
//    <div class="content">
//      <h2>Lisa Boley</h2>
//      <span>Marketing Coordinator</span>
//    </div>
//  </div>
//</div>
