use roko_dom::elements::*;
use roko_html::{Attrs, Children, Html};
use roko_macro::html;

use crate::{Message, Model};

pub fn page(_model: &Model, _attrs: Attrs<Message>, _children: Children<Message>) -> Html<Message> {
    html! {
        <div>
            "resume"
        </div>
    }
}
