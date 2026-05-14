use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn VaultLock(on_unlock: EventHandler<String>) -> Element {
    let c = use_theme_colors();
    let mut password = use_signal(String::new);
    let mut focused = use_signal(|| false);
    let input_border = if *focused.read() { c.accent } else { c.border };

    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background: {c.bg_canvas}; padding: 0 16px; position: relative;",

            div { style: "position: absolute; top: 0; left: 0; right: 0; height: 3px; background: linear-gradient(90deg, {c.accent}, {c.accent_green});" }

            div { style: "display: flex; flex-direction: column; align-items: center; margin-bottom: 40px; text-align: center;",
                span { class: "material-symbols-outlined fill", style: "font-size: 52px; color: {c.accent}; margin-bottom: 12px;", "lock" }
                h1 { style: "font-family: Inter; font-size: 32px; font-weight: 700; letter-spacing: -0.02em; color: {c.accent};", "Midnight Notes" }
                p { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; letter-spacing: 0.05em; color: {c.text_secondary}; margin-top: 6px;",
                    "Local-first Sync"
                    span { style: "width: 3px; height: 3px; background: {c.accent_green}; border-radius: 50%; display: inline-block; margin: 0 4px;" }
                    "End-to-End Encrypted"
                }
            }

            div { style: "width: 100%; max-width: 360px; background: {c.bg_surface_low}; border: 1px solid {c.border}; border-radius: 8px; padding: 24px; display: flex; flex-direction: column; gap: 20px;",
                button { style: "width: 100%; background: {c.accent}; color: {c.bg_primary}; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; letter-spacing: 0.05em; padding: 14px 0; border-radius: 8px; border: none; cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 6px;",
                    onclick: move |_| on_unlock.call(password.read().clone()),
                    span { class: "material-symbols-outlined fill", style: "font-size: 28px;", "fingerprint" }
                    "Unlock with Biometrics"
                }
                div { style: "display: flex; align-items: center;",
                    div { style: "flex-grow: 1; border-top: 1px solid {c.border};" }
                    span { style: "padding: 0 12px; font-family: 'JetBrains Mono', monospace; font-size: 10px; letter-spacing: 0.05em; color: {c.text_muted};", "OR" }
                    div { style: "flex-grow: 1; border-top: 1px solid {c.border};" }
                }
                div { style: "display: flex; flex-direction: column; gap: 12px;",
                    label { r#for: "master-password", style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500; letter-spacing: 0.05em; color: {c.text_muted}; display: flex; align-items: center; gap: 4px; text-transform: uppercase;",
                        span { class: "material-symbols-outlined", style: "font-size: 14px;", "key" } "Master Password"
                    }
                    input { r#type: "password", id: "master-password", placeholder: "Enter your master password",
                        value: "{password}", oninput: move |e| password.set(e.value()), onfocus: move |_| focused.set(true), onblur: move |_| focused.set(false),
                        style: "width: 100%; background: {c.bg_surface}; color: {c.text_primary}; font-family: 'JetBrains Mono', monospace; font-size: 13px; padding: 10px 8px; border: none; border-bottom: 1px solid {input_border}; outline: none;",
                    }
                    button { style: "width: 100%; background: transparent; border: 1px solid {c.border}; color: {c.text_primary}; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; letter-spacing: 0.05em; padding: 10px; border-radius: 4px; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 8px; margin-top: 4px;",
                        onclick: move |_| on_unlock.call(password.read().clone()),
                        span { "Unlock Vault" } span { class: "material-symbols-outlined", style: "font-size: 14px;", "arrow_forward" }
                    }
                }
            }

            div { style: "margin-top: 32px; text-align: center;",
                p { style: "font-family: 'JetBrains Mono', monospace; font-size: 9px; color: {c.text_muted}; max-width: 280px; line-height: 1.5; margin: 0 auto;",
                    "Vaults are zero-knowledge. We cannot recover your data if you lose your master password or biometric access." }
            }
        }
    }
}
