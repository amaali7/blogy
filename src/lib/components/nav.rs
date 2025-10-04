use crate::utils::json_db::NavNode;
use dioxus::prelude::*;
use dioxus_router::*;

#[derive(Props, Clone, PartialEq)]
pub struct NavBarProps {
    pub items: Vec<NavNode>,
}

#[component]
pub fn NavBar(props: NavBarProps) -> Element {
    rsx! {
        nav { class: "p-4",
            ul {
                for item in props.items {
                    match item {
                        NavNode::Page { name, path } => rsx! {
                            li { class: "mb-1",
                                Link {
                                    class: "block px-4 py-2 text-gray-600 hover:text-blue-600",
                                    to: path,
                                    "{name}"
                            }
                            }
                        },
                        NavNode::Directory { name, path, children } => rsx! {
                            li { class: "mb-2",
                                Link {
                                    class: "flex items-center justify-between w-full px-4 py-2 text-gray-700 hover:text-blue-600",
                                    to: path,
                                    span { "{name}" }
                                }
                                ul { class: "pl-4",
                                    // for child in children {
                                    //     if let NavNode::Page { name, path } = child {
                                    //         rsx! {
                                    //             li { class: "mb-1",
                                    //                 Link {
                                    //                     class: "block px-4 py-2 text-gray-600 hover:text-blue-600",
                                    //                     to: path,
                                    //                     name
                                    //                 }
                                    //             }
                                    //         }
                                    //     }
                                    // }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
// #[component]
// fn NavItem(props: NavItemProps) -> Element {
//     let class = if props.current {
//         "block px-4 py-2 text-blue-600 font-medium"
//     } else {
//         "block px-4 py-2 text-gray-600 hover:text-blue-600"
//     };

//     rsx! {
//         li { class: "mb-1",
//             a { class: class, href: "{props.path}", "{props.name}" }
//         }
//     }
// }

// #[derive(Props, Clone, PartialEq)]
// struct NavItemProps {
//     name: String,
//     path: String,
//     current: bool,
// }

// #[component]
// fn NavDropdown(props: NavDropdownProps) -> Element {
//     let is_open = use_state(|| false);
//     let has_active_child = props.children.iter().any(|child| match child {
//         NavNode::Page { path, .. } => path == &props.current_path,
//         _ => false,
//     });

//     rsx! {
//         li { class: "mb-2",
//             button {
//                 class: "flex items-center justify-between w-full px-4 py-2 text-gray-700 hover:text-blue-600",
//                 onclick: move |_| is_open.set(!is_open.get()),
//                 span { "{props.name}" }
//                 span { class: "ml-2", if is_open.get() { "▼" } else { "▶" } }
//             }
//             ul {
//                 class: if is_open.get() || has_active_child { "pl-4" } else { "hidden" },
//                 props.children.iter().map(|child| match child {
//                     NavNode::Page { name, path } => rsx! {
//                         NavItem {
//                             name: name.clone(),
//                             path: path.clone(),
//                             current: path == &props.current_path
//                         }
//                     },
//                     _ => None
//                 })
//             }
//         }
//     }
// }

// #[derive(Props, Clone, PartialEq)]
// struct NavDropdownProps {
//     name: String,
//     path: String,
//     children: Vec<NavNode>,
//     current_path: String,
// }
