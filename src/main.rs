#![allow(non_snake_case)]

use dioxus::{
    logger::{
        self,
        tracing::{self, Level},
    },
    prelude::*,
};

use syntect::{
    highlighting::ThemeSet,
    parsing::{SyntaxDefinition, SyntaxSet},
};
use ui::{App, SYNTAX_SET, THEME_SET};

fn main() {
    logger::init(Level::INFO).expect("failed to init logger");
    SYNTAX_SET.get_or_init(|| {
        let nix_syntax = include_str!("../syntax/nix.sublime-syntax");
        let syntax = SyntaxDefinition::load_from_str(nix_syntax, true, None)
            .expect("Failed to load Nix syntax");

        let mut builder = SyntaxSet::load_defaults_newlines().into_builder();
        builder.add(syntax);
        builder.build()
    });

    THEME_SET.get_or_init(ThemeSet::load_defaults);

    tracing::debug!("Rendering app!");
    launch(App);
}
