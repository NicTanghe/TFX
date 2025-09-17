use axum_extra::extract::cookie::CookieJar;
use http::header::InvalidHeaderValue;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use leptos::server;

use crate::prelude::auth_fc::auth_ta::ActiveUser;
pub enum CookieKey<'a> {
    Session,
    Other(&'a str),
}

use leptos::prelude::ServerFnError;

impl<'a> CookieKey<'a> {
    fn as_str(&'a self) -> &'a str {
        match self {
            CookieKey::Session => "session_token",
            CookieKey::Other(key) => *key,
        }
    }
}

impl<'a> Display for CookieKey<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Error, Debug)]
pub enum SetCookieError {
    #[error("Missing ResponseOptions in context")]
    MissingResponseOptions,
    #[error("Invalid header value")]
    InvalidHeaderValue(InvalidHeaderValue),
}

#[derive(Error, Debug)]
pub enum RemoveCookieError {
    #[error("Missing ResponseOptions in context")]
    MissingResponseOptions,
    #[error("Invalid header value")]
    InvalidHeaderValue(InvalidHeaderValue),
}

#[derive(Error, Debug)]
pub enum GetCookieError {
    #[error("Missing HeaderMap in context")]
    MissingHeaderMap,
    #[error("Cookie not found")]
    CookieNotFound,
}

#[cfg(not(target_arch = "wasm32"))]
pub mod cookieops {
    use super::{CookieKey, GetCookieError, RemoveCookieError, SetCookieError};
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use axum_extra::extract::CookieJar;
    use http::{header::SET_COOKIE, HeaderMap, HeaderValue};
    use leptos::prelude::use_context;
    use leptos_axum::ResponseOptions;

    fn new_cookie<'a>(
        key: &'a CookieKey,
        value: &'a str,
        duration: std::time::Duration,
    ) -> Cookie<'a> {
        Cookie::build((key.as_str(), value))
            .same_site(SameSite::Lax)
            .path("/")
            .secure(false)
            .expires(time::OffsetDateTime::now_utc() + duration)
            .build()
    }

    pub fn set(
        key: &CookieKey,
        value: &str,
        duration: std::time::Duration,
    ) -> Result<(), SetCookieError> {
        if let Some(response_options) = use_context::<ResponseOptions>() {
            let cookie = new_cookie(key, value, duration);
            response_options.append_header(
                SET_COOKIE,
                HeaderValue::from_str(&cookie.to_string())
                    .map_err(SetCookieError::InvalidHeaderValue)?,
            );
            Ok(())
        } else {
            Err(SetCookieError::MissingResponseOptions)
        }
    }

    pub fn get<'a>(
        key: &CookieKey<'a>,
        headers: &HeaderMap,
    ) -> Result<Option<String>, GetCookieError> {
        let jar = CookieJar::from_headers(headers);
        Ok(jar.get(key.as_str()).map(|c| c.value().to_string()))
    }

    pub fn remove(key: &CookieKey) -> Result<(), RemoveCookieError> {
        if let Some(response_options) = use_context::<ResponseOptions>() {
            let cookie = new_cookie(key, "", std::time::Duration::from_secs(0));
            response_options.append_header(
                SET_COOKIE,
                HeaderValue::from_str(&cookie.to_string())
                    .map_err(RemoveCookieError::InvalidHeaderValue)?,
            );
            Ok(())
        } else {
            Err(RemoveCookieError::MissingResponseOptions)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod cookieops {
    use super::{CookieKey, GetCookieError, RemoveCookieError, SetCookieError};
    use wasm_cookies::cookies::{CookieOptions, SameSite};

    pub fn set(
        key: &CookieKey,
        value: &str,
        duration: std::time::Duration,
    ) -> Result<(), SetCookieError> {
        let options = CookieOptions {
            path: Some("/"),
            domain: None,
            expires: None,
            secure: false,
            same_site: SameSite::Lax,
        }
        .expires_after(duration.into());
        wasm_cookies::set(key.as_str(), value, &options);
        Ok(())
    }

    pub fn get(key: &CookieKey) -> Result<String, GetCookieError> {
        wasm_cookies::get_raw(key.as_str()).ok_or(GetCookieError::CookieNotFound)
    }

    pub fn remove(key: &CookieKey) -> Result<(), RemoveCookieError> {
        wasm_cookies::delete(key.as_str());
        Ok(())
    }
}

pub fn extract_token_from_cookies(jar: &CookieJar) -> Option<String> {
    jar.get("token").map(|c| c.value().to_string())
}

#[server(GetActiveUser, "/ActiveUser")]
pub async fn get_user_details() -> Result<Option<ActiveUser>, ServerFnError> {
    use  leptos_axum::extract;
    // Retrieve the stored JSON string from the cookie
    let headers: http::HeaderMap = extract().await?;
    
    // Define the cookie key
    let cookie_key = CookieKey::Other("user");

    // Check for user cookie using the key
    if let Ok(Some(user_str)) = cookieops::get(&cookie_key, &headers) {
        // Deserialize and return the ActiveUser
        if let Ok(loaded_user) = serde_json::from_str::<ActiveUser>(&user_str) {
            return Ok(Some(loaded_user));
        }
    }

    // Default return if no cookie or deserialization fails
    Ok(None)
}
