use std::{rc::Rc, sync::Arc};

use dom::{HtmlCollection, HtmlElement};
use futures::channel::mpsc::UnboundedSender;
use roko_html::{Attribute, Html};
use wasm_bindgen::JsCast;
use web_sys as dom;

use crate::{
    app,
    render::{self, Render},
};

#[derive(Debug)]
pub enum AttrPatch<Msg> {
    Add(Attribute<Msg>),
    Remove(Attribute<Msg>),
}

#[derive(Debug)]
pub enum Patch<Msg> {
    Add(Html<Msg>),
    Replace(Html<Msg>),
    Update(Vec<Patch<Msg>>, Vec<AttrPatch<Msg>>),
    Remove,
    Nothing,
}

pub fn apply_children<Msg: 'static + Send + Sync>(
    parent: dom::Element,
    children: HtmlCollection,
    patches: Vec<Patch<Msg>>,
    channel: UnboundedSender<Arc<Msg>>,
) {
    for (i, patch) in patches.into_iter().enumerate() {
        if let Some(child) = children.get_with_index(i as u32) {
            patch.apply(child, channel.clone());
        } else {
            patch.apply(parent.clone(), channel.clone());
        }
    }
}

pub fn apply_attributes<Msg: 'static + Send + Sync>(
    el: dom::Element,
    patches: Vec<AttrPatch<Msg>>,
    channel: UnboundedSender<Arc<Msg>>,
) {
    for patch in patches {
        match patch {
            AttrPatch::Add(add) => {
                add.render(el.clone(), channel.clone());
            }
            AttrPatch::Remove(rem) => match rem {
                Attribute::OnClick(_) => el.dyn_ref::<HtmlElement>().unwrap().set_onclick(None),
                Attribute::Class(_) => el.set_class_name(""),
                Attribute::Style(_) => el.set_attribute("style", "").unwrap(),
            },
        }
    }
}

impl<Msg: 'static + Send + Sync> Patch<Msg> {
    pub fn apply(self, el: dom::Element, channel: UnboundedSender<Arc<Msg>>) {
        match self {
            Patch::Add(add) => {
                if let Some(el) = add.render(el, channel) {
                    el.append_child(&el).unwrap();
                }
            }
            Patch::Replace(replace) => {
                if let Some(el) = replace.render(el, channel) {
                    el.replace_with_with_node_1(&el).unwrap();
                }
            }
            Patch::Update(children, attr) => {
                apply_children(el.clone(), el.children(), children, channel.clone());
                apply_attributes(el, attr, channel.clone());
            }
            Patch::Remove => el.remove(),
            Patch::Nothing => (),
        }
    }
}
