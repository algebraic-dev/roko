//! Module to data structures related to HTML.

use std::{fmt::Debug, sync::Arc};

/// Html attribute in a format that supports a tag without a value e.g `disabled` and with
/// a value e.g `value="Hello World"`.
#[derive(PartialEq, Eq)]
pub enum Attribute<Msg> {
    OnClick(Arc<Msg>),
    Custom(String, String),
}

impl<Msg> std::fmt::Debug for Attribute<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnClick(_) => f.debug_tuple("OnClick").finish(),
            Self::Custom(arg0, arg1) => f.debug_tuple("Custom").field(arg0).field(arg1).finish(),
        }
    }
}

impl<Msg> Clone for Attribute<Msg> {
    fn clone(&self) -> Self {
        match self {
            Self::OnClick(arg0) => Self::OnClick(arg0.clone()),
            Self::Custom(arg0, arg1) => Self::Custom(arg0.clone(), arg1.clone()),
        }
    }
}

/// Html node that contains a tag, attributes and children.
pub struct Node<Msg> {
    pub tag: &'static str,
    pub attributes: Vec<Attribute<Msg>>,
    pub children: Vec<Html<Msg>>,
}

impl<Msg> std::fmt::Debug for Node<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("tag", &self.tag)
            .field("attributes", &self.attributes)
            .field("children", &self.children)
            .finish()
    }
}

impl<Msg> Clone for Node<Msg> {
    fn clone(&self) -> Self {
        Self {
            tag: self.tag,
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

impl<Msg> Html<Msg> {
    pub fn node(
        tag: &'static str,
        attributes: Vec<Attribute<Msg>>,
        children: Vec<Html<Msg>>,
    ) -> Self {
        Self::Node(Node {
            tag,
            attributes,
            children,
        })
    }
}

impl<Msg> Debug for Html<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node(arg0) => f.debug_tuple("Node").field(arg0).finish(),
            Self::Text(arg0) => f.debug_tuple("Text").field(arg0).finish(),
        }
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
