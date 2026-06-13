use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::Element;

pub fn use_click_outside(element_id: &'static str, is_open: Signal<bool>) {
    use_hook(move || {
        let mut is_open = is_open.to_owned();

        let closure = Closure::wrap(Box::new(move |evt: web_sys::PointerEvent| {
            if !is_open() {
                return;
            }

            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };

            let Some(root) = document.get_element_by_id(element_id) else {
                return;
            };

            let Ok(target) = evt.target().unwrap().dyn_into::<Element>() else {
                return;
            };

            if !root.contains(Some(&target)) {
                is_open.set(false);
            }
        }) as Box<dyn FnMut(_)>);

        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            let _ = document.add_event_listener_with_callback(
                "pointerdown",
                closure.as_ref().unchecked_ref(),
            );
        }

        closure.forget();
    });
}
