/// This module contains functions for creating HTML elements. These functions are used by the
/// `roko_macro` crate to create the HTML elements that are rendered to the DOM.
use roko_html::{Attribute, Html};

pub fn p<Msg>(
    id: Option<String>,
    attrs: Vec<Attribute<Msg>>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::node("p", id, attrs, children)
}

pub fn div<Msg>(
    id: Option<String>,
    attrs: Vec<Attribute<Msg>>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::node("div", id, attrs, children)
}

pub fn button<Msg>(
    id: Option<String>,
    attrs: Vec<Attribute<Msg>>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::node("button", id, attrs, children)
}
