use roko_html::{Attribute, Html};
use web_sys as dom;

use crate::render::Render;

pub enum AttrPatch<Msg> {
    Add(Attribute<Msg>),
    Remove(Attribute<Msg>),
}

pub enum Patch<Msg> {
    Add(Html<Msg>),
    Replace(Html<Msg>),
    Update(Vec<Patch<Msg>>, Vec<AttrPatch<Msg>>),
    Remove,
    Nothing,
}

impl<Msg> Patch<Msg> {
    pub fn apply_children(self, parent: dom::Element) {}

    pub fn apply(self, el: dom::Element) {
        match self {
            Patch::Add(add) => {
                if let Some(el) = add.render(el) {
                    el.append_child(&el).unwrap();
                }
            }
            Patch::Replace(replace) => {
                if let Some(el) = replace.render(el) {
                    el.replace_with_with_node_1(&el).unwrap();
                }
            }
            Patch::Update(children, attr) => {
                let child = el.children();
            }
            Patch::Remove => el.remove(),
            Patch::Nothing => todo!(),
        }
    }
}
