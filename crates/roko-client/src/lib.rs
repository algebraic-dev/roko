#![feature(type_alias_impl_trait)]

use roko_html::Html;
use roko_macro::html;
use roko_render::{start, Cmd};

use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Teste {
    Increment,
    Decrement,
    ErrorProcessing,
    Loaded(String),
}

type Model = String;

fn view(model: &Model) -> Html<Teste> {
    html! {
        <div>
            <button onclick={Teste::Increment}>
                "Increment"
            </button>
            <p>
                {model.clone()}
            </p>
            <button onclick={Teste::Decrement}>
                "Decrement"
            </button>
        </div>
    }
}

async fn read_file() -> Option<Teste> {
    let Ok(result) = reqwest::get("https://www.rust-lang.org").await else {
        return Some(Teste::ErrorProcessing);
    };

    let Ok(text) = result.text().await else {
        return Some(Teste::ErrorProcessing);
    };

    Some(Teste::Loaded(text))
}

async fn none() -> Option<Teste> {
    None
}

fn update(msg: Teste, n: Model) -> (Model, Cmd<Teste>) {
    match msg {
        Teste::Increment => ("oi".to_string(), Box::new(Box::pin(read_file()))),
        Teste::Decrement => ("no".to_string(), Box::new(Box::pin(none()))),
        Teste::ErrorProcessing => ("error".to_string(), Box::new(Box::pin(none()))),
        Teste::Loaded(s) => (format!("loaded: {s}"), Box::new(Box::pin(none()))),
    }
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let future = start(
        view,
        update,
        ("ata".to_string(), Box::new(Box::pin(read_file()))),
    );
    future.await;
    Ok(())
}
