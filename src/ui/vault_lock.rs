use dioxus::prelude::*;

#[component]
pub fn VaultLock(on_unlock: EventHandler<String>) -> Element {
    let mut password = use_signal(String::new);
    let mut focused = use_signal(|| false);
    let input_border = if *focused.read() {
        "#00dbe9"
    } else {
        "#3b494b"
    };

    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background: #0e0e0e; padding: 0 16px; position: relative;",

            // Accent bar at top
            div { style: "position: absolute; top: 0; left: 0; right: 0; height: 3px; background: linear-gradient(90deg, #00dbe9, #00e475);" }

            // Branding
            div { style: "display: flex; flex-direction: column; align-items: center; margin-bottom: 40px; text-align: center;",
                span { class: "material-symbols-outlined fill", style: "font-size: 52px; color: #00dbe9; margin-bottom: 12px;", "lock" }
                h1 { style: "font-family: Inter; font-size: 32px; font-weight: 700; line-height: 40px; letter-spacing: -0.02em; color: #00dbe9;", "Midnight Notes" }
                p { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; letter-spacing: 0.05em; color: #b9cacb; margin-top: 6px; display: flex; align-items: center; gap: 6px;",
                    "Local-first Sync"
                    span { style: "width: 3px; height: 3px; background: #00e475; border-radius: 50%; display: inline-block;" }
                    "End-to-End Encrypted"
                }
            }

            // Auth card
            div { style: "width: 100%; max-width: 360px; background: #1c1b1b; border: 1px solid #3b494b; border-radius: 8px; padding: 24px; display: flex; flex-direction: column; gap: 20px;",

                // Biometric
                button { style: "width: 100%; background: #00dbe9; color: #002022; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; letter-spacing: 0.05em; padding: 14px 0; border-radius: 8px; border: none; cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 6px; transition: all 0.1s;",
                    onclick: move |_| on_unlock.call(password.read().clone()),
                    span { class: "material-symbols-outlined fill", style: "font-size: 28px;", "fingerprint" }
                    "Unlock with Biometrics"
                }

                // Divider
                div { style: "display: flex; align-items: center; width: 100%;",
                    div { style: "flex-grow: 1; border-top: 1px solid #3b494b;" }
                    span { style: "padding: 0 12px; font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500; letter-spacing: 0.05em; color: #849495;", "OR" }
                    div { style: "flex-grow: 1; border-top: 1px solid #3b494b;" }
                }

                // Password form
                div { style: "display: flex; flex-direction: column; gap: 12px;",
                    label { r#for: "master-password",
                        style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 500; letter-spacing: 0.05em; color: #849495; display: flex; align-items: center; gap: 4px; text-transform: uppercase;",
                        span { class: "material-symbols-outlined", style: "font-size: 14px;", "key" }
                        "Master Password"
                    }
                    input { r#type: "password", id: "master-password", placeholder: "Enter your master password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                        onfocus: move |_| focused.set(true),
                        onblur: move |_| focused.set(false),
                        style: "width: 100%; background: #131313; color: #e5e2e1; font-family: 'JetBrains Mono', monospace; font-size: 13px; padding: 10px 8px; border: none; border-bottom: 1px solid {input_border}; transition: border-color 0.15s; outline: none;",
                    }
                    button { style: "width: 100%; background: transparent; border: 1px solid #3b494b; color: #e5e2e1; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; letter-spacing: 0.05em; padding: 10px; border-radius: 4px; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 8px; transition: all 0.1s; margin-top: 4px;",
                        onclick: move |_| { on_unlock.call(password.read().clone()); },
                        span { "Unlock Vault" }
                        span { class: "material-symbols-outlined", style: "font-size: 14px;", "arrow_forward" }
                    }
                }
            }

            // Info
            div { style: "margin-top: 32px; text-align: center;",
                p { style: "font-family: 'JetBrains Mono', monospace; font-size: 9px; color: #849495; max-width: 280px; line-height: 1.5; margin: 0 auto;",
                    "Vaults are zero-knowledge. We cannot recover your data if you lose your master password or biometric access." }
            }
        }
    }
}
