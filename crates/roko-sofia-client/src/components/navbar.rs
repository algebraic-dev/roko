//! Component for the navigation bar.

use roko_dom::{elements::*, Cmd};
use roko_html::{Attrs, Children, Html};
use roko_macro::html;

use crate::pages::{self, Page};
use crate::{Message, Model};

pub fn navbar(_: Option<String>, _: Attrs<Message>, _: Children<Message>) -> Html<Message> {
    let pages: Vec<_> = pages::PAGES.iter().map(menu_item).collect();

    html! {
        <div class="menu">
            <ul class="menu-inner" children={pages} />
        </div>
    }
}

fn menu_item(x: &Page) -> Html<Message> {
    html! {
        <li class="menu-text" onclick={Message::ChangePage(*x)}>
            {x.to_string()}
        </li>
    }
}

pub fn update(page: Page, model: Model) -> Cmd<Model, Message> {
    Cmd::none(Model { page, ..model })
}
