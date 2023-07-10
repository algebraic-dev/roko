use roko_html::{Html, Node};
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
pub trait Render {
    fn render(&self, container: dom::Element) -> Option<dom::Element>;
}

impl Render for String {
    fn render(&self, container: dom::Element) -> Option<dom::Element> {
        container.set_text_content(Some(self));
        None
    }
}

impl<Msg> Render for Node<Msg> {
    fn render(&self, container: dom::Element) -> Option<dom::Element> {
        let element = document().create_element(self.tag).unwrap();

        for attribute in &self.attributes {
            todo!()
        }

        for child in &self.children {
            child.render(element.clone());
            element.append_child(&element).unwrap();
        }

        Some(element)
    }
}

impl<Msg> Render for Html<Msg> {
    fn render(&self, container: dom::Element) -> Option<dom::Element> {
        match self {
            Html::Node(node) => node.render(container),
            Html::Text(text) => text.render(container),
        }
    }
}
