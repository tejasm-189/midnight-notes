use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn ProseEditor(content: String, oninput: EventHandler<String>) -> Element {
    let c = use_theme_colors();

    rsx! {
        div { style: "flex: 1; display: flex; flex-direction: column; min-height: 0;",
            textarea {
                value: "{content}",
                oninput: move |e| oninput.call(e.value()),
                placeholder: "Start writing...",
                style: "flex: 1; background: transparent; border: none; color: {c.text_primary};
                        font-family: Inter, -apple-system, sans-serif; font-size: 18px; line-height: 1.8;
                        resize: none; outline: none; padding: 0; min-height: 500px;",
            }
        }
    }
}
