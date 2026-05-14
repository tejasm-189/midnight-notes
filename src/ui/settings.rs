use dioxus::prelude::*;

use crate::ui::app::SharedDb;
use crate::ui::theme::{use_theme_colors, use_theme_signal, Theme};

#[component]
pub fn Settings(db: Option<SharedDb>, on_close: EventHandler<()>) -> Element {
    let c = use_theme_colors();
    let theme = use_theme_signal();

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

    let mut theme_mid = theme;
    let mut theme_dark = theme;
    let mut theme_light = theme;
    let db_mid = db.clone();
    let db_dark = db.clone();
    let db_light = db.clone();

    rsx! {
        div { style: "display: flex; flex-direction: column; height: 100vh; background: {c.bg_surface};",
            header { style: "height: 64px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between; padding: 0 24px;",
                h2 { style: "font-family: Inter; font-size: 24px; font-weight: 600; color: {c.text_primary};", "Settings" }
                button { style: "background: none; border: 1px solid {c.border}; color: {c.text_secondary}; padding: 6px 12px; border-radius: 4px; cursor: pointer; font-family: 'JetBrains Mono', monospace; font-size: 12px;",
                    onclick: move |_| on_close.call(()), "Close"
                }
            }
            div { style: "flex: 1; overflow-y: auto; padding: 24px; max-width: 640px; margin: 0 auto; width: 100%;",
                section { style: "margin-bottom: 40px;",
                    h3 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: {c.text_primary}; margin-bottom: 16px; display: flex; align-items: center; gap: 8px;",
                        span { class: "material-symbols-outlined", style: "font-size: 20px; color: {c.accent};", "palette" } "Appearance"
                    }
                    p { style: "font-family: Inter; font-size: 14px; color: {c.text_secondary}; margin-bottom: 16px;", "Select your preferred theme." }
                    div { style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px;",
                        ThemeOpt { t: Theme::Light, cur: *theme_light.read() == Theme::Light, c: c, onclick: move |_| { theme_light.set(Theme::Light); save_theme(&db_light, Theme::Light); } }
                        ThemeOpt { t: Theme::Dark, cur: *theme_dark.read() == Theme::Dark, c: c, onclick: move |_| { theme_dark.set(Theme::Dark); save_theme(&db_dark, Theme::Dark); } }
                        ThemeOpt { t: Theme::Midnight, cur: *theme_mid.read() == Theme::Midnight, c: c, onclick: move |_| { theme_mid.set(Theme::Midnight); save_theme(&db_mid, Theme::Midnight); } }
                    }
                }
                section {
                    h3 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: {c.text_primary}; margin-bottom: 12px; display: flex; align-items: center; gap: 8px;",
                        span { class: "material-symbols-outlined", style: "font-size: 20px; color: {c.accent};", "info" } "About"
                    }
                    div { style: "background: {c.bg_surface_container}; border: 1px solid {c.border}; border-radius: 8px; padding: 16px;",
                        p { style: "font-family: Inter; font-size: 14px; color: {c.text_secondary};", "Midnight Notes v0.1.0" }
                        p { style: "font-family: Inter; font-size: 14px; color: {c.text_secondary}; margin-top: 8px;", "Local-first, end-to-end encrypted notes." }
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
fn ThemeOpt(
    t: Theme,
    cur: bool,
    c: crate::ui::theme::ThemeColors,
    onclick: EventHandler<()>,
) -> Element {
    let border = if cur {
        "2px solid ".to_string() + c.accent
    } else {
        "1px solid ".to_string() + c.border
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
