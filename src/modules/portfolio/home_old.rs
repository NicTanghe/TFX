use leptos_router::{
    A,Outlet,NavigateOptions,use_navigate,
};
use std::sync::Arc;

use crate::modules::blog_posts::blog_compo::Post;
 
use crate::modules::cdn::images::fetch_image_from_cdn;
//use Engine::encode;
use base64::{engine::general_purpose::STANDARD, Engine as _};
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

fn gotolink(href: &str) {
    // Use Leptos `navigate` function to navigate to the link
    let navigate = use_navigate();
    navigate(href, NavigateOptions::default());
}


// we create a struct for this with the sole reason to be able to copy the array to avoid ownership
// issues if you change the amount of categories the thing needs to change aswell.
#[derive(Copy, Clone)]
struct Categories {
    data: [[&'static str; 2]; 5],  // A 2D array where each element is a fixed-size array of 2 strings
}








#[component]
pub fn HomePage_base64() -> impl IntoView {
    let gotolink = use_navigate();

    // Define the available categories and their corresponding image paths
    let categories = Categories {
        data: [
            ["modelling", "/programming.gif"],
            ["compositing", "/programming.gif"],
            ["pipeline", "/programming.gif"],
            ["editing", "/programming.gif"],
            ["programming", "/programming.gif"],
        ],
    };

    // Signal to store the fetched images as data URIs
    let (image_data_uris, set_image_data_uris) = create_signal(Vec::<Option<String>>::new());

    // Create a resource that fetches posts from the API
    let async_data_image_uris = create_resource(
        move || (), // Ensure it runs once
        move |_| {
            let categories_c = categories; // We can move because `Categories` is Copy
            async move {
                let mut fetched_images = Vec::new();
                for category in categories_c.data.iter() {
                    match fetch_image_from_cdn(category[1].to_string()).await {
                        Ok(image_bytes) => {
                            fetched_images.push(Some(STANDARD.encode(&image_bytes)));
                        }
                        Err(_) => fetched_images.push(None), // Handle errors
                    }
                }
                fetched_images
            }
        },
    );

    // Update the image data URIs when the resource loads
    create_effect(move |_| {
        if let Some(fetched_images) = async_data_image_uris.get() {
            set_image_data_uris(fetched_images.clone());
        }
    });

    // Signal to track the last visited link
    let (last_link, set_last_link) = create_signal(None::<String>);


 


    let precomputed_links: Vec<_> = categories
        .data
        .iter()
        .enumerate()
        .map(|(idx, category)| {
            let href = Arc::new(format!("/home/{}", category[0])); // Wrap the `String` in an Arc
            let last_link_clone = last_link.clone();
            let gotolink_cloned = gotolink.clone();
            let href_cloned_for_style = Arc::clone(&href); // Clone Arc for the style closure
            let href_cloned_for_mouseover = Arc::clone(&href); // Clone Arc for the mouseover closure
            view! {
                <div
                    class=move || {
                        if last_link_clone.with(|last| last.as_ref() == Some(&*href)) {
                            "active".to_string()
                        } else {
                            "".to_string()
                        }
                    }
                    style=move || {
                        let uris = image_data_uris.get();
                        uris.get(idx).and_then(|uri| uri.clone())
                            .map_or("".to_string(), |uri| format!("background: url(data:image/gif;base64,{}) no-repeat 100% / cover;", uri))

                    }
                    on:mouseover=move |_| {
                        if last_link.with(|last| last.as_ref() != Some(&*href_cloned_for_mouseover)) {
                            gotolink_cloned(&href_cloned_for_mouseover, NavigateOptions::default());
                            set_last_link(Some((*href_cloned_for_mouseover).clone()));
                        }
                    }
                >
                    <div
                        class=move || {
                            if last_link_clone.with(|last| last.as_ref() == Some(&*href_cloned_for_style)) {
                                "content active".to_string()
                            } else {
                                "content".to_string()
                            }
                        }
                    >
                        <h2>{category[0]}</h2>  // category[0] is the name
                        <span>"Lorem Ipsum Fixum Later"</span>
                    </div>
                </div>
            }
        })
        .collect();

    view! {
        <div class="portf_container">
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
