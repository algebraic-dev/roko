pub mod file;

use file::http_get;

use roko_dom::elements::*;
use roko_dom::{start, Cmd};
use roko_html::{Attribute, Html};
use roko_macro::html;

use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Msg {
    Increment,
    Decrement,
    ErrorProcessing,
    Loaded(String),
}

type Model = String;

pub fn test(model: &Model, _attrs: Vec<Attribute<Msg>>, _children: Vec<Html<Msg>>) -> Html<Msg> {
    html! {
        <div>
            <button>
                {model.clone()}
            </button>
        </div>
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <button onclick={Msg::Increment} class="be">
                "Increment"
            </button>
            <p>
                {model.clone()}
            </p>
            <test model={model}>

            </test>
            <button onclick={Msg::Decrement}>
                "Decrement"
            </button>
        </div>
    }
}

fn update(msg: Msg, _n: Model) -> Cmd<Model, Msg> {
    match msg {
        Msg::Increment => Cmd::new(
            "loading...".to_string(),
            http_get(
                "https://www.rust-lang.org",
                Msg::ErrorProcessing,
                Msg::Loaded,
            ),
        ),
        Msg::Decrement => Cmd::none("nothing!".to_string()),
        Msg::ErrorProcessing => Cmd::none("error".to_string()),
        Msg::Loaded(s) => Cmd::none(format!("loaded: {s}")),
    }
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let init = Cmd::new(
        "loading...".to_string(),
        http_get(
            "https://www.rust-lang.org",
            Msg::ErrorProcessing,
            Msg::Loaded,
        ),
    );

    start(view, update, init).await
}
