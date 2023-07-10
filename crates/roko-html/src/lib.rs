//! Module to data structures related to HTML.

/// Html attribute in a format that supports a tag without a value e.g `disabled` and with
/// a value e.g `value="Hello World"`.
#[derive(PartialEq, Eq)]
pub enum Attribute<Msg> {
    OnClick(Msg),
    Class(String),
}

/// Html node that contains a tag, attributes and children.
pub struct Node<Msg> {
    pub tag: &'static str,
    pub attributes: Vec<Attribute<Msg>>,
    pub children: Vec<Html<Msg>>,
}

/// Html data structure that can be either a node or a text. This is the main data structure that
/// is used to build a virtual dom.
pub enum Html<Msg> {
    Node(Node<Msg>),
    Text(String),
}

/// Creates a new node.

pub fn node<Msg>(
    tag: &'static str,
    attributes: Vec<Attribute<Msg>>,
    children: Vec<Html<Msg>>,
) -> Html<Msg> {
    Html::Node(Node {
        tag,
        attributes,
        children,
    })
}

pub fn text<Msg, T: Into<Html<Msg>>>(text: T) -> Html<Msg> {
    text.into()
}

impl<Msg> From<usize> for Html<Msg> {
    fn from(val: usize) -> Self {
        Html::Text(val.to_string())
    }
}

impl<Msg> From<&str> for Html<Msg> {
    fn from(val: &str) -> Self {
        Html::Text(val.to_string())
    }
}

impl<Msg> From<String> for Html<Msg> {
    fn from(val: String) -> Self {
        Html::Text(val)
    }
}
