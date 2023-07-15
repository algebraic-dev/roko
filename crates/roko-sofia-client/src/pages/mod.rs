//! Module for pages of the website. This module contains the data structures that represent the
//! pages of the website and a way to control them using a [Page] enum.

use std::fmt::Display;

use roko_html::{Attrs, Children, Html};

use crate::{Message, Model};

pub mod blog;
pub mod home;
pub mod projects;
pub mod resume;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Home,
    Blog,
    Projects,
    Resume,
}

impl Page {
    pub fn from_hash(hash: &str) -> Self {
        match hash {
            "" | "#" => Self::Home,
            "#blog" => Self::Blog,
            "#projects" => Self::Projects,
            "#resume" => Self::Resume,
            _ => Self::Home,
        }
    }

    pub fn to_hash(self) -> String {
        match self {
            Self::Home => String::from("#"),
            Self::Blog => String::from("#blog"),
            Self::Projects => String::from("#projects"),
            Self::Resume => String::from("#resume"),
        }
    }

    pub fn to_title(self) -> String {
        match self {
            Self::Home => String::from("Home | Sofia"),
            Self::Blog => String::from("Blog | Sofia"),
            Self::Projects => String::from("Projects | Sofia"),
            Self::Resume => String::from("Resume | Sofia"),
        }
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Home => write!(f, "home"),
            Page::Blog => write!(f, "blog"),
            Page::Projects => write!(f, "projects"),
            Page::Resume => write!(f, "resume"),
        }
    }
}

pub const PAGES: &[Page] = &[Page::Home, Page::Blog, Page::Projects, Page::Resume];

pub fn page(
    model: &Model,
    _id: Option<String>,
    attributes: Attrs<Message>,
    children: Children<Message>,
) -> Html<Message> {
    match model.page {
        Page::Home => home::page(model, attributes, children),
        Page::Blog => blog::page(model, attributes, children),
        Page::Projects => projects::page(model, attributes, children),
        Page::Resume => resume::page(model, attributes, children),
    }
}
