use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
struct SheetData {
    rows: Vec<Vec<String>>,
}

#[component]
pub fn SpreadsheetEditor(
    content: String,
    oninput: EventHandler<String>,
    title: String,
    ontitleinput: EventHandler<String>,
) -> Element {
    let c = use_theme_colors();

    // Parse content as JSON SheetData, fallback to 5x5 empty if fails
    let mut data = use_signal(|| {
        serde_json::from_str::<SheetData>(&content).unwrap_or_else(|_| SheetData {
            rows: vec![vec![String::new(); 5]; 10],
        })
    });

    let save = move |d: SheetData| {
        if let Ok(json) = serde_json::to_string(&d) {
            oninput.call(json);
        }
    };

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 24px; padding-bottom: 40px;",
            input { r#type: "text", value: "{title}", oninput: move |e| ontitleinput.call(e.value()),
                placeholder: "Sheet Title",
                style: "width: 100%; background: transparent; border: none; font-family: Inter; font-size: 32px; font-weight: 700; letter-spacing: -0.02em; color: {c.text_primary}; outline: none;",
            }

            div { style: "overflow-x: auto; background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 8px;",
                table { style: "border-collapse: collapse; width: 100%; font-family: 'JetBrains Mono', monospace; font-size: 13px;",
                    thead {
                        tr {
                            th { style: "width: 40px; background: {c.bg_canvas}; border-right: 1px solid {c.border}; border-bottom: 1px solid {c.border};" }
                            for i in 0..data.read().rows.first().map(|r| r.len()).unwrap_or(0) {
                                th {
                                    key: "{i}",
                                    style: "padding: 8px; background: {c.bg_canvas}; border-right: 1px solid {c.border}; border-bottom: 1px solid {c.border}; color: {c.text_muted}; font-weight: 500;",
                                    "{ (b'A' + i as u8) as char }"
                                }
                            }
                        }
                    }
                    tbody {
                        for (r_idx, row) in data.read().rows.iter().enumerate() {
                            tr { key: "{r_idx}",
                                td { style: "text-align: center; background: {c.bg_canvas}; border-right: 1px solid {c.border}; border-bottom: 1px solid {c.border}; color: {c.text_muted}; font-size: 11px;",
                                    "{r_idx + 1}"
                                }
                                for (c_idx, cell) in row.iter().enumerate() {
                                    td { key: "{c_idx}", style: "border-right: 1px solid {c.border}; border-bottom: 1px solid {c.border}; padding: 0;",
                                        input {
                                            style: "width: 100%; border: none; background: transparent; padding: 8px; color: {c.text_primary}; outline: none; transition: background 0.1s;",
                                            value: "{cell}",
                                            oninput: move |e| {
                                                let mut d = data.write();
                                                if let Some(r) = d.rows.get_mut(r_idx) {
                                                    if let Some(c) = r.get_mut(c_idx) {
                                                        *c = e.value();
                                                    }
                                                }
                                                save(d.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { style: "display: flex; gap: 12px;",
                button {
                    style: "background: {c.bg_surface_high}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; padding: 6px 12px; font-size: 11px; cursor: pointer;",
                    onclick: move |_| {
                        let mut d = data.write();
                        let col_count = d.rows.first().map(|r| r.len()).unwrap_or(5);
                        d.rows.push(vec![String::new(); col_count]);
                        save(d.clone());
                    },
                    "+ Add Row"
                }
                button {
                    style: "background: {c.bg_surface_high}; color: {c.text_primary}; border: 1px solid {c.border}; border-radius: 4px; padding: 6px 12px; font-size: 11px; cursor: pointer;",
                    onclick: move |_| {
                        let mut d = data.write();
                        for row in d.rows.iter_mut() {
                            row.push(String::new());
                        }
                        save(d.clone());
                    },
                    "+ Add Column"
                }
            }
        }
    }
}
