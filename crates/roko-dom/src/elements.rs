/// This module contains functions for creating HTML elements. These functions are used by the
/// `roko_macro` crate to create the HTML elements that are rendered to the DOM.
use roko_html::{Attribute, Html};

pub fn p<Msg>(attrs: Vec<Attribute<Msg>>, children: Vec<Html<Msg>>) -> Html<Msg> {
    Html::node("p", attrs, children)
}

pub fn div<Msg>(attrs: Vec<Attribute<Msg>>, children: Vec<Html<Msg>>) -> Html<Msg> {
    Html::node("div", attrs, children)
}

pub fn button<Msg>(attrs: Vec<Attribute<Msg>>, children: Vec<Html<Msg>>) -> Html<Msg> {
    Html::node("button", attrs, children)
}
