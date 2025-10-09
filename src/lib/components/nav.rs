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
                                        class: "block px-4 py-2 text-gray-600 hover:text-blue-600 hover:bg-gray-100 rounded transition-colors btn",
                                        to: route,
                                        "{name}"
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                li { class: "mb-1",
                                    span {
                                        class: "block px-4 py-2 text-gray-400 cursor-not-allowed",
                                        "{name} (Invalid path: {path})"
                                    }
                                }
                            }
                        }
                    },
                    NavNode::Directory { name, children, .. } => {
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
