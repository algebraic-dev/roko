use roko_macro::cmd;

#[cmd]
pub async fn http_get<Msg: 'static>(
    url: &'static str,
    error: Msg,
    ok: fn(String) -> Msg,
) -> Option<Msg> {
    let Ok(result) = reqwest::get(url).await else {
        return Some(error);
    };
    let Ok(text) = result.text().await else {
        return Some(error);
    };
    Some(ok(text))
}
