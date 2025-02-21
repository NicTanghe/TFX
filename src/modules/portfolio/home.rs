use leptos::*; 
use leptos_router::{
    A,Outlet,NavigateOptions,use_navigate,
};

use crate::modules::blog_posts::blog_compo::Post;


use crate::modules::statics::get_cdn;
#[component]
pub fn HomePage() -> impl IntoView {
    // Define categories as a Vec<String>
    let gotolink = use_navigate();

    let categories = vec![
        "modelling".to_string(),
        "compositing".to_string(),
        "pipeline".to_string(),
        "editing".to_string(),
        "programming".to_string(),
    ];
    
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
            let gif_url = filter_gif(&cat);
            (href, gif_url, cat.clone(), cat)
        })
        .collect();

    // Signal to store the last visited link
    let (last_link, set_last_link) = create_signal(None::<String>);
    
    let precomputed_links: Vec<_> = category_links
        .into_iter()
        .map(|(href, gif_url, cat_for_a, cat_for_img)| {
            let (href_signal, _href_signal_set) = create_signal(href.clone());
            let cat_for_a_cloned = cat_for_a.clone();
            let _cat_for_img_cloned = cat_for_img.clone();
            let gotolink_cloned = gotolink.clone();
            let last_link_clone = last_link.clone();
            
            view! {
                <div 
                    class=move || {
                        if last_link_clone.with(|last| last.as_ref() == Some(&href.clone())) {
                            "active".to_string()
                        } else {
                            "".to_string()
                        }
                    }
                    style=format!("background: url({}) no-repeat 100% / cover;", gif_url.clone().unwrap_or_default())
                    on:mouseover=move |_| {
                        if last_link.with(|last| last.as_ref() != Some(&href_signal.get())) {
                            gotolink_cloned(&href_signal.get(), NavigateOptions::default());
                            set_last_link(Some(href_signal.get()));
                        }
                    }
                >
                    <div 
                        class=move || {
                            if last_link_clone.with(|last| last.as_ref() == Some(&href_signal.get())) {
                                "content active".to_string()
                            } else {
                                "content".to_string()
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
        <div class="portf_container">
            {precomputed_links}
        </div>
        <Outlet/>
    }
}
