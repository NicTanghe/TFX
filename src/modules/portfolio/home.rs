use leptos::*; 
use leptos_router::{
    A,Outlet,NavigateOptions,use_navigate,
};

use crate::modules::blog_posts::blog_compo::Post;


use crate::modules::statics::get_cdn;
/// Renders the home page of your application.

async fn filter_gif(category: &str) -> Option<String> {
    let path = match category {
        "programming" => "/proxy/programming.gif",
        "modelling" => "/proxy/modelling.gif",
        "compositing" => "/proxy/compositing.gif",
        "pipeline" => "/proxy/pipeline.gif",
        "editing" => "/proxy/editing.gif",
        _ => return None, // Unknown category, return None
    };

    match get_cdn(5434, path.to_string()).await {
        Ok(url) => Some(url),
        Err(e) => {
            logging::error!("Error fetching GIF: {:?}", e);
            None
        }
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    let gotolink = use_navigate();

    let (categories, _set_categories) = create_signal(vec![
        "modelling".to_string(),
        "compositing".to_string(),
        "pipeline".to_string(),
        "editing".to_string(),
        "programming".to_string(),
    ]);

    let (last_link, set_last_link) = create_signal(None::<String>);

    let category_resource = create_resource(
        move || categories.get(),
        |categories| async move {
            let mut results = Vec::new();
            for cat in categories {
                let href = format!("/home/{}", cat);
                let gif_url = filter_gif(&cat).await.unwrap_or_default();
                results.push((href, gif_url, cat));
            }
            results
        },
    );

   //eureka happened but i dont understand it. 
    view! {
        
        <div class= "portf_container">
            {move || category_resource.get().map(|precomputed_links| {
                precomputed_links
                    .iter()
                    .map(|(href, gif_url, category)| {
                        let href_signal = create_rw_signal(href.clone());

                        let gotolink = gotolink.clone();
                        let last_link = last_link.clone();
                        let set_last_link = set_last_link.clone();

                        view! {
                            <div 
                                class=move || {
                                    if last_link.get().as_ref() == Some(&href_signal.get()) {
                                        "active".to_string()
                                    } else {
                                        "".to_string()
                                    }
                                }
                                style=format!("background: url({}) no-repeat 100% / cover;", gif_url.clone())
                                on:mouseover=move |_| {
                                    if last_link.get().as_ref() != Some(&href_signal.get()) {
                                        gotolink(&href_signal.get(), NavigateOptions::default());
                                        set_last_link(Some(href_signal.get()));
                                    }
                                }
                            >
                                <div 
                                    class=move || {
                                        if last_link.get().as_ref() == Some(&href_signal.get()) {
                                            "content active".to_string()
                                        } else {
                                            "content".to_string()
                                        }
                                    }
                                > 
                                    <h2>{category.clone()}</h2>
                                    <span>"Lorem Ipsum Fixum Later"</span>
                                </div>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>() // Collect into Vec<View>
            }).unwrap_or_default()}
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
