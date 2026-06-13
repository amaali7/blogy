#![allow(non_snake_case)]

use dioxus::{
    logger::{
        self,
        tracing::{self, Level},
    },
    prelude::*,
};

use ui::App;

fn main() {
    logger::init(Level::INFO).expect("failed to init logger");
    tracing::debug!("Rendering app!");
    launch(App);
}
