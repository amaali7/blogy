use dioxus::prelude::*;

#[component]
pub fn ScrollToTop() -> Element {
    let visible = crate::utils::hooks::scroll_header::use_scrolled_past(320.0);

    rsx! {
        button {
            class: if visible() { "scroll-top scroll-top--visible" } else { "scroll-top" },
            r#type: "button",
            aria_label: "Scroll to top",
            onclick: move |_| {
                if let Some(window) = web_sys::window() {
                    if let Some(doc) = window.document() {
                        if let Some(root) = doc.document_element() {
                            root.set_scroll_top(0);
                        }
                    }
                }
            },
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 19V5" }
                path { d: "M5 12l7-7 7 7" }
            }
        }
    }
}
