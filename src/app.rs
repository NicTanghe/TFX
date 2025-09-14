use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes, A},
    hooks::{use_location, use_params_map},
    path,
};

use crate::prelude::pages::{
    portfolio::landing::*,
    posts::posts_page::{posts_loader, PostsLander},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tfx-rewrite.css" />
        <Title text="Welcome to Leptos" />

        <Router>
            <main>
                <NavBar />

                <Routes fallback=move || view! { <NotFound /> }>

                    <Route path=path!("/") view=|| view! { <h3>"Home"</h3> } />

                    // -------- Portfolio --------
                    <ParentRoute path=path!("/portfolio") view=PortfolioLander>
                        <ParentRoute path=path!(":id") view=PortfolioInfo>
                            <Route path=path!("2d") view=PortfolioTab />
                            <Route path=path!("3d") view=PortfolioTab />
                        </ParentRoute>
                        <Route
                            path=path!("")
                            view=|| {
                                view! {
                                    <div class="select-user">
                                        "Select category for your viewing pleasure."
                                    </div>
                                }
                            }
                        />
                    </ParentRoute>

                    // -------- Contacts --------
                    <ParentRoute path=path!("/contacts") view=ContactList>
                        <ParentRoute path=path!(":id") view=ContactInfo>
                            <Route
                                path=path!("")
                                view=|| view! { <div class="tab">"(Contact Info)"</div> }
                            />
                            <Route
                                path=path!("conversations")
                                view=|| view! { <div class="tab">"(Conversations)"</div> }
                            />
                        </ParentRoute>
                        <Route
                            path=path!("")
                            view=|| {
                                view! {
                                    <div class="select-user">
                                        "Select a user to view contact info."
                                    </div>
                                }
                            }
                        />
                    </ParentRoute>

                    // -------- Posts --------
                    <ParentRoute path=path!("/posts") view=PostsLander>
                        <Route path=path!("") view=posts_loader />
                    </ParentRoute>

                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let location = use_location();

    // Returns a reactive class string depending on current path
    let class_for = |base: &'static str| {
        move || {
            if location.pathname.get().starts_with(base) {
                "nv-active"
            } else {
                "nv"
            }
        }
    };

    view! {
        <div class="navbar-container">
            <nav class="navbar">
                <a href="/" class="nv">
                    "Home"
                </a>
                <a href="/contacts" class=class_for("/contacts")>
                    "Contacts"
                </a>
                <a href="/portfolio" class=class_for("/portfolio")>
                    "Portfolio"
                </a>
                <a href="/posts" class=class_for("/posts")>
                    "Posts"
                </a>
            </nav>
        </div>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
            // here's our contact list component itself
            <h3>"Contacts"</h3>
            <div class="contact-list-contacts">
                <A href="alice">"Alice"</A>
                <A href="bob">"Bob"</A>
                <A href="steve">"Steve"</A>
            </div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet />
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>
                    "Contact Info"
                </A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet />
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // only works during server-side rendering
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_axum::ResponseOptions>();
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"404 Page Not Found"</h1> }
}
