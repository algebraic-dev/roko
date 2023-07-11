use crate::diff::Diff;
use crate::render::Render;

use futures::channel::mpsc;
use futures::StreamExt;
use roko_html::Html;

pub async fn start<Model, Msg: Eq + PartialEq + std::fmt::Debug + Send + Sync + 'static>(
    mut view: impl FnMut(&Model) -> Html<Msg>,
    mut update: impl FnMut(&Msg, &mut Model),
    mut init: Model,
) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut result: Html<Msg> = view(&init);

    let (sender, mut recv) = mpsc::unbounded();

    let result_to = result.clone();
    let sender_to = sender.clone();

    let res = result_to.render(body.clone().into(), sender_to);

    if let Some(el) = res {
        body.append_child(&el.clone()).unwrap();

        let body = document.body().expect("document should have a body");

        while let Some(msg) = recv.next().await {
            update(&msg, &mut init);
            let result_new = view(&init);

            let diff = Diff::diff(result, result_new.clone());

            web_sys::console::log_1(&format!("diff: {:?}", diff).into());

            diff.apply(el.clone(), sender.clone());

            result = result_new;
        }
    }
}
