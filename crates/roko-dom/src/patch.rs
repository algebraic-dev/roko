//! Module for patching the DOM with the [Patch] type that express the difference between the last
//! evaluation of the virtual dom and the current one.

use roko_html::{Attribute, Html};

use dom::{HtmlCollection, HtmlElement};
use futures::SinkExt;
use std::fmt::Debug;

use wasm_bindgen::JsCast;
use web_sys as dom;

use crate::render::{Context, Render};

/// Patch for attributes
pub enum AttrPatch<Msg> {
    Add(Attribute<Msg>),
    Remove(Attribute<Msg>),
}

impl<Msg> Debug for AttrPatch<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(arg0) => f.debug_tuple("Add").field(arg0).finish(),
            Self::Remove(arg0) => f.debug_tuple("Remove").field(arg0).finish(),
        }
    }
}

/// The patch type that express the difference between the last evaluation of the virtual dom and
/// the current one.
pub enum Patch<Msg> {
    Add(Html<Msg>),
    Replace(Html<Msg>),
    Update(Vec<Patch<Msg>>, Vec<AttrPatch<Msg>>),
    Remove(Option<String>),
    Nothing,
}

impl<Msg> Patch<Msg> {
    pub fn is_nothing(&self) -> bool {
        matches!(self, Self::Nothing)
    }
}

impl<Msg> Debug for Patch<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(arg0) => f.debug_tuple("Add").field(arg0).finish(),
            Self::Replace(arg0) => f.debug_tuple("Replace").field(arg0).finish(),
            Self::Update(arg0, arg1) => f.debug_tuple("Update").field(arg0).field(arg1).finish(),
            Self::Remove(arg0) => f.debug_tuple("Remove").field(arg0).finish(),
            Self::Nothing => write!(f, "Nothing"),
        }
    }
}

/// Applies a sequence of pathes for children.
fn apply_children<Msg: 'static + Send + Sync>(
    parent: dom::Element,
    children: HtmlCollection,
    patches: Vec<Patch<Msg>>,
    context: &mut Context<'_, Msg>,
) {
    for (i, patch) in patches.into_iter().enumerate() {
        if let Some(child) = children.get_with_index(i as u32) {
            patch.apply(child, context);
        } else {
            patch.apply(parent.clone(), context);
        }
    }
}

/// Applies a sequence of patches for a sequence of attributes.
fn apply_attributes<Msg: 'static + Send + Sync>(
    el: dom::Element,
    patches: Vec<AttrPatch<Msg>>,
    context: &mut Context<'_, Msg>,
) {
    for patch in patches {
        match patch {
            AttrPatch::Add(add) => {
                add.render(el.clone(), context);
            }
            AttrPatch::Remove(rem) => match rem {
                Attribute::OnClick(_) => el.dyn_ref::<HtmlElement>().unwrap().set_onclick(None),
                Attribute::Custom(n, _) => el.set_attribute(&n, "").unwrap(),
                Attribute::OnMount(_) => (),
                Attribute::OnUnmount(ev) => {
                    let ev = ev.clone();
                    let context = context.channel.clone();

                    let ev_future = async move { context.clone().send(ev).await };

                    futures::executor::block_on(ev_future).unwrap();
                }
            },
        }
    }
}

impl<'a, Msg: 'static + Send + Sync> Patch<Msg> {
    /// This function applies a patch to the real dom.
    pub fn apply(self, el: dom::Element, context: &mut Context<'a, Msg>) {
        match self {
            Patch::Add(add) => {
                if let Some(new_el) = add.render(el.clone(), context) {
                    el.append_child(&new_el).unwrap();
                }
            }
            Patch::Replace(replace) => {
                if let Some(new_el) = replace.render(el.clone(), context) {
                    el.replace_with_with_node_1(&new_el).unwrap();
                }
            }
            Patch::Update(children, attr) => {
                apply_children(el.clone(), el.children(), children, context);
                apply_attributes(el, attr, context);
            }
            Patch::Remove(key) => {
                if let Some((on_mount, key)) = context.on_mount.as_ref().zip(key.as_ref()) {
                    on_mount(el.clone(), key.clone());
                }
                el.remove()
            }
            Patch::Nothing => (),
        }
    }
}
