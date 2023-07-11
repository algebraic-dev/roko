use dom::HtmlElement;
use futures::channel::mpsc::UnboundedSender;
use futures::SinkExt;
use roko_html::{Attribute, Html, Node};
use std::sync::Arc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys as dom;

pub fn window() -> dom::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> dom::Document {
    window()
        .document()
        .expect("should have a document on window")
}

/// Trait for rendering a virtual dom to the real dom.
pub trait Render<T> {
    fn render(
        &self,
        container: dom::Element,
        channel: UnboundedSender<Arc<T>>,
    ) -> Option<dom::Element>;
}

impl<Msg: 'static> Render<Msg> for String {
    fn render(
        &self,
        container: dom::Element,
        _: UnboundedSender<Arc<Msg>>,
    ) -> Option<dom::Element> {
        container.set_text_content(Some(self));
        None
    }
}

impl<Msg: 'static + Send + Sync> Render<Msg> for Attribute<Msg> {
    fn render(
        &self,
        container: dom::Element,
        channel: UnboundedSender<Arc<Msg>>,
    ) -> Option<dom::Element> {
        match self {
            Attribute::OnClick(click) => {
                web_sys::console::log_1(&"click!!!".into());
                let click = click.clone();

                let data: Box<dyn FnMut()> = Box::new(move || {
                    let click = click.clone();
                    web_sys::console::log_1(&"click".into());
                    let channel = channel.clone();

                    let click_future = async move { channel.clone().send(click).await };

                    if let Err(err) = futures::executor::block_on(click_future) {
                        web_sys::console::log_1(&format!("error: {:?}", err).into());
                    }

                    web_sys::console::log_1(&"click middle".into());

                    web_sys::console::log_1(&"click end".into());
                });

                let closure = Closure::wrap(data);

                container
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .set_onclick(Some(closure.as_ref().unchecked_ref()));

                closure.forget()
            }
            Attribute::Class(class) => container.set_attribute("class", class).unwrap(),
            Attribute::Style(style) => container.set_attribute("style", style).unwrap(),
        };
        None
    }
}

impl<Msg: 'static + Send + Sync> Render<Msg> for Node<Msg> {
    fn render(&self, _: dom::Element, channel: UnboundedSender<Arc<Msg>>) -> Option<dom::Element> {
        let element = document().create_element(self.tag).unwrap();

        for attribute in &self.attributes {
            attribute.render(element.clone(), channel.clone());
        }

        for child in &self.children {
            if let Some(result) = child.render(element.clone(), channel.clone()) {
                element.append_child(&result).unwrap();
            }
        }

        Some(element)
    }
}

impl<Msg: 'static + Send + Sync> Render<Msg> for Html<Msg> {
    fn render(
        &self,
        container: dom::Element,
        channel: UnboundedSender<Arc<Msg>>,
    ) -> Option<dom::Element> {
        match self {
            Html::Node(node) => node.render(container, channel),
            Html::Text(text) => text.render(container, channel),
        }
    }
}
