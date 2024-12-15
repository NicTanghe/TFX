
#[cfg(feature = "ssr")]
#[tokio::main]
//ok u need to fix the linter for one but also like check the axum template and make sure all is well
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos::prelude;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use home_portal::app::*;
    use home_portal::fileserv::file_and_error_handler;
    use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

    use tracing_subscriber;

    use tracing::debug;


  // Set up CORS layer


    
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    debug!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}


