use std::sync::OnceLock;
use dioxus::prelude::*;
use dioxus_router::*;
use syntect::{highlighting::ThemeSet, parsing::{SyntaxDefinition, SyntaxSet}};

mod components;
mod utils;

pub use components::{NavBar, PreviewArea};
pub use utils::json_db::{JsonDb, DataError};

// Static resources
pub static BASE_URL: &str = "https://raw.githubusercontent.com/amaali7/markdown_files/refs/heads/main/MarkDown/";
pub static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
pub static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
// pub static JSON_DB: OnceLock<JsonDb> = OnceLock::new();
// To use RwLock for thread safety:
use std::sync::RwLock;
pub static JSON_DB: OnceLock<RwLock<JsonDb>> = OnceLock::new();
static CONTENT_STATE: GlobalSignal<ContentState> = Signal::global(|| ContentState::Loading);


#[derive(Routable, Clone)]
enum Route {
    #[layout(AppContent)]
        #[route("/:..path")]
        PageContent{ path: Vec<String> },
}

#[component]
fn PageContent(path: Vec<String>) -> Element {

    use_effect(move || {
        let path_ = path.join("/").to_string();

        spawn(async move {
            // if let Some(mut db) = json_db. {
                match load_content(&path_).await {
                    Ok(state) => *CONTENT_STATE.write() = state,
                    Err(e) => {
                        eprintln!("Content loading error: {}", e);
                        *CONTENT_STATE.write() = ContentState::Error(e.to_string());
                    }
                }
            // }
        });
    });

    match &*(CONTENT_STATE.read()) {
        ContentState::Loading => rsx! { LoadingSpinner {} },
        ContentState::Error(e) => rsx! { ErrorMessage { error: e.clone() } },
        ContentState::Ready(content) => rsx! {
            PreviewArea {
                content: content.clone(),
                class: "max-w-4xl mx-auto"
            }
        }
    }
}


#[component]
pub fn AppContent() -> Element {

    rsx! {
        link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
        div { class: "flex min-h-screen bg-gray-50",
            div { class: "w-64 bg-white border-r",
                // match JSON_DB.get() {
                //     Some(db) => rsx! { NavBar { items: db.get_nav_tree() } },
                //     None => rsx! { div { class: "p-4 text-gray-500", "Loading navigation..." } }
                // }
                match JSON_DB.get() {
                    Some(db_lock) => {
                        let db = db_lock.read().unwrap(); // or handle error properly
                        rsx! { NavBar { items: db.get_nav_tree() } }
                    },
                    None => rsx! { div { class: "p-4 text-gray-500", "Loading navigation..." } }
                }
            }
            main { class: "flex-1 p-6 overflow-auto",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    let db = use_resource(move || async move { JsonDb::load().await });

    match &*db.read_unchecked() {
        Some(Ok(jsondb)) => {
            if JSON_DB.get().is_none() {
                JSON_DB.get_or_init(|| {
                    RwLock::new(jsondb.clone())
                });
            }
            rsx! {
                Router::<Route> {
                }
            }
        },
        Some(Err(e)) => rsx! { p { "Loading Json DB failed, {e}" } },
        None =>  rsx! { p { "Loading..." } }
    }
}

// #[component]
// pub fn App() -> Element {
//     let db = use_resource(move || async move { JsonDb::load().await });

//     match &*db.read_unchecked() {
//         Some(Ok(jsondb)) => {
//             if JSON_DB.get().is_none() {
//                 JSON_DB.get_or_init(|| {
//                     jsondb.clone()
//                 });
//             }
//             rsx! {
//                 Router::<Route> {
//                 }
//             }
//         },
//         Some(Err(e)) => rsx! { p { "Loading Json DB failed, {e}" } },
//         None =>  rsx! { p { "Loading..." } }
//     }
// }



#[derive(Clone)]
enum ContentState {
    Loading,
    Error(String),
    Ready(String),
}

async fn load_content(path: &str) -> Result<ContentState, DataError> {
    let db_lock = JSON_DB.get().unwrap();
    let db = db_lock.read().unwrap();

    let (section, page) = db.find_page(path)
        .ok_or(DataError::PageNotFound)?;

    // You'll need to clone or use interior mutability
    let mut db_clone = db.clone();
    let content = db_clone.get_html_content(section, page).await?;

    Ok(ContentState::Ready(content))
}

// async fn load_content(path: &str) -> Result<ContentState, DataError> {
//     let (section, page) = JSON_DB.get().unwrap().find_page(path)
//         .ok_or(DataError::PageNotFound)?;
//     let mut db = JSON_DB.get().unwrap().clone();
//     let content = db.get_html_content(&section, &page).await?;

//     Ok(ContentState::Ready(content))
// }

#[component]
fn LoadingSpinner() -> Element {
    rsx! {
        div { class: "flex justify-center items-center h-full",
            div { class: "animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500" }
        }
    }
}

#[component]
fn ErrorMessage(error: String) -> Element {
    rsx! {
        div { class: "bg-red-50 border-l-4 border-red-500 text-red-700 p-4 rounded",
            div { class: "flex items-center",
                svg {
                    class: "w-5 h-5 mr-2",
                    view_box: "0 0 20 20",
                    fill: "currentColor",
                    path {
                        fill_rule: "evenodd",
                        d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                        clip_rule: "evenodd"
                    }
                }
                p { "Error: {error}" }
            }
        }
    }
}
// use std::sync::OnceLock;
// use dioxus::{ prelude::*};
// use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

// mod components;
// mod utils;

// pub use components::{NavBar, PreviewArea};
// pub use utils::json_db::{JsonDb, DataError};

// // Static resources
// pub static BASE_URL: &str = "https://raw.githubusercontent.com/amaali7/markdown_files/refs/heads/main/MarkDown/";
// pub static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
// pub static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
// pub static JSON_DB: OnceLock<JsonDb> = OnceLock::new();
// static CONTENT_STATE: GlobalSignal<ContentState> = Signal::global(|| ContentState::Loading);


// #[derive(Routable, Clone)]
// enum Route {
//         #[layout(AppContent)]
//             #[route("/:..path")]
//             PageContent{ path: Vec<String> },
// }

// #[component]
// fn PageContent(path: Vec<String>) -> Element {

//     // use_effect(move || {
//         let path_ = path.join("/").to_string();

//         spawn(async move {
//             if JSON_DB.get().is_some() {
//                 match load_content(&path_).await {
//                     Ok(state) => *CONTENT_STATE.write() = state,
//                     Err(e) => {
//                         eprintln!("Content loading error: {}", e);
//                         *CONTENT_STATE.write() = ContentState::Error(e.to_string());
//                     }
//                 }
//             }
//         // });
//     });

//     match &*(CONTENT_STATE.read()) {
//         ContentState::Loading => rsx! { LoadingSpinner {} },
//         ContentState::Pending => rsx! { LoadingSpinner {} },
//         ContentState::Error(e) => rsx! { ErrorMessage { error: e.clone() } },
//         ContentState::Ready(content) => rsx! {
//             PreviewArea {
//                 content: content.clone(),
//                 class: "max-w-4xl mx-auto"
//             }
//         }
//     }
// }


// #[component]
// pub fn AppContent() -> Element {
//     use_effect(move || {
//        init_app();
//     });

//     rsx! {
//         link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
//         div { class: "flex min-h-screen bg-gray-50",
//             div { class: "w-64 bg-white border-r",
//                 match JSON_DB.get() {
//                     Some(db) => rsx! { NavBar { items: db.get_nav_tree() } },
//                     None => rsx! { div { class: "p-4 text-gray-500", "Loading navigation..." } }
//                 }
//             }
//             main { class: "flex-1 p-6 overflow-auto",
//                 Outlet::<Route> {}
//             }
//         }
//     }
// }




// #[component]
// pub fn App() -> Element {
//     rsx! {
//         Router::<Route> {
//         }
//     }
// }



// fn init_app() {
//     // SYNTAX_SET.get_or_init(|| {
//     //     let nix_syntax = include_str!("../../syntax/nix.sublime-syntax");
//     //     let syntax = SyntaxDefinition::load_from_str(nix_syntax, true, None)
//     //         .expect("Failed to load Nix syntax");

//     //     let mut builder = SyntaxSet::load_defaults_newlines().into_builder();
//     //     builder.add(syntax);
//     //     builder.build()
//     // });

//     // THEME_SET.get_or_init(ThemeSet::load_defaults);

//     spawn(async move {
//         match JsonDb::load().await {
//             Ok(db) => {let _ =JSON_DB.set(db.clone());},
//             Err(e) => eprintln!("Failed to load site data: {}", e),
//         };
//         *CONTENT_STATE.write() = ContentState::Pending;
//     });
// // Load JSON database asynchronously
//     // if JSON_DB.get().is_none() {
//     //     spawn(async move {
//     //         match JsonDb::load().await {
//     //             Ok(db) => {
//     //                 let _ = JSON_DB.set(db);
//     //                 // Trigger re-render after DB is loaded
//     //                 // You might need to use a signal here to force update
//     //             },
//     //             Err(e) => eprintln!("Failed to load site data: {}", e),
//     //         };
//     //     });
//     // }
//     }

// #[derive(Clone)]
// enum ContentState {
//     Loading,
//     Pending,
//     Error(String),
//     Ready(String),
// }


// async fn load_content(path: &str) -> Result<ContentState, DataError> {
//     let (section, page) = JSON_DB.get().unwrap().find_page(path)
//         .ok_or(DataError::PageNotFound)?;
//     let mut db = JSON_DB.get().unwrap().clone();
//     let content = db.get_html_content(&section, &page).await?;

//     Ok(ContentState::Ready(content))
// }

// #[component]
// fn LoadingSpinner() -> Element {
//     rsx! {
//         div { class: "flex justify-center items-center h-full",
//             div { class: "animate-spin rounded-full h-24 w-24 border-t-2 border-b-2 border-blue-500" }
//         }
//     }
// }

// #[component]
// fn ErrorMessage(error: String) -> Element {
//     rsx! {
//         div { class: "bg-red-50 border-l-4 border-red-500 text-red-700 p-4 rounded",
//             div { class: "flex items-center",
//                 svg {
//                     class: "w-5 h-5 mr-2",
//                     view_box: "0 0 20 20",
//                     fill: "currentColor",
//                     path {
//                         fill_rule: "evenodd",
//                         d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
//                         clip_rule: "evenodd"
//                     }
//                 }
//                 p { "Error: {error}" }
//             }
//         }
//     }
// }
