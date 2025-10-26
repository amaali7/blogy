use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Document, Element};

pub fn use_click_outside(elem: Signal<Option<Element>>, on_outside: impl Fn() + 'static) {
    let cb = use_hook(|| std::rc::Rc::new(on_outside));

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let doc: Document = window.document().unwrap();

        let listener = Closure::wrap(Box::new(move |evt: web_sys::PointerEvent| {
            if let Some(el) = elem() {
                let target = evt.target().unwrap().dyn_into::<Element>().unwrap();
                if !el.contains(Some(&target)) {
                    cb();
                }
            }
        }) as Box<dyn FnMut(_)>);

        doc.add_event_listener_with_callback("pointerdown", listener.as_ref().unchecked_ref())
            .unwrap();

        // move the closure into the cleanup value so it stays alive
        let listener = std::rc::Rc::new(listener);
        move || {
            doc.remove_event_listener_with_callback(
                "pointerdown",
                listener.as_ref().unchecked_ref(),
            )
            .unwrap();
        }
    });
}
