//! Module to data structures related to HTML.

use std::{collections::HashMap, fmt::Debug, sync::Arc};

pub type Attrs<Message> = Vec<Attribute<Message>>;

pub type Children<Message> = Vec<Html<Message>>;

/// Html attribute in a format that supports a tag without a value e.g `disabled` and with
/// a value e.g `value="Hello World"`.
#[derive(PartialEq, Eq)]
pub enum Attribute<Msg> {
    OnClick(Arc<Msg>),
    OnMount(Arc<Msg>),
    OnUnmount(Arc<Msg>),
    Custom(String, String),
}

impl<Msg> Debug for Attribute<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnClick(_) => f.debug_tuple("OnClick").finish(),
            Self::OnMount(_) => f.debug_tuple("OnMount").finish(),
            Self::OnUnmount(_) => f.debug_tuple("OnUnmount").finish(),
            Self::Custom(arg0, arg1) => f.debug_tuple("Custom").field(arg0).field(arg1).finish(),
        }
    }
}

impl<Msg> Clone for Attribute<Msg> {
    fn clone(&self) -> Self {
        match self {
            Self::OnClick(arg0) => Self::OnClick(arg0.clone()),
            Self::OnMount(arg0) => Self::OnMount(arg0.clone()),
            Self::OnUnmount(arg0) => Self::OnUnmount(arg0.clone()),
            Self::Custom(arg0, arg1) => Self::Custom(arg0.clone(), arg1.clone()),
        }
    }
}

pub fn to_map<Msg>(attrs: Attrs<Msg>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for attr in attrs {
        if let Attribute::Custom(key, value) = attr {
            map.insert(key, value);
        }
    }

    map
}

/// Html node that contains a tag, attributes and children.
pub struct Node<Msg> {
    pub tag: &'static str,
    pub id: Option<String>,
    pub attributes: Vec<Attribute<Msg>>,
    pub children: Vec<Html<Msg>>,
}

impl<Msg> Debug for Node<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("tag", &self.tag)
            .field("id", &self.id)
            .field("attributes", &self.attributes)
            .field("children", &self.children)
            .finish()
    }
}

impl<Msg> Clone for Node<Msg> {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag,
            id: self.id.clone(),
            attributes: self.attributes.clone(),
            children: self.children.clone(),
        }
    }
}

/// Html data structure that can be either a node or a text. This is the main data structure that
/// is used to build a virtual dom.
pub enum Html<Msg> {
    Node(Node<Msg>),
    Text(String),
}

impl<Msg> Debug for Html<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node(arg0) => f.debug_tuple("Node").field(arg0).finish(),
            Self::Text(arg0) => f.debug_tuple("Text").field(arg0).finish(),
        }
    }
}

impl<Msg> Html<Msg> {
    pub fn node(
        tag: &'static str,
        id: Option<String>,
        attributes: Vec<Attribute<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Self::Node(Node {
            tag,
            id,
            attributes,
            children,
        })
    }
}

impl<Msg> Clone for Html<Msg> {
    fn clone(&self) -> Self {
        match self {
            Self::Node(arg0) => Self::Node(arg0.clone()),
            Self::Text(arg0) => Self::Text(arg0.clone()),
        }
    }
}

/// Creates a new node.

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
