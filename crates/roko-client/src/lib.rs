#![feature(type_alias_impl_trait)]

use std::pin::{pin, Pin};

use futures::Future;
use roko_html::Html;
use roko_macro::html;
use roko_render::{start, Response};

use wasm_bindgen::prelude::*;

async fn read_file_future() -> Option<Message> {
    let Ok(result) = reqwest::get("https://www.rust-lang.org").await else {
        return Some(Message::ErrorProcessing);
    };
    let Ok(text) = result.text().await else {
        return Some(Message::ErrorProcessing);
    };
    Some(Message::Loaded(text))
}

fn read_file() -> Box<dyn Future<Output = Option<Message>> + Unpin> {
    Box::new(Box::pin(read_file_future()))
}

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
            <button onclick={Message::Increment}>
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

fn update(msg: Message, _n: Model) -> Response<Model, Message> {
    match msg {
        Message::Increment => Response::new("loading...".to_string(), read_file()),
        Message::Decrement => Response::none("nothing!".to_string()),
        Message::ErrorProcessing => Response::none("error".to_string()),
        Message::Loaded(s) => Response::none(format!("loaded: {s}")),
    }
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let var_name = Response::none("init".to_string());
    let future = start(view, update, var_name);

    future.await;
    Ok(())
}
