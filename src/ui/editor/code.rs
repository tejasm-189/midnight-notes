use dioxus::prelude::*;

#[component]
pub fn CodeEditor(
    content: String,
    oninput: EventHandler<String>,
    title: String,
    ontitleinput: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            style: "flex: 1; display: flex; flex-direction: column; min-height: 0;",

            input {
                r#type: "text",
                value: "{title}",
                oninput: move |e| ontitleinput.call(e.value()),
                placeholder: "Untitled",
                style: "width: 100%; background: transparent; border: none;
                        font-family: Inter; font-size: 32px; font-weight: 700;
                        letter-spacing: -0.02em; color: #e5e2e1;
                        padding: 24px 0 16px; outline: none;",
            }

            textarea {
                value: "{content}",
                oninput: move |e| oninput.call(e.value()),
                placeholder: "// Start coding...",
                spellcheck: false,
                style: "flex: 1; width: 100%; background: #0a0a0a; border: 1px solid #3b494b; border-radius: 4px;
                        color: #e5e2e1; font-family: 'JetBrains Mono', monospace; font-size: 14px; line-height: 1.5;
                        resize: none; outline: none; padding: 16px; min-height: 200px;
                        tab-size: 2;",
            }
        }
    }
}
