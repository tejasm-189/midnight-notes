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
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center;
                    height: 100vh; background: #0e0e0e; padding: 0 16px;",

            div {
                style: "display: flex; flex-direction: column; align-items: center; margin-bottom: 40px; text-align: center;",
                span {
                    class: "material-symbols-outlined fill",
                    style: "font-size: 48px; color: #00dbe9; margin-bottom: 8px;",
                    "lock"
                }
                h1 { style: "font-family: Inter; font-size: 32px; font-weight: 700; line-height: 40px; letter-spacing: -0.02em; color: #00dbe9;", "Midnight Notes" }
                p { style: "font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; letter-spacing: 0.05em; color: #b9cacb; margin-top: 4px; display: flex; align-items: center; gap: 4px;",
                    "Local-first Sync",
                    span { style: "width: 4px; height: 4px; background: #00e475; border-radius: 50%; display: inline-block;" },
                    "End-to-End Encrypted"
                }
            }

            div {
                style: "width: 100%; max-width: 384px; background: #1c1b1b; border: 1px solid #3b494b; border-radius: 8px; padding: 24px; display: flex; flex-direction: column; gap: 24px;",

                button {
                    style: "width: 100%; background: #00dbe9; color: #002022; font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; letter-spacing: 0.05em; padding: 12px 0; border-radius: 8px; border: none; cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 8px;",
                    onclick: move |_| on_unlock.call(password.read().clone()),
                    span { class: "material-symbols-outlined fill", style: "font-size: 32px;", "fingerprint" }
                    span { "Unlock with Biometrics" }
                }

                div {
                    style: "display: flex; align-items: center; width: 100%;",
                    div { style: "flex-grow: 1; border-top: 1px solid #3b494b;" }
                    span { style: "padding: 0 16px; font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; letter-spacing: 0.05em; color: #b9cacb;", "OR" }
                    div { style: "flex-grow: 1; border-top: 1px solid #3b494b;" }
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label { r#for: "master-password",
                            style: "font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; letter-spacing: 0.05em; color: #b9cacb; display: flex; align-items: center; gap: 4px;",
                            span { class: "material-symbols-outlined", style: "font-size: 16px;", "key" }
                            "Master Password"
                        }
                        input {
                            r#type: "password", id: "master-password", placeholder: "••••••••••••••••",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                            onfocus: move |_| focused.set(true),
                            onblur: move |_| focused.set(false),
                            style: "width: 100%; background: #131313; color: #e5e2e1; font-family: 'JetBrains Mono', monospace; font-size: 14px; padding: 8px; border: none; border-bottom: 1px solid {input_border};",
                        }
                    }
                    button {
                        style: "width: 100%; background: transparent; border: 1px solid #3b494b; color: #e5e2e1; font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; letter-spacing: 0.05em; padding: 8px; border-radius: 4px; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 8px;",
                        onclick: move |_| on_unlock.call(password.read().clone()),
                        span { "Unlock Vault" }
                        span { class: "material-symbols-outlined", style: "font-size: 16px;", "arrow_forward" }
                    }
                }
            }

            div {
                style: "margin-top: 24px; text-align: center;",
                p { style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; color: #849495; margin-top: 8px; max-width: 320px; line-height: 1.4; margin-left: auto; margin-right: auto;",
                    "Vaults are zero-knowledge. We cannot recover your data if you lose your master password or biometric access." }
            }
        }
    }
}
