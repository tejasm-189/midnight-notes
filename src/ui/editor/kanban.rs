use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
struct KanbanColumn {
    title: String,
    tasks: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct KanbanData {
    columns: Vec<KanbanColumn>,
}

#[component]
pub fn KanbanEditor(
    content: String,
    oninput: EventHandler<String>,
    title: String,
    ontitleinput: EventHandler<String>,
) -> Element {
    let c = use_theme_colors();

    let mut data = use_signal(|| {
        serde_json::from_str::<KanbanData>(&content).unwrap_or_else(|_| KanbanData {
            columns: vec![
                KanbanColumn {
                    title: "Todo".into(),
                    tasks: vec![],
                },
                KanbanColumn {
                    title: "Doing".into(),
                    tasks: vec![],
                },
                KanbanColumn {
                    title: "Done".into(),
                    tasks: vec![],
                },
            ],
        })
    });

    let save = move |d: KanbanData| {
        if let Ok(json) = serde_json::to_string(&d) {
            oninput.call(json);
        }
    };

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px; height: 100%;",
            input { r#type: "text", value: "{title}", oninput: move |e| ontitleinput.call(e.value()),
                placeholder: "Board Title",
                style: "width: 100%; background: transparent; border: none; font-family: Inter; font-size: 32px; font-weight: 700; letter-spacing: -0.02em; color: {c.text_primary}; outline: none;",
            }

            div { style: "display: flex; gap: 16px; overflow-x: auto; flex: 1; padding-bottom: 20px; align-items: flex-start;",
                for (col_idx, column) in data.read().columns.iter().enumerate() {
                    div { key: "{col_idx}", style: "min-width: 280px; max-width: 280px; background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 8px; display: flex; flex-direction: column; max-height: 100%;",
                        div { style: "padding: 12px; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between;",
                            input {
                                style: "background: transparent; border: none; color: {c.text_primary}; font-family: Inter; font-weight: 600; font-size: 14px; outline: none; width: 80%;",
                                value: "{column.title}",
                                oninput: move |e| {
                                    let mut d = data.write();
                                    d.columns[col_idx].title = e.value();
                                    save(d.clone());
                                }
                            }
                            span { style: "font-size: 11px; color: {c.text_muted}; background: {c.bg_canvas}; padding: 2px 6px; border-radius: 10px;", "{column.tasks.len()}" }
                        }

                        div { style: "flex: 1; overflow-y: auto; padding: 8px; display: flex; flex-direction: column; gap: 8px;",
                            for (task_idx, task) in column.tasks.iter().enumerate() {
                                div { key: "{task_idx}", style: "background: {c.bg_canvas}; border: 1px solid {c.border}; border-radius: 4px; padding: 12px; position: relative; group",
                                    textarea {
                                        style: "width: 100%; background: transparent; border: none; color: {c.text_primary}; font-family: Inter; font-size: 13px; resize: none; outline: none; line-height: 1.4;",
                                        value: "{task}",
                                        oninput: move |e| {
                                            let mut d = data.write();
                                            d.columns[col_idx].tasks[task_idx] = e.value();
                                            save(d.clone());
                                        }
                                    }
                                    div { style: "display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;",
                                        button {
                                            style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; font-size: 14px; padding: 0;",
                                            onclick: move |_| {
                                                let mut d = data.write();
                                                d.columns[col_idx].tasks.remove(task_idx);
                                                save(d.clone());
                                            },
                                            span { class: "material-symbols-outlined", style: "font-size: 16px;", "delete" }
                                        }
                                    }
                                }
                            }

                            button {
                                style: "width: 100%; padding: 8px; background: transparent; border: 1px dashed {c.border}; border-radius: 4px; color: {c.text_muted}; font-size: 12px; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 4px;",
                                onclick: move |_| {
                                    let mut d = data.write();
                                    d.columns[col_idx].tasks.push(String::new());
                                    save(d.clone());
                                },
                                span { class: "material-symbols-outlined", style: "font-size: 16px;", "add" }
                                "Add Card"
                            }
                        }
                    }
                }

                button {
                    style: "min-width: 200px; padding: 12px; background: {c.bg_surface_low}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_secondary}; font-family: Inter; font-size: 13px; font-weight: 500; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 8px; height: 48px;",
                    onclick: move |_| {
                        let mut d = data.write();
                        d.columns.push(KanbanColumn { title: "New Column".into(), tasks: vec![] });
                        save(d.clone());
                    },
                    span { class: "material-symbols-outlined", style: "font-size: 20px;", "add_column" }
                    "Add Column"
                }
            }
        }
    }
}
