pub mod file;

use file::http_get;

use roko_dom::elements::*;
use roko_dom::{start, Cmd};
use roko_html::Html;
use roko_macro::html;

use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    ErrorProcessing,
    Loaded(String),
}

type Model = String;

fn view(model: &Model) -> Html<Message> {
    html! {
        <div>
            <button onclick={Message::Increment} class="be">
                "Increment"
            </button>
            <p>
                {model.clone()}
            </p>
            <button onclick={Message::Decrement}>
                "Decrement"
            </button>
        </div>
    }
}

fn update(msg: Message, _n: Model) -> Cmd<Model, Message> {
    match msg {
        Message::Increment => Cmd::new(
            "loading...".to_string(),
            http_get(
                "https://www.rust-lang.org",
                Message::ErrorProcessing,
                Message::Loaded,
            ),
        ),
        Message::Decrement => Cmd::none("nothing!".to_string()),
        Message::ErrorProcessing => Cmd::none("error".to_string()),
        Message::Loaded(s) => Cmd::none(format!("loaded: {s}")),
    }
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let init = Cmd::new(
        "loading...".to_string(),
        http_get(
            "https://www.rust-lang.org",
            Message::ErrorProcessing,
            Message::Loaded,
        ),
    );

    start(view, update, init).await
}
