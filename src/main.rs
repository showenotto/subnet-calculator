// src/main.rs
mod app;
mod components;
mod ipv4;
mod ipv6;
mod common;
mod assistant;

use dioxus::{desktop::{Config, LogicalSize, WindowBuilder}, prelude::*};
use app::app;
use std::fs;
use std::env;

fn main() {
    
    let mut config = Config::default()
        .with_window(
            WindowBuilder::new()
                .with_title("Subnet Calculator")
                .with_resizable(true)
                .with_visible(false)
                .with_always_on_top(false)
                .with_inner_size(LogicalSize::new(1200, 800))
        );
    #[cfg(target_os = "windows")]
    {
        // Try to get the local app data directory
        if let Ok(app_data) = env::var("LOCALAPPDATA") {
            use std::path::PathBuf;

            let data_dir = PathBuf::from(app_data).join("com.showen.SubnetCalculator");
            
            // Ensure the directory exists
            let _ = fs::create_dir_all(&data_dir);
            
            // Update config only if we successfully got the path
            config = config.with_data_directory(data_dir);
        }

    }

    // Launch with the specific config
    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(app);
}