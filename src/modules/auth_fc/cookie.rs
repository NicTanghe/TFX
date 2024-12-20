use std::fmt::{Display, Formatter};
use http::header::InvalidHeaderValue;
use thiserror::Error;

pub enum CookieKey<'a> {
    Session,
    Other(&'a str),
}

impl<'a> CookieKey<'a> {
    fn as_str(&'a self) -> &'a str {
        match self {
            CookieKey::Session => "session_token",
            CookieKey::Other(key) => *key,
        }
    }
}

impl<'a> Display for CookieKey<'a>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.as_str())
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
    use leptos::use_context;
    use leptos_axum::ResponseOptions;

    fn new_cookie<'a>(key: &'a CookieKey, value: &'a str, duration: std::time::Duration) -> Cookie<'a> {
        let cookie = Cookie::build((key.as_str(), value))
            .same_site(SameSite::Lax)
            .path("/")
            .secure(false)
            .expires(time::OffsetDateTime::now_utc() + duration)
            .build();
        cookie
    }

    pub fn set(
        key: &CookieKey,
        value: &str,
        duration: std::time::Duration,
    ) -> Result<(), SetCookieError> {
        if let Some(response_options) = use_context::<ResponseOptions>() {
            let cookie = new_cookie(key, value, duration);
            response_options.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).map_err(|err|SetCookieError::InvalidHeaderValue(err))?);
            Ok(())
        }
        else {
            Err(SetCookieError::MissingResponseOptions)
        }
    }

    pub fn get<'a>(
        key: &CookieKey<'a>,
        headers: &HeaderMap,
    ) -> Result<Option<String>, GetCookieError> {
        let jar = CookieJar::from_headers(&headers);
        if let Some(cookie) = jar.get(key.as_str()) {
            Ok(Some(cookie.value().to_string()))
        } else {
            Ok(None)
        }
    }

    pub fn remove(key: &CookieKey) -> Result<(), RemoveCookieError> {
        if let Some(response_options) = use_context::<ResponseOptions>() {
            let cookie = new_cookie(key, "", std::time::Duration::from_secs(0));
            response_options.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).map_err(|err|RemoveCookieError::InvalidHeaderValue(err))?);
            Ok(())
        }
        else{
            Err(RemoveCookieError::MissingResponseOptions)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod cookieops {
    use http::HeaderMap;
    use super::{CookieKey, GetCookieError, RemoveCookieError, SetCookieError};
    use wasm_cookies::cookies::CookieOptions;
    use wasm_cookies::cookies::SameSite;

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
        wasm_cookies::get_raw(key.as_str()).ok_or_else(||GetCookieError::CookieNotFound)
    }
    pub fn remove(key: &CookieKey) -> Result<(), RemoveCookieError> {
        wasm_cookies::delete(key.as_str());
        Ok(())
    }
}

