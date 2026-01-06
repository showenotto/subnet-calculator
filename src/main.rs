// src/main.rs
mod app;
mod components;
mod ipv4;
mod ipv6;
mod common;
mod assistant;

use dioxus::prelude::*;
use app::App;

fn main() {
    launch(App);
}