use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};

fn use_scroll_past(threshold: f64) -> Signal<bool> {
    let past = use_signal(|| false);

    use_hook(move || {
        let mut past = past.to_owned();

        if let Some(window) = web_sys::window() {
            let y = window.scroll_y().unwrap_or(0.0);
            past.set(y > threshold);
        }

        let closure = Closure::wrap(Box::new(move || {
            if let Some(window) = web_sys::window() {
                let y = window.scroll_y().unwrap_or(0.0);
                past.set(y > threshold);
            }
        }) as Box<dyn FnMut()>);

        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        }

        closure.forget();
    });

    past
}

pub fn use_header_compact(threshold: f64) -> Signal<bool> {
    use_scroll_past(threshold)
}

pub fn use_scrolled_past(threshold: f64) -> Signal<bool> {
    use_scroll_past(threshold)
}
