use roko_html::{Attribute, Html};

use crate::patch::{AttrPatch, Patch};

pub trait Diff {
    type Output;

    fn diff(old: Self, new: Self) -> Self::Output;
}

impl<Msg: PartialEq + Eq> Diff for Vec<Html<Msg>> {
    type Output = Vec<Patch<Msg>>;

    fn diff(old: Vec<Html<Msg>>, new: Vec<Html<Msg>>) -> Vec<Patch<Msg>> {
        let mut patches = Vec::new();

        let min_len = old.len().min(new.len());

        let mut new_iter = new.into_iter();
        let mut old_iter = old.into_iter();

        let added_children: Vec<_> = new_iter.by_ref().take(min_len).collect();
        let removed_children: Vec<_> = old_iter.by_ref().take(min_len).collect();

        for (old, new) in old_iter.zip(new_iter) {
            patches.push(Diff::diff(old, new));
        }

        patches.extend(added_children.into_iter().map(Patch::Add));
        patches.extend(removed_children.into_iter().map(|_| Patch::Remove));

        patches
    }
}

impl<Msg: PartialEq + Eq> Diff for Vec<Attribute<Msg>> {
    type Output = Vec<AttrPatch<Msg>>;

    fn diff(old: Vec<Attribute<Msg>>, new: Vec<Attribute<Msg>>) -> Vec<AttrPatch<Msg>> {
        let mut patches = Vec::new();

        let min_len = old.len().min(new.len());

        let mut new_iter = new.into_iter();
        let mut old_iter = old.into_iter();

        let added_attrs: Vec<_> = new_iter.by_ref().take(min_len).collect();
        let removed_attrs: Vec<_> = old_iter.by_ref().take(min_len).collect();

        for (old, new) in old_iter.zip(new_iter) {
            if old != new {
                patches.push(AttrPatch::Remove(old));
                patches.push(AttrPatch::Add(new));
            }
        }

        patches.extend(added_attrs.into_iter().map(AttrPatch::Add));
        patches.extend(removed_attrs.into_iter().map(AttrPatch::Remove));

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
