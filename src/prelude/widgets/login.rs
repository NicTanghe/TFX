
use leptos::*;
use leptos::html::Input;
use serde_json;
use std::time::Duration;

use crate::auth::{ActiveUser, get_access_token}; // adjust import paths
use crate::cookie::{self, CookieKey};            // adjust import paths

/// Login form for entering username + password.
#[component]
pub fn LoginForm(set_user: WriteSignal<ActiveUser>) -> impl IntoView {
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let username_ref: NodeRef<Input> = create_node_ref();
    let password_ref: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if let (Some(username_el), Some(password_el)) = (username_ref(), password_ref()) {
            let username_val = username_el.value();
            let password_val = password_el.value();

            set_username(username_val.clone());
            set_password(password_val.clone());

            leptos::spawn_local(async move {
                let mut updated_user = ActiveUser::default();
                updated_user.name = username_val.clone();

                match get_access_token(username_val.clone(), password_val.clone()).await {
                    Ok(token) => {
                        updated_user.token = token;
                    }
                    Err(err) => {
                        updated_user.token = "Error".to_string();
                        logging::log!("Login failed: {:?}", err);
                    }
                }

                set_user.update(|user| {
                    *user = updated_user.clone();
                });

                // Set cookie depending on success
                let cookie_duration = if updated_user.token == "Error" {
                    Duration::new(2, 0)
                } else {
                    Duration::new(324_890, 0)
                };

                cookie::cookieops::set(
                    &CookieKey::Other("user"),
                    &serde_json::to_string(&updated_user).expect("serialize user"),
                    cookie_duration,
                );
            });
        }
    };

    view! {
        <form on:submit=on_submit class="login-form">
            <input type="text" placeholder="Username" value=username node_ref=username_ref />
            <input type="password" placeholder="Password" value=password node_ref=password_ref />
            <button type="submit">Sign in</button>
        </form>
    }
}

/// Widget for showing user info, logout, or login form.
#[component]
pub fn UserWidget(
    user: ReadSignal<ActiveUser>,
    set_user: WriteSignal<ActiveUser>,
    is_hiding: ReadSignal<u8>,
) -> impl IntoView {
    let hide_class = move || match is_hiding.get() {
        0 => "hover-element from-hiding",
        1 => "hover-element showing",
        2 => "hover-element to-hiding",
        _ => "",
    };

    let logout_action = move || {
        set_user.update(|user| {
            user.name.clear();
            user.token.clear();
            user.roles.clear();

            cookie::cookieops::set(
                &CookieKey::Other("user"),
                &serde_json::to_string(&user).expect("serialize user"),
                Duration::new(0, 0),
            );
        });
    };

    view! {
        <div class=hide_class style="position: absolute; top: 10px; right: 10px;">
            {move || {
                let user_state = user.get();
                if user_state.token == "Error" {

                    view! {
                        <>
                            <span>
                                {format!("Wrong username or password: {}", user_state.name)}
                            </span>
                            <button on:click=move |_| logout_action()>"Try Again"</button>
                        </>
                    }
                        .into_any()
                } else if !user_state.name.is_empty() {
                    view! {
                        <>
                            <span>{format!("Welcome back, {}", user_state.name)}</span>
                            <button on:click=move |_| logout_action()>"Log Out"</button>
                        </>
                    }
                        .into_any()
                } else {
                    view! { <LoginForm set_user=set_user /> }.into_any()
                }
            }}
        </div>
    }
}
