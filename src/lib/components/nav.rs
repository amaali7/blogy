use std::sync::OnceLock;

use crate::{utils::json_db::NavNode, Route};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavBarProps {
    pub items: Vec<NavNode>,
}

// Consider making this more robust - handle edge cases better
fn path_to_route(path: &str) -> Option<Route> {
    let trimmed_path = path.trim_start_matches('/');

    if trimmed_path.is_empty() {
        // Handle root path - adjust based on your actual root route
        return Some(Route::PageContent {
            path: vec!["pages".to_string(), "home".to_string()],
        });
    }

    let segments: Vec<String> = trimmed_path
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    if !segments.is_empty() {
        Some(Route::PageContent { path: segments })
    } else {
        None
    }
}

#[derive(Clone, PartialEq, Props)]
struct MenuItemProps {
    pub name: String,
    pub items: Vec<NavNode>,
}

#[derive(Clone, PartialEq, Props)]
struct LinkItemProps {
    label: String,
    href: String,
    current_page: bool,
}

#[component]
fn MenuItem(props: MenuItemProps) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        li {
            class: "menu-item",
            onmouseenter: move |_| is_open.set(true),
            onmouseleave: move |_| is_open.set(false),

            div {
                class: "menu-label",
                "{props.name}"
                span { class: "dropdown-arrow", "▼" }
            }
            ul {
                class: if is_open() { "dropdown-menu dropdown-open" } else { "dropdown-menu" },
                for item in props.items {
                    match item {
                        NavNode::Page { name, path } => {
                            if let Some(route) = path_to_route(&path) {

                                rsx! {
                                    LinkItem{ label: name, href: route, current_page: false}
                                    }
                            } else {
                                rsx! {
                                    li {
                                        span {
                                            "{name} (Invalid path: {path})"
                                        }
                                    }
                                }
                            }
                        },
                        NavNode::Directory { name, children, .. } => {
                            rsx! {
                                MenuItem{ name: name , items: children.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LinkItem(props: LinkItemProps) -> Element {
    rsx! {
        li {
            a {
                href: "{props.href}",
                "data-dioxus-id": "auto-generate",
                aria_current: if props.current_page { "page" } else { "false" },
                class: if props.current_page { "link-item current-page" } else { "link-item " },
                "{props.label}"
            }
        }
    }
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    let js = serde_wasm_bindgen::to_value(&props.items).unwrap();
    web_sys::console::log_1(&js);
    rsx! {
        nav {
            id: "navbar",
            class: "navbar",
            if let NavNode::Directory { name, children,  .. } = &props.items[0]{
                div { class: "logo", "{name}" }
                MenuItem{name: "Menu", items: children.clone()}
            }
        }
    }
}
