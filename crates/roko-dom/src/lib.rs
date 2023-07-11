//! This module renders a virtual dom to a the real dom, does patching and reconciliation.

pub mod app;
pub mod diff;
pub mod elements;
pub mod patch;
pub mod render;

use std::pin::Pin;
use std::sync::Arc;

use wasm_bindgen::JsValue;
pub use web_sys as dom;

use crate::diff::Diff;
use crate::render::Render;

use futures::channel::mpsc;
use futures::{Future, StreamExt};
use roko_html::Html;

pub struct Cmd<Model, Msg> {
    future: Pin<Box<dyn Future<Output = Option<Msg>>>>,
    model: Model,
}

#[macro_export]
macro_rules! response {
    ($e:expr, $p:expr) => {
        $crate::Response::new($e, Box::new(Box::pin($p)))
    };
}

impl<Model, Msg: 'static> Cmd<Model, Msg> {
    pub fn new(model: Model, future: Box<dyn Future<Output = Option<Msg>> + Unpin>) -> Self {
        Self {
            future: Box::pin(future),
            model,
        }
    }

    pub fn none(model: Model) -> Self {
        Self {
            future: Box::pin(futures::future::ready(None)),
            model,
        }
    }
}

impl<T: Unpin> Future for Cmd<T, ()> {
    type Output = Option<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();

        let res = this.future.as_mut().poll(cx);

        if let std::task::Poll::Ready(Some(_)) = res {
            std::task::Poll::Ready(Some(()))
        } else {
            std::task::Poll::Pending
        }
    }
}

/// This is the entrypoint of all the roko applications. It takes a view function, an update
/// function and an initial model and starts the application. The view function is called every
/// time the model is updated and the update function is called every time a message is sent to the
/// application. The initial model is the model that is used to start the application.
pub async fn start<
    Model,
    Msg: Eq + PartialEq + std::fmt::Debug + Send + Sync + 'static + Clone,
    V,
    U,
>(
    mut view: V,
    mut update: U,
    mut init: Cmd<Model, Msg>,
) -> Result<(), JsValue>
where
    V: FnMut(&Model) -> Html<Msg>,
    U: FnMut(Msg, Model) -> Cmd<Model, Msg>,
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // The initial view of the application.
    // TODO: Hydration
    let mut result: Html<Msg> = view(&init.model);

    let (sender, mut recv) = mpsc::unbounded();

    let result_to = result.clone();
    let sender_to = sender.clone();

    let res = result_to.render(body.clone().into(), sender_to);

    if let Some(el) = res {
        body.append_child(&el.clone()).unwrap();

        if let Some(msg) = init.future.await {
            sender.unbounded_send(Arc::new(msg)).unwrap();
        }

        while let Some(msg) = recv.next().await {
            let new_init = update(msg.as_ref().clone(), init.model);

            init = new_init;

            let result_new = view(&init.model);

            let diff = Diff::diff(result, result_new.clone());

            result = result_new;

            diff.apply(el.clone(), sender.clone());

            if let Some(msg) = init.future.await {
                sender.unbounded_send(Arc::new(msg)).unwrap();
            }
        }
    }

    Ok(())
}

pub async fn none<Msg>() -> Option<Msg> {
    None
}
