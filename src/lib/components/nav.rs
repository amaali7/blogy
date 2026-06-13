use crate::{utils::json_db::NavNode, Route};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavBarProps {
    pub items: Vec<NavNode>,
}

fn path_to_route(path: &str) -> Option<Route> {
    let trimmed_path = path.trim_start_matches('/');

    if trimmed_path.is_empty() {
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
    pub path: String,
    #[props(default = Callback::default())]
    on_close: Callback<()>,
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
    let close_menu = props.on_close;

    if props.items.is_empty() {
        if let Some(route) = path_to_route(&props.path) {
            return rsx! {
                LinkItem {
                    label: props.name,
                    href: route,
                    current_page: false,
                    on_navigate: move |_| close_menu.call(()),
                }
            };
        }
    }

    let mut is_open = use_signal(|| false);
    let folder_route = path_to_route(&props.path);

    rsx! {
        li {
            class: "menu-item",
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
            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                is_open.with_mut(|v| *v = !*v);
            },
            if let Some(route) = folder_route {
                Link {
                    to: route,
                    class: "menu-label",
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        close_menu.call(());
                    },
                    "{props.name}"
                }
            } else {
                div { class: "menu-label", "{props.name}" }
            }
            ul {
                class: if is_open() { "dropdown-menu dropdown-open" } else { "dropdown-menu" },
                for item in props.items {
                    match item {
                        NavNode::Page { name, path } => {
                            if let Some(route) = path_to_route(&path) {
                                rsx! {
                                    LinkItem {
                                        label: name,
                                        href: route,
                                        current_page: false,
                                        on_navigate: move |_| {
                                            is_open.set(false);
                                            close_menu.call(());
                                        },
                                    }
                                }
                            } else {
                                rsx! {
                                    li {
                                        span { "{name} (Invalid path: {path})" }
                                    }
                                }
                            }
                        },
                        NavNode::Directory { name, path, children, .. } => {
                            rsx! {
                                MenuItem {
                                    name: name.clone(),
                                    path: path.clone(),
                                    items: children.clone(),
                                    on_close: close_menu,
                                }
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
    let route = props.href.clone();
    let on_nav = props.on_navigate;
    rsx! {
        li {
            Link {
                to: route,
                aria_current: if props.current_page { "page" } else { "false" },
                class: if props.current_page { "link-item current-page" } else { "link-item" },
                onclick: move |_| on_nav.call(()),
                "{props.label}"
            }
        }
    }
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    let mut root_open = use_signal(|| false);
    let close_menu = Callback::new(move |_| root_open.set(false));

    rsx! {
        nav {
            id: "navbar",
            class: "navbar",
            if let NavNode::Directory { name, children, .. } = &props.items[0] {
                li {
                    class: "menu-item logo-menu",
                    onpointerenter: move |e: Event<PointerData>| {
                        if e.data.pointer_type() != "touch" {
                            root_open.set(true);
                        }
                    },
                    onpointerleave: move |e: Event<PointerData>| {
                        if e.data.pointer_type() != "touch" {
                            root_open.set(false);
                        }
                    },
                    a {
                        href: "https://github.com/amaali7/blogy",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "logo menu-label",
                        "{name}"
                    }
                    ul {
                        id: "main_menu",
                        class: if root_open() {
                            "navbar-nav dropdown-menu dropdown-open"
                        } else {
                            "navbar-nav dropdown-menu"
                        },
                        for item in children.iter().cloned() {
                            match item {
                                NavNode::Page { name, path } => {
                                    if let Some(route) = path_to_route(&path) {
                                        rsx! {
                                            LinkItem {
                                                label: name,
                                                href: route,
                                                current_page: false,
                                                on_navigate: close_menu,
                                            }
                                        }
                                    } else {
                                        rsx! {}
                                    }
                                },
                                NavNode::Directory { name, path, children, .. } => {
                                    rsx! {
                                        MenuItem {
                                            name: name,
                                            path: path,
                                            items: children,
                                            on_close: close_menu,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
