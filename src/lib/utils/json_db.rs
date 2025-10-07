use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use gloo_storage::{LocalStorage, Storage};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use syntect::html::highlighted_html_for_string;
use crate::{BASE_URL, SYNTAX_SET, THEME_SET};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonDb {
    #[serde(skip)]
    pages: HashMap<PageKey, PageData>,
    #[serde(skip)]
    nav_tree: Vec<NavNode>,
    #[serde(skip)]
    html_cache: HashMap<String, String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct PageKey {
    section: String,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PageData {
    path: String,
    file: Option<String>,
    last_updated: Option<NaiveDateTime>,
    raw_content: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavNode {
    Page { name: String, path: String },
    Directory { name: String, path: String, children: Vec<NavNode> },
}

impl JsonDb {
    pub async fn load() -> Result<Self, DataError> {
        if let Ok(cached) = LocalStorage::get::<String>("JsonDB") {
            Self::from_json(&cached)
        }else {
           Self::update().await
        }
    }


    pub async fn update() -> Result<Self, DataError> {
        let json = reqwest::get(format!("{}index.json", BASE_URL))
            .await?
            .text()
            .await?;
        let _ = LocalStorage::set("JsonDB", &json);
        Self::from_json(&json)
    }

    pub fn from_json(json: &str) -> Result<Self, DataError> {
        let value: serde_json::Value = serde_json::from_str(json)?;
        let mut db = Self {
            pages: HashMap::new(),
            nav_tree: Vec::new(),
            html_cache: HashMap::new(),
        };
        db.build_cache(&value)?;
        Ok(db)
    }

    // fn build_cache(&mut self, value: &serde_json::Value) -> Result<(), DataError> {
    //     if let Some(root) = value.get("root") {
    //         let mut nav_tree = Vec::new();
    //         Self::process_node_static(
    //             root,
    //             "",
    //             &mut nav_tree,
    //             &mut self.pages
    //         )?;
    //         self.nav_tree = nav_tree;
    //     }
    //     Ok(())
    // }

    // fn process_node_static(
    //     node: &serde_json::Value,
    //     current_section: &str,
    //     nav_nodes: &mut Vec<NavNode>,
    //     pages: &mut HashMap<PageKey, PageData>,
    // ) -> Result<(), DataError> {
    //     let name = node["name"].as_str().ok_or(DataError::InvalidStructure)?;
    //     let path = node["path"].as_str().ok_or(DataError::InvalidStructure)?;
    //     let item_type = node["type"].as_str().ok_or(DataError::InvalidStructure)?;

    //     match item_type {
    //         "page" => {
    //             let key = PageKey {
    //                 section: current_section.to_string(),
    //                 name: name.to_string(),
    //             };

    //             pages.insert(key, PageData {
    //                 path: path.to_string(),
    //                 file: node["file"].as_str().map(|s| s.to_string()),
    //                 last_updated: node["date"].as_str()
    //                     .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").ok()),
    //                 raw_content: None,
    //             });

    //             nav_nodes.push(NavNode::Page {
    //                 name: name.to_string(),
    //                 path: path.to_string(),
    //             });
    //         }
    //         "directory" => {
    //             let mut children = Vec::new();
    //             if let Some(child_nodes) = node["children"].as_array() {
    //                 for child in child_nodes {
    //                     Self::process_node_static(child, name, &mut children, pages)?;
    //                 }
    //             }

    //             nav_nodes.push(NavNode::Directory {
    //                 name: name.to_string(),
    //                 path: path.to_string(),
    //                 children,
    //             });
    //         }
    //         _ => return Err(DataError::InvalidStructure),
    //     }
    //     Ok(())
    // }

fn build_cache(&mut self, value: &serde_json::Value) -> Result<(), DataError> {
    if let Some(root) = value.get("root") {
        let mut nav_tree = Vec::new();
        Self::process_node_static(
            root,
            "", // Start with empty path for root
            &mut nav_tree,
            &mut self.pages
        )?;
        self.nav_tree = nav_tree;
    }
    Ok(())
}

    fn process_node_static(
    node: &serde_json::Value,
    current_path: &str, // Changed from current_section to current_path
    nav_nodes: &mut Vec<NavNode>,
    pages: &mut HashMap<PageKey, PageData>,
) -> Result<(), DataError> {
    let name = node["name"].as_str().ok_or(DataError::InvalidStructure)?;
    let path = node["path"].as_str().ok_or(DataError::InvalidStructure)?;
    let item_type = node["type"].as_str().ok_or(DataError::InvalidStructure)?;

    match item_type {
        "page" => {
            let key = PageKey {
                section: current_path.to_string(), // Use current_path as section
                name: name.to_string(),
            };

            pages.insert(key, PageData {
                path: path.to_string(),
                file: node["file"].as_str().map(|s| s.to_string()),
                last_updated: node["date"].as_str()
                    .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").ok()),
                raw_content: None,
            });

            nav_nodes.push(NavNode::Page {
                name: name.to_string(),
                path: path.to_string(),
            });
        }
        "directory" => {
            let mut children = Vec::new();
            if let Some(child_nodes) = node["children"].as_array() {
                for child in child_nodes {
                    // Pass the directory's path as the new current_path
                    Self::process_node_static(child, path, &mut children, pages)?;
                }
            }

            nav_nodes.push(NavNode::Directory {
                name: name.to_string(),
                path: path.to_string(),
                children,
            });
        }
        _ => return Err(DataError::InvalidStructure),
    }
    Ok(())
}

    pub fn get_nav_tree(&self) -> Vec<NavNode> {
        self.nav_tree.clone()
    }

    // Change all mutable self references to immutable where possible
    pub async fn get_html_content(&mut self, section: &str, page: &str) -> Result<String, DataError> {
        let path = self.get_page_path(section, page)?;

        if let Some(cached) = self.html_cache.get(&path) {
            return Ok(cached.clone());
        }

        let markdown = self.get_raw_content(section, page).await?;
        let html = markdown_to_html(&markdown);
        // Need interior mutability here - consider using RwLock or Mutex
        // For now, we'll skip caching in this example
        Ok(html)
    }

    // pub fn find_page(&self, path: &str) -> Option<(&str, &str)> {
    //     self.pages.iter()
    //         .find(|(_, data)| data.path == path)
    //         .map(|(key, _)| (key.section.as_str(), key.name.as_str()))
    // }
    pub fn find_page(&self, path: &str) -> Option<(&str, &str)> {
    // Normalize the path by ensuring it starts with /
    let search_path = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    };

    self.pages.iter()
        .find(|(_, data)| data.path == search_path)
        .map(|(key, _)| (key.section.as_str(), key.name.as_str()))
}

    async fn get_raw_content(
        &mut self,
        section: &str,
        page: &str,
    ) -> Result<String, DataError> {
        let key = PageKey {
            section: section.to_string(),
            name: page.to_string(),
        };

        // Separate the mutable borrow of pages from the async operation
        let needs_load = {
            let page_data = self.pages.get_mut(&key)
                .ok_or(DataError::PageNotFound)?;

            if page_data.raw_content.is_none() {
                true
            } else {
                return Ok(page_data.raw_content.as_deref().unwrap().to_string());
            }
        };

        if needs_load {
            let content = self.load_content_from_source(section, page).await?;
            if let Some(page_data) = self.pages.get_mut(&key) {
                page_data.raw_content = Some(content.clone());
            }
            Ok(content)
        } else {
            Ok(self.pages[&key].raw_content.as_ref().unwrap().clone())
        }
    }

    async fn load_content_from_source(
        &self,
        section: &str,
        page: &str,
    ) -> Result<String, DataError> {
        let storage_key = format!("{}-{}", section, page);

        // 1. Check local storage first
        if let Ok(cached) = LocalStorage::get(&storage_key) {
            return Ok(cached);
        }

        // 2. Download from network
        let url = self.get_download_url(section, page)?;
        let content = reqwest::get(&url).await?.text().await?;

        // 3. Cache in storage
        LocalStorage::set(&storage_key, &content)?;

        Ok(content)
    }

    fn get_page_path(&self, section: &str, page: &str) -> Result<String, DataError> {
        let key = PageKey {
            section: section.to_string(),
            name: page.to_string(),
        };

        self.pages.get(&key)
            .map(|data| data.path.clone())
            .ok_or(DataError::PageNotFound)
    }

fn get_download_url(&self, section: &str, page: &str) -> Result<String, DataError> {
    let key = PageKey {
        section: section.to_string(),
        name: page.to_string(),
    };

    self.pages.get(&key)
        .and_then(|page_data| {
            page_data.file.as_ref().map(|file| {
                // Construct the full URL properly
                if let Some(base_path) = page_data.path.strip_suffix(&page) {
                    format!("{}{}{}", BASE_URL, base_path, file)
                } else {
                    format!("{}{}/{}", BASE_URL, page_data.path, file)
                }
            })
        })
        .ok_or(DataError::PageNotFound)
}

    // fn get_download_url(&self, section: &str, page: &str) -> Result<String, DataError> {
    //     let key = PageKey {
    //         section: section.to_string(),
    //         name: page.to_string(),
    //     };

    //     let path = self.get_page_path(section, page)?;
    //     self.pages.get(&key)
    //         .and_then(|page_data| match &page_data.file {
    //             Some(file) => Some(format!("{}{}{}", BASE_URL,path, file)),
    //             None => None,
    //         })
    //         .ok_or(DataError::PageNotFound)
    // }
}

pub fn markdown_to_html(markdown: &str) -> String {
    let ss = SYNTAX_SET.get().unwrap();
    let mut sr = ss.find_syntax_plain_text();
    let mut code = String::new();
    let mut code_block = false;
    let theme = &THEME_SET.get().unwrap().themes["base16-ocean.dark"];

    let parser = Parser::new(markdown).filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
            sr = ss.find_syntax_by_token(lang.trim()).unwrap_or(sr);
            code_block = true;
            None
        }
        Event::End(TagEnd::CodeBlock) => {
            let html = highlighted_html_for_string(&code, ss, sr, theme).unwrap_or(code.clone());
            code.clear();
            code_block = false;
            Some(Event::Html(html.into()))
        }
        Event::Text(t) if code_block => {
            code.push_str(&t);
            None
        }
        _ => Some(event),
    });

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

#[derive(Debug)]
pub enum DataError {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    Storage(gloo_storage::errors::StorageError),
    InvalidStructure,
    PageNotFound,
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Reqwest(e) => format!("Network error: {}", e),
            Self::Json(e) => format!("JSON error: {}", e),
            Self::Storage(e) => format!("Storage error: {}", e),
            Self::InvalidStructure => "Invalid data structure".into(),
            Self::PageNotFound => "Page not found".into(),
        })
    }
}

impl From<reqwest::Error> for DataError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<serde_json::Error> for DataError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<gloo_storage::errors::StorageError> for DataError {
    fn from(e: gloo_storage::errors::StorageError) -> Self {
        Self::Storage(e)
    }
}
