use dioxus::prelude::*;

use crate::ui::app::SharedDb;
use crate::ui::theme::{use_theme_colors, use_theme_signal, Theme};

#[component]
pub fn Settings(db: Option<SharedDb>, on_close: EventHandler<()>) -> Element {
    let c = use_theme_colors();
    let theme = use_theme_signal();

    let mut theme_mid = theme;
    let mut theme_dark = theme;
    let mut theme_light = theme;
    let db_mid = db.clone();
    let db_dark = db.clone();
    let db_light = db.clone();

    let sidebar_item = |icon: &str, label: &str, active: bool| {
        let bg = if active {
            c.bg_surface_high
        } else {
            "transparent"
        };
        let color = if active { c.accent } else { c.text_secondary };
        let left = if active {
            format!("2px solid {}", c.accent)
        } else {
            "2px solid transparent".to_string()
        };
        rsx! {
            div { style: "display: flex; align-items: center; gap: 12px; padding: 8px 16px; background: {bg}; color: {color}; border-left: {left}; font-family: 'JetBrains Mono', monospace; font-size: 11px; cursor: pointer;",
                span { class: "material-symbols-outlined", style: "font-size: 18px;", "{icon}" }
                span { "{label}" }
            }
        }
    };

    rsx! {
        div { style: "display: flex; height: 100vh; background: {c.bg_primary};",

            // Sidebar matching DESIGN.md
            nav { style: "width: 256px; min-width: 256px; background: {c.bg_surface}; border-right: 1px solid {c.border}; display: flex; flex-direction: column; padding: 16px 0;",
                div { style: "padding: 0 16px; margin-bottom: 24px; display: flex; align-items: center; gap: 8px;",
                    span { class: "material-symbols-outlined fill", style: "font-size: 28px; color: {c.accent};", "lock" }
                    div { h1 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: {c.accent};", "Midnight Notes" } p { style: "font-size: 11px; color: {c.accent_green};", "Local-first Sync" } }
                }
                div { style: "flex: 1; display: flex; flex-direction: column; gap: 2px; padding: 0 8px;",
                    {sidebar_item("description", "All Notes", false)}
                    {sidebar_item("auto_awesome", "Smart Views", false)}
                    div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }
                    {sidebar_item("archive", "Archived", false)}
                    {sidebar_item("delete", "Trash", false)}
                    {sidebar_item("settings", "Settings", true)}
                }
                div { style: "border-top: 1px solid {c.border}; padding: 8px;",
                    div { style: "display: flex; align-items: center; gap: 12px; padding: 8px 16px; color: {c.text_secondary}; font-family: 'JetBrains Mono', monospace; font-size: 11px; cursor: pointer;",
                        onclick: move |_| on_close.call(()),
                        span { class: "material-symbols-outlined", style: "font-size: 18px;", "arrow_back" }
                        span { "Back to Workspace" }
                    }
                }
            }

            // Settings content
            div { style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",
                div { style: "flex: 1; overflow-y: auto; padding: 32px 48px; max-width: 720px;",
                    h2 { style: "font-family: Inter; font-size: 28px; font-weight: 700; color: {c.text_primary}; letter-spacing: -0.02em; margin-bottom: 4px;", "Settings & Security" }
                    p { style: "font-family: Inter; font-size: 14px; color: {c.text_secondary}; margin-bottom: 40px;", "Manage your encryption keys, appearance, and sync preferences." }

                    // Theme section
                    section { style: "margin-bottom: 48px;",
                        h3 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: {c.text_primary}; margin-bottom: 16px; display: flex; align-items: center; gap: 8px; border-bottom: 1px solid {c.border}; padding-bottom: 8px;",
                            span { class: "material-symbols-outlined", style: "font-size: 20px; color: {c.accent};", "palette" } "Appearance"
                        }
                        p { style: "font-family: Inter; font-size: 14px; color: {c.text_secondary}; margin-bottom: 16px;", "Select your preferred theme." }
                        div { style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px;",
                            ThemeOpt { t: Theme::Light, cur: *theme_light.read() == Theme::Light, c: c, onclick: move |_| { theme_light.set(Theme::Light); save_theme(&db_light, Theme::Light); } }
                            ThemeOpt { t: Theme::Dark, cur: *theme_dark.read() == Theme::Dark, c: c, onclick: move |_| { theme_dark.set(Theme::Dark); save_theme(&db_dark, Theme::Dark); } }
                            ThemeOpt { t: Theme::Midnight, cur: *theme_mid.read() == Theme::Midnight, c: c, onclick: move |_| { theme_mid.set(Theme::Midnight); save_theme(&db_mid, Theme::Midnight); } }
                        }
                    }

                    // About
                    section { style: "margin-bottom: 48px;",
                        h3 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: {c.text_primary}; margin-bottom: 12px; display: flex; align-items: center; gap: 8px; border-bottom: 1px solid {c.border}; padding-bottom: 8px;",
                            span { class: "material-symbols-outlined", style: "font-size: 20px; color: {c.accent};", "info" } "About"
                        }
                        div { style: "background: {c.bg_surface_container}; border: 1px solid {c.border}; border-radius: 8px; padding: 16px;",
                            p { style: "font-family: Inter; font-size: 14px; color: {c.text_secondary};", "Midnight Notes v0.1.0" }
                            p { style: "font-family: Inter; font-size: 14px; color: {c.text_secondary}; margin-top: 8px;", "Local-first, end-to-end encrypted notes. 114 tests passing." }
                        }
                    }
                }
            }

            // Footer
            div { style: "position: fixed; bottom: 0; left: 256px; right: 0; height: 28px; background: {c.bg_canvas}; border-top: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between; padding: 0 16px; font-family: 'JetBrains Mono', monospace; font-size: 10px;",
                span { style: "font-weight: 700; color: {c.accent};", "End-to-End Encrypted" }
                div { style: "display: flex; gap: 16px; color: {c.text_muted};", span { "v0.1.0" } }
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
        let _ = db_val.conn().execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES ('theme', ?1)",
            rusqlite::params![val],
        );
    }
}

#[component]
fn ThemeOpt(
    t: Theme,
    cur: bool,
    c: crate::ui::theme::ThemeColors,
    onclick: EventHandler<()>,
) -> Element {
    let border = if cur {
        format!("2px solid {}", c.accent)
    } else {
        format!("1px solid {}", c.border)
    };
    let bg = if cur { c.bg_surface_high } else { c.bg_canvas };
    let label_c = if cur { c.accent } else { c.text_secondary };
    rsx! {
        div { style: "border: {border}; border-radius: 8px; padding: 16px; background: {bg}; cursor: pointer; text-align: center;",
            onclick: move |_| onclick.call(()),
            span { class: "material-symbols-outlined", style: "font-size: 32px; color: {c.accent}; display: block; margin-bottom: 8px;", "{t.icon()}" }
            div { style: "font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; color: {label_c};", "{t.label()}" }
        }
    }
}
