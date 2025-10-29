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
    #[props(default = String::default())]
    id: String,
}

#[derive(Clone, PartialEq, Props)]
struct LinkItemProps {
    label: String,
    href: Route,
    current_page: bool,
    #[props(default = Callback::default())]
    on_navigate: Callback<()>,
}

#[component]
fn MenuItem(props: MenuItemProps) -> Element {
    let mut is_open = use_signal(|| false);
    let id_val = if props.id.is_empty() {
        None
    } else {
        Some(props.id.clone())
    };

    rsx! {
        li {
            id: id_val,
            class: "menu-item",
            // onmouseenter: move |_| is_open.set(true),
            // onmouseleave: move |_| is_open.set(false),
            // onclick: toggle,                       // ← new
            // desktop hover
            // attach the DOM element to the ref
            onpointerenter: move |e: Event<PointerData>| {
                if e.data.pointer_type() != "touch" {
                    is_open.set(true);
                }
            },
            onpointerleave: move |e: Event<PointerData>| {
                if e.data.pointer_type() != "touch" {
                    is_open.set(false);
                }
            },

            // mobile / keyboard tap
            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                is_open.with_mut(|v| *v = !*v);
            },            div { class: "menu-label", "{props.name}" }
            ul {
                class: if is_open() { "dropdown-menu dropdown-open" }
                       else { "dropdown-menu" },
                for item in props.items {
                    match item {
                        NavNode::Page { name, path } => {
                            if let Some(route) = path_to_route(&path) {

                                rsx! {
                                    LinkItem{ label: name, href: route, current_page: false, on_navigate: move |_| is_open.set(false),}
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
    let route = props.href.clone(); // Route enum we built in path_to_route
    let on_nav = props.on_navigate;
    rsx! {
        li {
            Link {
                to: route,
                aria_current: if props.current_page { "page" } else { "false" },
                class: if props.current_page { "link-item current-page" }
                       else { "link-item" },
                onclick: move |_| on_nav.call(()),
                "{props.label}"
            }
        }
    }
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    rsx! {
        nav {
            id: "navbar",
            class: "navbar",
            if let NavNode::Directory { name, children,  .. } = &props.items[0]{
                // div { class: "logo", "{name}" }
                a {
                    href: "https://github.com/amaali7/blogy",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "logo", "{name}"
                }
                MenuItem{name: "Menu", items: children.clone(), id: "main_menu"}
            }
        }
    }
}
