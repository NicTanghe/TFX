




// this is old code from conversation with lazy who seems to overcomplicate or understands and
// codes things at a higher level then i deem nececairy for now

#[cfg(not(target_arch = "wasm32"))]
pub mod server {
    use axum_extra::extract::cookie::{Cookie, SameSite};

    use http::{header::SET_COOKIE, HeaderValue};

    fn new_cookie<'a>(
        key: &'a str,
        value: &'a str,
        duration: time::Duration,
    ) -> axum_extra::extract::cookie::Cookie<'a> {
        let cookie = Cookie::build((key, value))
            .same_site(SameSite::Lax)
            .path("/")
            .secure(false)
            .expires(time::OffsetDateTime::now_utc() + duration)
            .build();
        cookie
    }
    pub fn set(
        key: &str,
        value: &str,
        duration: time::Duration,
        response: &leptos_axum::ResponseOptions,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        let cookie = new_cookie(key, value, duration);
        response.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }

    pub fn remove(
        key: &str,
        response: &leptos_axum::ResponseOptions,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        let cookie = new_cookie(key, "", time::Duration::seconds(-60 * 24));
        response.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }
}
pub mod wasm {
    use wasm_cookies::cookies::CookieOptions;
    use wasm_cookies::cookies::SameSite;

    #[cfg(target_arch = "wasm32")]
    pub fn set(key: &str, value: &str, duration: std::time::Duration) {
        let options = CookieOptions {
            path: Some("/"),
            domain: None,
            expires: None,
            secure: false,
            same_site: SameSite::Lax,
        }
        .expires_after(duration);
        wasm_cookies::set(key.as_ref(), value, &options);
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set(_key: &str, _value: &str, _duration: std::time::Duration) {}

    #[cfg(target_arch = "wasm32")]
    pub fn remove(key: &str) {
        wasm_cookies::delete(key);
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn remove(_key: &str) {}
}
