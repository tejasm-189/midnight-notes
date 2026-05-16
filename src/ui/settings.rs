use crate::ui::app::SharedDb;
use crate::ui::theme::{use_theme_colors, use_theme_signal, Theme, ThemeColors};
use dioxus::prelude::*;

#[component]
pub fn Settings(db: Option<SharedDb>) -> Element {
    let c = use_theme_colors();
    let theme = use_theme_signal();

    let mut theme_mid = theme;
    let mut theme_dark = theme;
    let mut theme_light = theme;
    let db_mid = db.clone();
    let db_dark = db.clone();
    let db_light = db.clone();

    rsx! {
        div { style: "flex: 1; overflow-y: auto; background: {c.bg_canvas}; display: flex; flex-direction: column;",

            // Integrated Header (Minimalist)
            div { style: "padding: 32px 40px; border-bottom: 1px solid {c.border}; background: {c.bg_primary};",
                h1 { style: "font-family: Inter; font-size: 28px; font-weight: 700; color: {c.text_primary}; letter-spacing: -0.02em;", "Settings & Security" }
                p { style: "font-family: Inter; font-size: 14px; color: {c.text_secondary}; margin-top: 4px;", "Manage your vault encryption, appearance, and local storage." }
            }

            // Sections List (Centered to match Editor)
            div { style: "width: 100%; max-width: 1200px; margin: 0 auto; padding: 40px 40px 120px; display: flex; flex-direction: column; gap: 48px;",

                // Encryption Section
                SettingsSection { icon: "security", title: "Encryption Engine", c: c,
                    div { style: "background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 4px; overflow: hidden;",
                        SettingsItem { title: "Key Management", desc: "View and cycle your active XChaCha20-Poly1305 encryption keys.", c: c,
                            button { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 600; padding: 8px 16px; border: 1px solid {c.border}; color: {c.text_primary}; border-radius: 2px; background: {c.bg_surface_low}; cursor: pointer; display: flex; align-items: center; gap: 8px;",
                                span { class: "material-symbols-outlined", style: "font-size: 16px;", "key" }
                                "Manage Keys"
                            }
                        }
                        div { style: "height: 1px; background: {c.border};" }
                        SettingsItem { title: "Argon2id Parameters", desc: "Adjust memory and iteration costs for key derivation.", c: c,
                            div { style: "display: flex; align-items: center; gap: 12px;",
                                span { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; color: {c.text_muted}; background: {c.bg_canvas}; padding: 4px 8px; border-radius: 2px;", "m=65536, t=3, p=4" }
                                button { style: "background: none; border: none; color: {c.accent}; cursor: pointer;",
                                    span { class: "material-symbols-outlined", style: "font-size: 18px;", "settings_suggest" }
                                }
                            }
                        }
                    }
                }

                // Appearance Section
                SettingsSection { icon: "palette", title: "Appearance", c: c,
                    div { style: "background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 4px; padding: 24px;",
                        div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 16px;",
                            ThemeCard { t: Theme::Light, active: *theme_light.read() == Theme::Light, c: c, onclick: move |_| { theme_light.set(Theme::Light); save_theme(&db_light, Theme::Light); } }
                            ThemeCard { t: Theme::Dark, active: *theme_dark.read() == Theme::Dark, c: c, onclick: move |_| { theme_dark.set(Theme::Dark); save_theme(&db_dark, Theme::Dark); } }
                            ThemeCard { t: Theme::Midnight, active: *theme_mid.read() == Theme::Midnight, c: c, onclick: move |_| { theme_mid.set(Theme::Midnight); save_theme(&db_mid, Theme::Midnight); } }
                        }
                    }
                }

                // Sync & Backup Section
                SettingsSection { icon: "sync", title: "Storage & Backup", c: c,
                    div { style: "background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 4px; overflow: hidden;",
                        SettingsItem { title: "Vault Location", desc: "Root directory for encrypted .mnotes files.", c: c,
                            div { style: "display: flex; align-items: center; gap: 8px; background: {c.bg_canvas}; padding: 6px 12px; border-radius: 2px; border: 1px solid {c.border};",
                                span { class: "material-symbols-outlined", style: "font-size: 16px; color: {c.accent};", "folder_open" }
                                code { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; color: {c.text_primary};", "~/Documents/MidnightVault" }
                            }
                        }
                        div { style: "height: 1px; background: {c.border};" }
                        SettingsItem { title: "Encrypted Export", desc: "Generate a portable archive of your entire vault.", c: c,
                            button { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; padding: 10px 20px; background: {c.accent}; color: {c.bg_primary}; border: none; border-radius: 4px; cursor: pointer; display: flex; align-items: center; gap: 8px; font-weight: 700;",
                                span { class: "material-symbols-outlined", style: "font-size: 18px;", "cloud_download" }
                                "Export Now"
                            }
                        }
                    }
                }

                // Biometrics Section
                SettingsSection { icon: "fingerprint", title: "Security", c: c,
                    div { style: "background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 4px; padding: 24px; display: flex; align-items: center; justify-content: space-between;",
                        div {
                            h4 { style: "font-family: Inter; font-size: 16px; font-weight: 600; color: {c.text_primary};", "Hardware Unlock" }
                            p { style: "font-family: Inter; font-size: 13px; color: {c.text_secondary}; margin-top: 2px;", "Use system biometrics to unlock the vault." }
                        }
                        Toggle { active: true, c: c }
                    }
                }
            }

            // Minimal Footer
            div { style: "margin-top: auto; padding: 24px 40px; border-top: 1px solid {c.border}; display: flex; gap: 24px; font-family: 'JetBrains Mono', monospace; font-size: 10px; color: {c.text_muted};",
                span { "Version: 0.1.0" }
                span { "Engine: XChaCha20-Poly1305" }
                span { "KDF: Argon2id" }
            }
        }
    }
}

#[component]
fn SettingsSection(icon: String, title: String, c: ThemeColors, children: Element) -> Element {
    rsx! {
        section {
            div { style: "margin-bottom: 16px; padding-bottom: 8px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; gap: 10px;",
                span { class: "material-symbols-outlined", style: "font-size: 20px; color: {c.accent};", "{icon}" }
                h3 { style: "font-family: Inter; font-size: 18px; font-weight: 700; color: {c.text_primary};", "{title}" }
            }
            {children}
        }
    }
}

#[component]
fn SettingsItem(title: String, desc: String, c: ThemeColors, children: Element) -> Element {
    rsx! {
        div {
            style: "padding: 20px 24px; display: flex; align-items: center; justify-content: space-between; gap: 24px;",
            div { style: "flex: 1;",
                h4 { style: "font-family: Inter; font-size: 15px; font-weight: 600; color: {c.text_primary};", "{title}" }
                p { style: "font-family: Inter; font-size: 13px; color: {c.text_secondary}; margin-top: 2px;", "{desc}" }
            }
            {children}
        }
    }
}

#[component]
fn ThemeCard(t: Theme, active: bool, c: ThemeColors, onclick: EventHandler<()>) -> Element {
    let border = if active {
        format!("2px solid {}", c.accent)
    } else {
        format!("1px solid {}", c.border)
    };
    let bg = if active {
        c.bg_surface_high
    } else {
        c.bg_surface_low
    };
    let icon_color = if active { c.accent } else { c.text_secondary };

    rsx! {
        div {
            style: "border: {border}; border-radius: 4px; padding: 16px; display: flex; flex-direction: column; align-items: center; gap: 8px; cursor: pointer; background: {bg}; transition: all 0.1s;",
            onclick: move |_| onclick.call(()),
            span { class: "material-symbols-outlined", style: "font-size: 24px; color: {icon_color};", "{t.icon()}" }
            span { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; color: {icon_color}; font-weight: 700; text-transform: uppercase;", "{t.label()}" }
        }
    }
}

#[component]
fn Toggle(active: bool, c: ThemeColors) -> Element {
    let bg = if active { c.accent } else { c.bg_surface_high };
    let translate = if active {
        "translateX(18px)"
    } else {
        "translateX(0)"
    };
    rsx! {
        div { style: "width: 42px; height: 24px; background: {bg}; border: 1px solid {c.border}; border-radius: 4px; padding: 2px; cursor: pointer; transition: background 0.3s; position: relative;",
            div { style: "width: 18px; height: 18px; background: {c.bg_primary}; border-radius: 2px; transition: transform 0.2s; transform: {translate};" }
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
