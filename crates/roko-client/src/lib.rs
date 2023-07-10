use roko_html::Html;
use roko_macro::html;

use roko_render::render::Render;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let a = "background:#ff0".to_string();

    let result: Html<u8> = html! {
        <p class="ata" onclick={2}>
            2
        </p>
    };

    result.render(body.into(), document);

    Ok(())
}
