// src/main.rs
mod app;
mod components;
mod ipv4;
mod ipv6;
mod common;
mod assistant;

use dioxus::{desktop::{Config, LogicalSize, WindowBuilder}, prelude::*};
use app::app;

fn main() {
    //launch(App);
    let config = Config::default()
        .with_window(
            WindowBuilder::new()
                .with_title("Subnet Calculator")
                .with_resizable(true)
                .with_visible(false)
                .with_always_on_top(false)
                .with_background_color([17, 24, 39, 255].into())
                .with_inner_size(LogicalSize::new(1200, 800))
        );

    // Launch with the specific config
    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(app);
}