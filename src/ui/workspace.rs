use dioxus::prelude::*;

use crate::core::note::NoteService;
use crate::storage::models::Note;
use crate::ui::app::SharedDb;

#[component]
pub fn Workspace(
    db: Option<SharedDb>,
    on_lock: EventHandler<()>,
    on_open_settings: EventHandler<()>,
) -> Element {
    let mut mode = use_signal(|| "Prose");
    let mut notes: Signal<Vec<Note>> = use_signal(Vec::new);
    let mut selected_id = use_signal(|| None::<String>);
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);

    let db_new = db.clone();
    let db_list = db.clone();
    let db_save = db.clone();

    use_effect(move || {
        if let Some(ref db) = db_list {
            if let Ok(list) = NoteService::new(db).list_active() {
                notes.set(list);
            }
        }
    });

    rsx! {
        div { style: "display: flex; height: 100vh; background: #131313;",

            nav { style: "width: 256px; min-width: 256px; height: 100vh; background: #131313; border-right: 1px solid #3b494b; display: flex; flex-direction: column; padding: 16px 0;",
                div { style: "padding: 0 16px; margin-bottom: 24px; display: flex; align-items: center; gap: 8px;",
                    span { class: "material-symbols-outlined fill", style: "font-size: 24px; color: #00dbe9;", "description" }
                    div { h1 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: #00dbe9;", "Midnight Notes" } p { style: "font-family: 'JetBrains Mono', monospace; font-size: 12px; color: #b9cacb;", "Local-first Sync" } }
                }
                button { style: "margin: 0 16px 16px; display: flex; align-items: center; justify-content: center; gap: 8px; padding: 8px; background: #00dbe9; color: #002022; border: none; border-radius: 4px; font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; cursor: pointer;",
                    onclick: move |_| {
                        if let Some(ref db) = db_new {
                            if let Ok(note) = NoteService::new(db).create("Untitled", "") {
                                title.set(note.title.clone());
                                content.set(note.content.clone());
                                selected_id.set(Some(note.id.clone()));
                                notes.write().insert(0, note);
                            }
                        }
                    },
                    span { class: "material-symbols-outlined", style: "font-size: 16px;", "add" }
                    "New Note"
                }

                div { style: "flex: 1; overflow-y: auto; padding: 0 8px;",
                    {let db = &db; notes.read().iter().map(|note| {
                        let is_active = selected_id.read().as_deref() == Some(&note.id);
                        let bg = if is_active { "#2a2a2a" } else { "transparent" };
                        let title_color = if is_active { "#00dbe9" } else { "#e5e2e1" };
                        let note_id = note.id.clone();
                        let db_click = db.clone();
                        rsx! {
                            div {
                                key: "{note.id}",
                                style: "padding: 8px 12px; margin-bottom: 2px; border-radius: 4px; background: {bg}; cursor: pointer;",
                                onclick: move |_| {
                                    if let Some(ref db) = db_click {
                                        if let Ok(Some(n)) = NoteService::new(db).get(&note_id) {
                                            selected_id.set(Some(note_id.clone()));
                                            title.set(n.title);
                                            content.set(n.content);
                                        }
                                    }
                                },
                                div { style: "font-family: Inter; font-size: 13px; font-weight: 600; color: {title_color}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{note.title}" }
                                div { style: "font-family: Inter; font-size: 12px; color: #b9cacb; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; margin-top: 2px;",
                                    if note.content.is_empty() { "Empty note" } else { "{note.content.chars().take(60).collect::<String>()}" }
                                }
                            }
                        }
                    })}
                    if notes.read().is_empty() {
                        div { style: "padding: 16px; text-align: center; font-family: Inter; font-size: 13px; color: #849495;", "No notes yet. Create one!" }
                    }
                }

                div { style: "border-top: 1px solid #3b494b; padding: 8px;",
                    a { style: "display: flex; align-items: center; gap: 8px; padding: 6px 8px; border-radius: 4px; font-family: 'JetBrains Mono', monospace; font-size: 11px; color: #b9cacb; cursor: pointer;",
                        onclick: move |_| on_open_settings.call(()),
                        span { class: "material-symbols-outlined", style: "font-size: 16px;", "settings" }
                        span { "Settings" }
                    }
                    a { style: "display: flex; align-items: center; gap: 8px; padding: 6px 8px; border-radius: 4px; font-family: 'JetBrains Mono', monospace; font-size: 11px; color: #b9cacb; cursor: pointer; margin-top: 2px;",
                        onclick: move |_| on_lock.call(()),
                        span { class: "material-symbols-outlined", style: "font-size: 16px;", "lock" }
                        span { "Lock Vault" }
                    }
                }
            }

            section { style: "flex: 1; display: flex; flex-direction: column; min-width: 0; background: #000;",
                header { style: "height: 56px; background: #0e0e0e; border-bottom: 1px solid #3b494b; display: flex; align-items: center; justify-content: space-between; padding: 0 16px;",
                    div { style: "display: flex; background: #131313; border-radius: 4px; border: 1px solid #3b494b; padding: 2px;",
                        ModeBtn { label: "Prose", active: mode() == "Prose", onclick: move |_| mode.set("Prose") }
                        ModeBtn { label: "Code", active: mode() == "Code", onclick: move |_| mode.set("Code") }
                        ModeBtn { label: "Vim", active: mode() == "Vim", onclick: move |_| mode.set("Vim") }
                    }
                    button { style: "background: #00dbe9; color: #002022; border: none; border-radius: 4px; padding: 6px 16px; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; cursor: pointer;",
                        onclick: move |_| {
                            if let Some(ref db) = db_save {
                                if let Some(ref id) = *selected_id.read() {
                                    let _ = NoteService::new(db).update(id, &title.read(), &content.read());
                                }
                            }
                        },
                        "Save"
                    }
                }

                if selected_id.read().is_some() {
                    div { style: "flex: 1; display: flex; flex-direction: column; padding: 24px 32px; max-width: 840px; margin: 0 auto; width: 100%; overflow-y: auto;",
                        input { r#type: "text", value: "{title}", oninput: move |e| title.set(e.value()),
                            style: "width: 100%; background: transparent; border: none; font-family: Inter; font-size: 28px; font-weight: 700; letter-spacing: -0.02em; color: #e5e2e1; margin-bottom: 16px; outline: none;",
                        }
                        textarea { value: "{content}", oninput: move |e| content.set(e.value()),
                            style: "flex: 1; width: 100%; background: transparent; border: none; color: #e5e2e1; font-family: Inter; font-size: 16px; line-height: 1.6; resize: none; outline: none; min-height: 200px;",
                        }
                    }
                } else {
                    div { style: "flex: 1; display: flex; align-items: center; justify-content: center;",
                        p { style: "font-family: Inter; font-size: 16px; color: #849495;", "Select a note or create a new one" }
                    }
                }

                footer { style: "height: 28px; background: #0e0e0e; border-top: 1px solid #3b494b; display: flex; align-items: center; justify-content: space-between; padding: 0 16px;",
                    span { style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; font-weight: 700; color: #00dbe9;", "End-to-End Encrypted" }
                    span { style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; color: #849495;",
                        "Words: {content.read().split_whitespace().count()}" }
                }
            }
        }
    }
}

#[component]
fn ModeBtn(label: String, active: bool, onclick: EventHandler<()>) -> Element {
    let bg = if active { "#2a2a2a" } else { "transparent" };
    let color = if active { "#00dbe9" } else { "#b9cacb" };
    rsx! {
        button { style: "padding: 4px 8px; border-radius: 2px; background: {bg}; color: {color}; border: none; cursor: pointer; font-family: 'JetBrains Mono', monospace; font-size: 11px;",
            onclick: move |_| onclick.call(()), "{label}"
        }
    }
}
