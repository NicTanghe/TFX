use crate::prelude::{
    auth_fc::{
        cookie::get_user_details,
        auth_ta::ActiveUser,
    },
    pages::{
        portfolio::landing::*,
        posts::posts_page::{posts_loader, PostsLander},
    },
    widgets::login::UserWidget,
};
// use leptos::prelude::codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes, A},
    hooks::{use_location, use_params_map},
    path,
};
use leptos::task::spawn_local;

use urlencoding::decode;
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

    // Start with empty user
    let (user, set_user) = signal(ActiveUser::default());
    provide_context(user);
    provide_context(set_user);

    // Resource to fetch user details
    let user_resource = Resource::new(
        || (), // no dependency, runs once
        |_| async move {
            get_user_details().await.ok().flatten()
        },
    );

    // Effect to update the signal whenever resource resolves
    Effect::new(move |_| {
        if let Some(Some(active_user)) = user_resource.get() {
            set_user.set(active_user);
        }
    });




    // Signal for hide/show state (like old code)
    let (is_hiding, set_hiding) = signal(2u8);

    view! {
        <Stylesheet id="leptos" href="/pkg/tfx-rewrite.css" />
        <Title text="Welcome to Leptos" />

        <Router>
            <main>
                <NavBar is_hiding=is_hiding />

                <Routes fallback=move || view! { <NotFound /> }>

                    <Route path=path!("/") view=|| view! { <h3>"Home"</h3> } />

                    // -------- Portfolio --------
                    <ParentRoute path=path!("/portfolio") view=PortfolioLander>
                        <ParentRoute path=path!(":id") view=PortfolioInfo>
                            <Route path=path!(":tab") view=PortfolioTab />
                            <Route path=path!("") view=|| view! { <div>"(Portfolio Info)"</div> } />
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
pub fn NavBar(is_hiding: ReadSignal<u8>) -> impl IntoView {
    let location = use_location();
    let user = use_context::<ReadSignal<ActiveUser>>().unwrap();
    let set_user = use_context::<WriteSignal<ActiveUser>>().unwrap();

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
                <div class="nav-left">
                    <A href="/" attr:class="nv">
                        "Home"
                    </A>
                    <A href="/contacts" attr:class=class_for("/contacts")>
                        "Contacts"
                    </A>
                    <A href="/portfolio" attr:class=class_for("/portfolio")>
                        "Portfolio"
                    </A>
                    <A href="/posts" attr:class=class_for("/posts")>
                        "Posts"
                    </A>
                </div>
                <div class="nav-right">
                    <UserWidget user=user set_user=set_user is_hiding=is_hiding />
                </div>
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
