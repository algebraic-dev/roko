use reqwest::{Body, IntoUrl};
use roko_macro::cmd;

/// A GET request with a URL.
#[cmd]
async fn get<Msg: 'static>(
    url: impl IntoUrl + 'static,
    ok: fn(String) -> Msg,
    failure: Msg,
) -> Option<Msg> {
    let Ok(result) = reqwest::get(url).await else {
        return Some(failure);
    };

    let Ok(text) = result.text().await else {
        return Some(failure);
    };

    Some(ok(text))
}

/// A POST request with a URL and a body.
#[cmd]
pub async fn post<Msg: 'static>(
    url: impl IntoUrl + 'static,
    body: impl Into<Body> + 'static,
    ok: fn(String) -> Msg,
    failure: Msg,
) -> Option<Msg> {
    let Ok(result) = reqwest::Client::new().post(url).body(body).send().await else {
        return Some(failure);
    };

    let Ok(text) = result.text().await else {
        return Some(failure);
    };

    Some(ok(text))
}
