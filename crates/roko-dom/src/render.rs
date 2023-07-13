//! This module renders the virtual dom to the real dom. The main structure of this module is the
//! [Render] trait that is implemented for all the types that can be rendered to the real dom.

use roko_html::{Attribute, Html, Node};

use dom::HtmlElement;
use futures::channel::mpsc::UnboundedSender;
use futures::SinkExt;
use std::sync::Arc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys as dom;

fn window() -> dom::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> dom::Document {
    window()
        .document()
        .expect("should have a document on window")
}

pub struct Context<'a, Msg> {
    pub channel: UnboundedSender<Arc<Msg>>,
    pub on_mount: &'a Option<Box<dyn Fn(dom::Element, String)>>,
    pub on_unmount: &'a Option<Box<dyn Fn(dom::Element, String)>>,
}

/// Trait for rendering a virtual dom to the real dom.
pub trait Render<'a, T> {
    fn render(&self, container: dom::Element, ctx: &mut Context<'a, T>) -> Option<dom::Element>;
}

impl<'a, Msg: 'static> Render<'a, Msg> for String {
    fn render(&self, container: dom::Element, _: &mut Context<'a, Msg>) -> Option<dom::Element> {
        container.set_text_content(Some(self));
        None
    }
}

impl<'a, Msg: 'static + Send + Sync> Render<'a, Msg> for Attribute<Msg> {
    fn render(
        &self,
        container: dom::Element,
        context: &mut Context<'a, Msg>,
    ) -> Option<dom::Element> {
        match self {
            Attribute::OnClick(click) => {
                let click = click.clone();

                let channel = context.channel.clone();

                let data: Box<dyn FnMut()> = Box::new(move || {
                    let click = click.clone();

                    let channel = channel.clone();

                    let click_future = async move { channel.clone().send(click).await };

                    futures::executor::block_on(click_future).unwrap()
                });

                let closure = Closure::wrap(data);

                container
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .set_onclick(Some(closure.as_ref().unchecked_ref()));

                closure.forget()
            }
            Attribute::Custom(name, value) => container.set_attribute(name, value).unwrap(),
            Attribute::OnMount(ev) => {
                let ev = ev.clone();
                let channel = context.channel.clone();

                let ev_future = async move { channel.clone().send(ev).await };

                futures::executor::block_on(ev_future).unwrap()
            }
            Attribute::OnUnmount(_) => (),
        };
        None
    }
}

impl<'a, Msg: 'static + Send + Sync> Render<'a, Msg> for Node<Msg> {
    fn render(&self, _: dom::Element, context: &mut Context<'a, Msg>) -> Option<dom::Element> {
        let element = document().create_element(self.tag).unwrap();

        if let Some((on_mount, id)) = context.on_mount.as_ref().zip(self.id.as_ref()) {
            on_mount(element.clone(), id.clone());
        }

        for attribute in &self.attributes {
            attribute.render(element.clone(), context);
        }

        for child in &self.children {
            if let Some(result) = child.render(element.clone(), context) {
                element.append_child(&result).unwrap();
            }
        }

        Some(element)
    }
}

impl<'a, Msg: 'static + Send + Sync> Render<'a, Msg> for Html<Msg> {
    fn render(
        &self,
        container: dom::Element,
        context: &mut Context<'a, Msg>,
    ) -> Option<dom::Element> {
        match self {
            Html::Node(node) => node.render(container, context),
            Html::Text(text) => text.render(container, context),
        }
    }
}
