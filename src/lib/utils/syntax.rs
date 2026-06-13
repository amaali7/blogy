use std::collections::HashSet;
use std::sync::{OnceLock, RwLock};

use crate::{SYNTAX_SET, THEME_SET};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use syntect::{
    highlighting::ThemeSet,
    parsing::{SyntaxDefinition, SyntaxSet, SyntaxSetBuilder},
};

static ATTEMPTED_LANGS: OnceLock<RwLock<HashSet<String>>> = OnceLock::new();

fn attempted_langs() -> &'static RwLock<HashSet<String>> {
    ATTEMPTED_LANGS.get_or_init(|| RwLock::new(HashSet::new()))
}

fn syntax_asset_url(lang: &str) -> Result<String, String> {
    let window = web_sys::window().ok_or("no window")?;
    let origin = window
        .location()
        .origin()
        .map_err(|_| "no origin".to_string())?;

    let base = dioxus::cli_config::base_path()
        .filter(|path| !path.is_empty())
        .map(|path| format!("/{}", path.trim_matches('/')))
        .unwrap_or_default();

    Ok(format!("{origin}{base}/assets/syntax/{lang}.sublime-syntax"))
}

fn sanitize_lang(lang: &str) -> Option<String> {
    let lang = lang.trim().to_lowercase();
    if lang.is_empty() || lang.len() > 32 {
        return None;
    }
    if !lang
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return None;
    }
    Some(lang)
}

pub fn collect_code_languages(markdown: &str) -> Vec<String> {
    let mut langs = Vec::new();
    for event in Parser::new(markdown) {
        if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) = event {
            if let Some(lang) = sanitize_lang(&lang) {
                if !langs.contains(&lang) {
                    langs.push(lang);
                }
            }
        }
    }
    langs
}

fn syntax_loaded(ss: &SyntaxSet, lang: &str) -> bool {
    ss.find_syntax_by_token(lang).is_some()
}

fn mark_attempted(lang: &str) {
    attempted_langs()
        .write()
        .unwrap_or_else(|e| e.into_inner())
        .insert(lang.to_string());
}

fn was_attempted(lang: &str) -> bool {
    attempted_langs()
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .contains(lang)
}

async fn ensure_syntax(lang: &str) -> Result<(), String> {
    {
        let ss = SYNTAX_SET
            .get()
            .ok_or("syntax highlighter not initialized")?
            .read()
            .unwrap_or_else(|e| e.into_inner());
        if syntax_loaded(&ss, lang) {
            return Ok(());
        }
    }

    if was_attempted(lang) {
        return Ok(());
    }

    let url = syntax_asset_url(lang)?;
    let response = match reqwest::get(&url).await {
        Ok(response) => response,
        Err(_) => {
            mark_attempted(lang);
            return Ok(());
        }
    };

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        mark_attempted(lang);
        return Ok(());
    }

    let content = match response.error_for_status() {
        Ok(response) => match response.text().await {
            Ok(content) => content,
            Err(_) => {
                mark_attempted(lang);
                return Ok(());
            }
        },
        Err(_) => {
            mark_attempted(lang);
            return Ok(());
        }
    };

    let syntax = match SyntaxDefinition::load_from_str(&content, true, None) {
        Ok(syntax) => syntax,
        Err(_) => {
            mark_attempted(lang);
            return Ok(());
        }
    };

    {
        let mut ss = SYNTAX_SET
            .get()
            .ok_or("syntax highlighter not initialized")?
            .write()
            .unwrap_or_else(|e| e.into_inner());
        let mut builder = ss.clone().into_builder();
        builder.add(syntax);
        *ss = builder.build();
    }

    mark_attempted(lang);
    Ok(())
}

pub async fn ensure_syntaxes_for_markdown(markdown: &str) -> Result<(), String> {
    for lang in collect_code_languages(markdown) {
        ensure_syntax(&lang).await?;
    }
    Ok(())
}

pub async fn init_syntax_highlighter() -> Result<(), String> {
    let mut builder = SyntaxSetBuilder::new();
    builder.add_plain_text_syntax();

    SYNTAX_SET.get_or_init(|| RwLock::new(builder.build()));
    attempted_langs();

    THEME_SET
        .set(ThemeSet::load_defaults())
        .map_err(|_| "theme set already initialized".to_string())?;

    Ok(())
}
