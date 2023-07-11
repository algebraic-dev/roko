use roko_macro::html;
use roko_render::app::start;

use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Debug)]
pub enum Teste {
    A,
    B,
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    start(
        |n| {
            let a = "background:#00f".to_string();

            html! {
                <p class="ata" style={a} onclick={Teste::A}>
                    {*n}
                </p>
            }
        },
        |_, b| *b += 1,
        0,
    )
    .await;

    Ok(())
}
