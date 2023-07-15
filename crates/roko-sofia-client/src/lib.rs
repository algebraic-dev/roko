#![feature(type_alias_impl_trait)]

pub mod components;
pub mod pages;

use components::*;

use pages::Page;

use roko_dom::events::{change_hash, change_title, get_page_hash, on_hash_change};
use roko_dom::{elements::*, Channel};
use roko_dom::{start, Cmd};
use roko_html::Html;
use roko_macro::html;

use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Message {
    Init,
    ChangePage(Page),
    HashChange(String),
}

pub struct Project {
    pub title: String,
    pub description: String,
    pub link: String,
}

pub struct Model {
    pub page: Page,
    pub projects: Vec<Project>,
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
        // System Events
        Message::HashChange(hash) => {
            Cmd::message(model, Message::ChangePage(Page::from_hash(&hash)))
        }

        // User events
        Message::Init => {
            let page = model.page;
            Cmd::message(model, Message::ChangePage(page))
        }

        Message::ChangePage(page) => {
            change_hash(&page.to_hash());
            change_title(&page.to_title());
            navbar::update(page, model)
        }
    }
}

fn subscriptions(channel: Channel<Message>) {
    on_hash_change(channel, Message::HashChange)
}

fn init() -> Cmd<Model, Message> {
    Cmd::message(
        Model {
            page: Page::from_hash(&get_page_hash()),
            projects: vec![
                Project {
                    title: "Vulpi".to_string(),
                    description: "A compiler written in Rust for the backend of this site :)"
                        .to_string(),
                    link: "https://github.com/vulpi-lang/vulpi".to_string(),
                },
                Project {
                    title: "Nuko".to_string(),
                    description:
                        "A compiler written in Haskell for a language with Higher rank polymorphism"
                            .to_string(),
                    link: "https://github.com/algebraic-sofia/nuko".to_string(),
                },
                Project {
                    title: "Roko".to_string(),
                    description: "A simple elm-like virtual dom for this website!".to_string(),
                    link: "https://github.com/algebraic-sofia/roko".to_string(),
                },
            ],
        },
        Message::Init,
    )
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    start(view, update, init(), subscriptions, None, None).await
}
