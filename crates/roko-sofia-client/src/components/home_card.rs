use roko_html::{to_map, Attrs, Children, Html};
use roko_macro::html;

use roko_dom::elements::*;

use crate::Message;

pub fn card(_: Option<String>, attrs: Attrs<Message>, _: Children<Message>) -> Html<Message> {
    let attrs = to_map(attrs);

    let empty = "".to_string();

    let title = attrs.get("title").unwrap_or(&empty);
    let description = attrs.get("description").unwrap_or(&empty);
    let link = attrs.get("link").unwrap_or(&empty);

    html! {
        <a class="home-card">
            <h2>{title.clone()}</h2>
            <p>{description.clone()}</p>
            <a href={link.clone()} class="button">{"Read More"}</a>
        </a>
    }
}
