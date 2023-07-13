//! Diffing algorithm for HTML nodes. It simply compares two nodes and returns a [Patch] that can
//! be applied to the DOM to update it.

use roko_html::{Attribute, Html};

use crate::patch::{AttrPatch, Patch};

/// Diff trait that is implemented for all types that can be diffed.
pub trait Diff {
    type Output;

    fn diff(old: Self, new: Self) -> Self::Output;
}

impl<Msg: PartialEq + Eq> Diff for Vec<Html<Msg>> {
    type Output = Vec<Patch<Msg>>;

    fn diff(old: Vec<Html<Msg>>, new: Vec<Html<Msg>>) -> Vec<Patch<Msg>> {
        let mut patches = Vec::new();

        let min_len = old.len().min(new.len());

        let mut added_children = new.into_iter();
        let mut removed_children = old.into_iter();

        let new_iter: Vec<_> = added_children.by_ref().take(min_len).collect();
        let old_iter: Vec<_> = removed_children.by_ref().take(min_len).collect();

        for (old, new) in old_iter.into_iter().zip(new_iter) {
            patches.push(Diff::diff(old, new));
        }

        patches.extend(added_children.map(Patch::Add));
        patches.extend(removed_children.map(|_| Patch::Remove));

        patches
    }
}

impl<Msg: PartialEq + Eq> Diff for Vec<Attribute<Msg>> {
    type Output = Vec<AttrPatch<Msg>>;

    fn diff(old: Vec<Attribute<Msg>>, new: Vec<Attribute<Msg>>) -> Vec<AttrPatch<Msg>> {
        let mut patches = Vec::new();

        let min_len = old.len().min(new.len());

        let mut added_attrs = new.into_iter();
        let mut removed_attrs = old.into_iter();

        let new_iter: Vec<_> = added_attrs.by_ref().take(min_len).collect();
        let old_iter: Vec<_> = removed_attrs.by_ref().take(min_len).collect();

        for (old, new) in old_iter.into_iter().zip(new_iter.into_iter()) {
            if old != new {
                patches.push(AttrPatch::Remove(old));
                patches.push(AttrPatch::Add(new));
            }
        }

        patches.extend(added_attrs.map(AttrPatch::Add));
        patches.extend(removed_attrs.map(AttrPatch::Remove));

        patches
    }
}

impl<Msg: PartialEq + Eq> Diff for Html<Msg> {
    type Output = Patch<Msg>;

    fn diff(old: Html<Msg>, new: Html<Msg>) -> Patch<Msg> {
        match (old, new) {
            (Html::Node(_), Html::Text(t)) => Patch::Replace(Html::Text(t)),
            (Html::Text(_), Html::Node(t)) => Patch::Replace(Html::Node(t)),
            (Html::Text(old), Html::Text(new_text)) if old == *new_text => Patch::Nothing,
            (Html::Text(_), Html::Text(t)) => Patch::Replace(Html::Text(t)),
            (Html::Node(old), Html::Node(new_ui)) if old.tag != new_ui.tag => {
                Patch::Replace(Html::Node(new_ui))
            }
            (Html::Node(old_ui), Html::Node(new_ui)) => {
                let children = Diff::diff(old_ui.children, new_ui.children);
                let attrs = Diff::diff(old_ui.attributes, new_ui.attributes);

                if children.is_empty() && attrs.is_empty() {
                    Patch::Nothing
                } else {
                    Patch::Update(children, attrs)
                }
            }
        }
    }
}
