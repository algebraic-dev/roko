use roko_dom::elements::*;
use roko_html::{Attrs, Children, Html};
use roko_macro::html;

use crate::{components::project, Message, Model};

pub fn page(model: &Model, _attrs: Attrs<Message>, _children: Children<Message>) -> Html<Message> {
    let posts = model.projects.iter().take(3).map(project_card).collect();

    html! {
        <div class="project-posts">
            <div class="posts" children={posts} />
        </div>
    }
}

fn project_card(x: &crate::Project) -> Html<Message> {
    html! {
        <project::card
            title={x.title.clone()}
            description={x.description.clone()}
            link={x.link.clone()}
        />
    }
}
