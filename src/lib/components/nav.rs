use crate::{utils::json_db::NavNode, Route};
use dioxus::prelude::*;
use dioxus_router::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavBarProps {
    pub items: Vec<NavNode>,
}

fn path_to_route(path: &str) -> Option<Route> {
    let segments: Vec<String> = path
        .trim_start_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    if !segments.is_empty() {
        Some(Route::PageContent { path: segments })
    } else {
        Some(Route::PageContent {
            path: vec!["pages".to_string(), "home".to_string()],
        })
    }
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    fn render_nav_items(items: &[NavNode]) -> Element {
        rsx! {
            for item in items {
                match item {
                    NavNode::Page { name, path } => {
                        if let Some(route) = path_to_route(&path) {
                            rsx! {
                                li { class: "mb-1",
                                    Link {
                                        class: "block px-4 py-2 text-gray-600 hover:text-blue-600 hover:bg-gray-100 rounded transition-colors",
                                        to: route,
                                        "{name}"
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                li { class: "mb-1",
                                    span { class: "block px-4 py-2 text-gray-400", "{name} (Invalid)" }
                                }
                            }
                        }
                    },
                    NavNode::Directory { name, path, children } => {
                        rsx! {
                            li { class: "mb-2",
                                // Directory as non-clickable header
                                div { class: "px-4 py-2 text-gray-700 font-medium", "{name}" }
                                ul { class: "pl-4 mt-1",
                                    {render_nav_items(&children)}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    rsx! {
        nav { class: "p-4",
            ul {
                {render_nav_items(&props.items)}
            }
        }
    }
}
