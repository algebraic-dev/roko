//! Module to data structures and functions related to events in the browser. This module contains
//! functions to handle events in the browser.

use std::sync::Arc;

use crate::Channel;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::HashChangeEvent;

pub fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

/// Function to handle the `onhashchange` event in the browser.
pub fn on_hash_change<Msg: 'static>(channel: Channel<Msg>, on_change: fn(String) -> Msg) {
    let data: Box<dyn FnMut(HashChangeEvent)> = Box::new(move |e: HashChangeEvent| {
        let old = e.old_url();
        let new = e.new_url();

        if old != new {
            let new = window().location().hash().unwrap();
            channel.unbounded_send(Arc::new(on_change(new))).unwrap();
        }
    });

    let closure = Closure::wrap(data);
    window().set_onhashchange(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

/// Function to change the hash in the browser.
pub fn change_hash(hash: &str) {
    window().location().set_hash(hash).unwrap();
}

pub fn get_page_hash() -> String {
    window().location().hash().unwrap()
}

pub fn change_title(title: &str) {
    window().document().unwrap().set_title(title);
}
