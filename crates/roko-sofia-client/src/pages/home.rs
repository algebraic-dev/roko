use roko_dom::elements::*;
use roko_html::{Attrs, Children, Html};
use roko_macro::html;

use crate::{components::home_card, Message, Model};

pub fn page(model: &Model, _attrs: Attrs<Message>, _children: Children<Message>) -> Html<Message> {
    let posts = model.projects.iter().take(3).map(home_card).collect();

    html! {
        <main class="home">
            <section class="left-side">
                <div class="introduction">
                    <p class="hi">
                       "Hi,"
                    </p>
                    <p class="description">
                        "I'm Sofia, a software engineer that loves compilers!"
                    </p>
                </div>
            </section>
            <div class="ball"/>
            <section class="right-side">
                <div class="posts" children={posts} />
            </section>
        </main>
    }
}

fn home_card(x: &crate::Project) -> Html<Message> {
    html! {
        <home_card::card
            title={x.title.clone()}
            description={x.description.clone()}
            link={x.link.clone()}
        />
    }
}
