use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

pub struct FmtAction {
    pub label: &'static str,
    pub icon: &'static str,
    pub insert_before: &'static str,
    pub insert_after: &'static str,
    pub newline: bool,
}

pub const FORMAT_ACTIONS: &[FmtAction] = &[
    FmtAction {
        label: "Heading 1",
        icon: "title",
        insert_before: "# ",
        insert_after: "",
        newline: true,
    },
    FmtAction {
        label: "Heading 2",
        icon: "title",
        insert_before: "## ",
        insert_after: "",
        newline: true,
    },
    FmtAction {
        label: "Heading 3",
        icon: "title",
        insert_before: "### ",
        insert_after: "",
        newline: true,
    },
    FmtAction {
        label: "Bold",
        icon: "format_bold",
        insert_before: "**",
        insert_after: "**",
        newline: false,
    },
    FmtAction {
        label: "Italic",
        icon: "format_italic",
        insert_before: "*",
        insert_after: "*",
        newline: false,
    },
    FmtAction {
        label: "Code",
        icon: "code",
        insert_before: "`",
        insert_after: "`",
        newline: false,
    },
    FmtAction {
        label: "Code Block",
        icon: "data_object",
        insert_before: "```\n",
        insert_after: "\n```",
        newline: true,
    },
    FmtAction {
        label: "Quote",
        icon: "format_quote",
        insert_before: "> ",
        insert_after: "",
        newline: true,
    },
    FmtAction {
        label: "Bullet List",
        icon: "list",
        insert_before: "- ",
        insert_after: "",
        newline: true,
    },
    FmtAction {
        label: "Numbered List",
        icon: "format_list_numbered",
        insert_before: "1. ",
        insert_after: "",
        newline: true,
    },
    FmtAction {
        label: "Task List",
        icon: "checklist",
        insert_before: "- [ ] ",
        insert_after: "",
        newline: true,
    },
    FmtAction {
        label: "Horizontal Rule",
        icon: "horizontal_rule",
        insert_before: "\n---\n",
        insert_after: "",
        newline: true,
    },
    FmtAction {
        label: "Table",
        icon: "table_chart",
        insert_before: "\n| Column 1 | Column 2 |\n| --- | --- |\n|  |  |\n",
        insert_after: "",
        newline: true,
    },
];

#[component]
pub fn FormatMenu(
    visible: bool,
    on_close: EventHandler<()>,
    on_action: EventHandler<(String, String)>,
    x: i32,
    y: i32,
) -> Element {
    let c = use_theme_colors();
    if !visible {
        return rsx! {};
    }

    rsx! {
        style {
            r#"
            .ctx-menu-item {{
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 6px 12px;
                cursor: pointer;
                font-size: 13px;
                color: var(--text-secondary);
                font-family: Inter, sans-serif;
                transition: all 0.1s ease;
            }}
            .ctx-menu-item:hover {{
                background: var(--bg-surface-high);
                color: var(--accent);
            }}
            "#
        }
        div {
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 9999;",
            onclick: move |_| on_close.call(()),
            oncontextmenu: move |e| { e.prevent_default(); on_close.call(()); },
            div {
                style: "position: absolute; top: {y}px; left: {x}px; background: {c.bg_surface_container};
                        border: 1px solid {c.border}; border-radius: 6px; padding: 6px 0;
                        min-width: 200px; box-shadow: 0 8px 24px rgba(0,0,0,0.5); z-index: 10000;
                        max-height: 420px; overflow-y: auto;",

                // Editor Modes Group
                div { style: "padding: 4px 12px 6px; font-size: 10px; font-weight: 700; color: {c.text_muted}; text-transform: uppercase; font-family: Inter, sans-serif; letter-spacing: 0.05em;",
                    "Editor Mode"
                }

                {vec![
                    ("RichText", "edit", "RichText"),
                    ("Code Editor", "code", "Code"),
                    ("Vim Editor", "keyboard", "Vim"),
                    ("Spreadsheet", "table_chart", "Sheet"),
                    ("Kanban Board", "dashboard", "Board"),
                ].into_iter().map(|(label, icon, m)| {
                    let lbl = format!("Mode: {}", m);
                    rsx! {
                        div {
                            key: "{m}",
                            class: "ctx-menu-item",
                            onclick: move |_| on_action.call((lbl.clone(), String::new())),
                            span { class: "material-symbols-outlined", style: "font-size: 16px; color: {c.accent_green};", "{icon}" }
                            span { "{label}" }
                        }
                    }
                })}

                div { style: "height: 1px; background: {c.border}; margin: 6px 0;" }

                // Formatting Options Group
                div { style: "padding: 4px 12px 6px; font-size: 10px; font-weight: 700; color: {c.text_muted}; text-transform: uppercase; font-family: Inter, sans-serif; letter-spacing: 0.05em;",
                    "Formatting"
                }

                {FORMAT_ACTIONS.iter().map(|action| {
                    let label = action.label.to_string();
                    let ins = format!("{}{}", action.insert_before, action.insert_after);
                    rsx! {
                        div {
                            key: "{action.label}",
                            class: "ctx-menu-item",
                            onclick: move |_| on_action.call((label.clone(), ins.clone())),
                            span { class: "material-symbols-outlined", style: "font-size: 16px; color: {c.accent};", "{action.icon}" }
                            span { "{action.label}" }
                        }
                    }
                })}
            }
        }
    }
}
