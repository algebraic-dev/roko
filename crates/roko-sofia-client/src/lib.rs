#![feature(type_alias_impl_trait)]

pub mod components;
pub mod pages;

use components::*;
use pages::Page;

use roko_dom::elements::*;
use roko_dom::{start, Cmd};
use roko_html::Html;
use roko_macro::html;

use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Message {
    Init,
    ChangePage(Page),
}

pub struct Post {
    pub title: String,
    pub description: String,
    pub link: String,
}

pub struct Model {
    pub page: Page,
    pub posts: Vec<Post>,
}

fn view(model: &Model) -> Html<Message> {
    html! {
        <div>
            <navbar::navbar />
            <pages::page model={model} />
        </div>
    }
}

fn update(msg: Message, model: Model) -> Cmd<Model, Message> {
    match msg {
        Message::Init => Cmd::none(model),
        Message::ChangePage(page) => navbar::update(page, model),
    }
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let var_name = Cmd::none(Model {
        page: Page::Home,
        posts: vec![
            Post {
                title: "Roko".to_string(),
                description: "A compiler for a language that I'm designing.".to_string(),
                link: "".to_string(),
            },
            Post {
                title: "Roko".to_string(),
                description: "A compiler for a language that I'm designing.".to_string(),
                link: "".to_string(),
            },
            Post {
                title: "Roko".to_string(),
                description: "A compiler for a language that I'm designing.".to_string(),
                link: "".to_string(),
            },
        ],
    });

    start(view, update, var_name, None, None).await
}
