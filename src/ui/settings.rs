use dioxus::prelude::*;

use crate::ui::app::SharedDb;
use crate::ui::theme::{use_theme_signal, Theme};

#[component]
pub fn Settings(db: Option<SharedDb>, on_close: EventHandler<()>) -> Element {
    let theme = use_theme_signal();

    // Load saved theme from meta
    let db_for_load = db.clone();
    let mut theme_signal = theme;
    use_effect(move || {
        if let Some(ref db_val) = db_for_load {
            let conn = db_val.conn();
            if let Ok(saved) =
                conn.query_row("SELECT value FROM meta WHERE key = 'theme'", [], |row| {
                    row.get::<_, String>(0)
                })
            {
                let t = match saved.as_str() {
                    "dark" => Theme::Dark,
                    "light" => Theme::Light,
                    _ => Theme::Midnight,
                };
                theme_signal.set(t);
            }
        }
    });

    // Create clones for each closure
    let mut theme_mid = theme;
    let mut theme_dark = theme;
    let mut theme_light = theme;
    let db_mid = db.clone();
    let db_dark = db.clone();
    let db_light = db.clone();

    rsx! {
        div { style: "display: flex; flex-direction: column; height: 100vh; background: var(--bg-surface);",
            header { style: "height: 64px; border-bottom: 1px solid var(--border); display: flex; align-items: center; justify-content: space-between; padding: 0 24px;",
                h2 { style: "font-family: Inter; font-size: 24px; font-weight: 600; color: var(--text-primary);", "Settings" }
                button {
                    style: "background: none; border: 1px solid var(--border); color: var(--text-secondary); padding: 6px 12px; border-radius: 4px; cursor: pointer; font-family: 'JetBrains Mono', monospace; font-size: 12px;",
                    onclick: move |_| on_close.call(()),
                    "Close"
                }
            }
            div { style: "flex: 1; overflow-y: auto; padding: 24px; max-width: 640px; margin: 0 auto; width: 100%;",
                section { style: "margin-bottom: 40px;",
                    h3 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: var(--text-primary); margin-bottom: 16px; display: flex; align-items: center; gap: 8px;",
                        span { class: "material-symbols-outlined", style: "font-size: 20px; color: var(--accent);", "palette" }
                        "Appearance"
                    }
                    p { style: "font-family: Inter; font-size: 14px; color: var(--text-secondary); margin-bottom: 16px;", "Select your preferred theme." }
                    div { style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px;",
                        ThemeOption { t: Theme::Light, current: *theme_light.read() == Theme::Light, onselect: move |_| { theme_light.set(Theme::Light); save_theme(&db_light, Theme::Light); } }
                        ThemeOption { t: Theme::Dark, current: *theme_dark.read() == Theme::Dark, onselect: move |_| { theme_dark.set(Theme::Dark); save_theme(&db_dark, Theme::Dark); } }
                        ThemeOption { t: Theme::Midnight, current: *theme_mid.read() == Theme::Midnight, onselect: move |_| { theme_mid.set(Theme::Midnight); save_theme(&db_mid, Theme::Midnight); } }
                    }
                }
                section {
                    h3 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: var(--text-primary); margin-bottom: 12px; display: flex; align-items: center; gap: 8px;",
                        span { class: "material-symbols-outlined", style: "font-size: 20px; color: var(--accent);", "info" }
                        "About"
                    }
                    div { style: "background: var(--bg-surface-container); border: 1px solid var(--border); border-radius: 8px; padding: 16px;",
                        p { style: "font-family: Inter; font-size: 14px; color: var(--text-secondary);", "Midnight Notes v0.1.0" }
                        p { style: "font-family: Inter; font-size: 14px; color: var(--text-secondary); margin-top: 8px;", "Local-first, end-to-end encrypted notes." }
                    }
                }
            }
        }
    }
}

fn save_theme(db: &Option<SharedDb>, theme: Theme) {
    if let Some(ref db_val) = db {
        let val = match theme {
            Theme::Midnight => "midnight",
            Theme::Dark => "dark",
            Theme::Light => "light",
        };
        let conn = db_val.conn();
        let _ = conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES ('theme', ?1)",
            rusqlite::params![val],
        );
    }
}

#[component]
fn ThemeOption(t: Theme, current: bool, onselect: EventHandler<()>) -> Element {
    let border = if current {
        "2px solid var(--accent)"
    } else {
        "1px solid var(--border)"
    };
    let bg = if current {
        "var(--bg-surface-high)"
    } else {
        "var(--bg-canvas)"
    };
    let label_color = if current {
        "var(--accent)"
    } else {
        "var(--text-secondary)"
    };

    rsx! {
        div {
            style: "border: {border}; border-radius: 8px; padding: 16px; background: {bg}; cursor: pointer; text-align: center; transition: all 0.15s;",
            onclick: move |_| onselect.call(()),
            span { class: "material-symbols-outlined", style: "font-size: 32px; color: var(--accent); display: block; margin-bottom: 8px;", "{t.icon()}" }
            div { style: "font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; color: {label_color};", "{t.label()}" }
        }
    }
}
