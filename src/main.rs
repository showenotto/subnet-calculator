// src/main.rs
mod app;
mod components;
mod theme;
mod ipv4;
mod ipv6;

use dioxus::prelude::*;
use app::App;

fn main() {
    launch(App);
}