// src/theme.rs
use dioxus::prelude::*;

/// The current theme
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

/// Initialize the global theme signal (call once in App)
pub fn use_theme_provider() -> Signal<Theme> {
    use_context_provider(|| {
        let initial = if cfg!(target_family = "wasm") {
            // Web platform – try to load from localStorage
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(saved)) = storage.get_item("theme") {
                        if saved == "dark" {
                            Theme::Dark
                        } else {
                            Theme::Light
                        }
                    } else {
                        Theme::Dark
                    }
                } else {
                    Theme::Dark
                }
            } else {
                Theme::Dark
            }
        } else {
            Theme::Dark
        };

        Signal::new(initial)
    })
}

/// Get the current theme signal from anywhere
pub fn use_theme() -> Signal<Theme> {
    use_context()
}

/*
pub fn toggle_theme() {
    let mut theme = use_theme();
    let new_theme = if *theme.read() == Theme::Dark {
        Theme::Light
    } else {
        Theme::Dark
    };

    *theme.write() = new_theme;

    // Only run DOM code on web
    if cfg!(target_family = "wasm") {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Fix: document_element() returns Result<HtmlElement, JsValue>
                if let Ok(Some(html)) = document.document_element() {
                    if new_theme == Theme::Dark {
                        let _ = html.class_list().add_1("dark");
                    } else {
                        let _ = html.class_list().remove_1("dark");
                    }
                }
            }

            // Save preference
            if let Ok(Some(storage)) = window.local_storage() {
                let value = if new_theme == Theme::Dark { "dark" } else { "light" };
                let _ = storage.set_item("theme", value);
            }
        }
    }
}

/// Apply the correct class on initial load (prevents flash)
pub fn use_apply_theme() {
    let theme = use_theme();
    use_effect(move || {
        if cfg!(target_family = "wasm") {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Ok(Some(html)) = document.document_element() {
                        if *theme.read() == Theme::Dark {
                            let _ = html.class_list().add_1("dark");
                        } else {
                            let _ = html.class_list().remove_1("dark");
                        }
                    }
                }
            }
        }
    });
}
*/