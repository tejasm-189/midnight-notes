use chrono::Utc;
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
    let mut search_query = use_signal(String::new);

    let db_new = db.clone();
    let db_save = db.clone();
    let db_list = db.clone();
    use_effect(move || {
        if let Some(ref db) = db_list {
            if let Ok(list) = NoteService::new(db).list_active() {
                notes.set(list);
            }
        }
    });

    let word_count = || -> usize {
        let c = content.read();
        if c.is_empty() {
            0
        } else {
            c.split_whitespace().count()
        }
    };

    let read_time = || -> String {
        let wc = word_count();
        if wc == 0 {
            "0m".into()
        } else {
            format!("{}m", (wc as f64 / 200.0).ceil() as usize)
        }
    };

    let formatted_date = |note: &Note| -> String {
        let now = Utc::now();
        let diff = now - note.updated_at;
        if diff.num_minutes() < 1 {
            "Just now".into()
        } else if diff.num_hours() < 1 {
            format!("{}m ago", diff.num_minutes())
        } else if diff.num_days() < 1 {
            format!("{}h ago", diff.num_hours())
        } else if diff.num_days() < 7 {
            format!("{}d ago", diff.num_days())
        } else {
            note.updated_at.format("%b %d").to_string()
        }
    };

    rsx! {
        div { style: "display: flex; height: 100vh; background: #131313;",

            // ====== SIDEBAR ======
            nav { style: "width: 256px; min-width: 256px; height: 100vh; background: #131313; border-right: 1px solid #3b494b; display: flex; flex-direction: column; padding: 16px 0; font-family: 'JetBrains Mono', monospace; font-size: 12px;",

                // Branding
                div { style: "padding: 0 16px; margin-bottom: 24px; display: flex; align-items: center; gap: 8px;",
                    span { class: "material-symbols-outlined fill", style: "font-size: 28px; color: #00dbe9;", "description" }
                    div {
                        h1 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: #00dbe9; line-height: 1.2;", "Midnight Notes" }
                        p { style: "font-size: 11px; color: #00e475; letter-spacing: 0.03em;", "Local-first Sync" }
                    }
                }

                // New Note
                button { style: "margin: 0 16px 16px; display: flex; align-items: center; justify-content: center; gap: 6px; padding: 8px; background: #00dbe9; color: #002022; border: none; border-radius: 4px; font-size: 11px; font-weight: 500; letter-spacing: 0.03em; cursor: pointer;",
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

                // Nav section header
                div { style: "flex: 1; overflow-y: auto;",
                    // All Notes (active)
                    NavItem { icon: "description", label: "All Notes", active: true }
                    NavItem { icon: "auto_awesome", label: "Smart Views", active: false }
                    div { style: "height: 1px; background: #3b494b; margin: 8px 16px;" }
                    NavItem { icon: "archive", label: "Archived", active: false }
                    NavItem { icon: "delete", label: "Trash", active: false }
                    NavItem { icon: "lock", label: "Encrypted", active: false }
                    NavItem { icon: "settings", label: "Settings", active: false,
                        onclick: move |_| on_open_settings.call(()), }
                }

                // Bottom section
                div { style: "border-top: 1px solid #3b494b; padding: 8px 8px 0;",
                    NavItem { icon: "help", label: "Help", active: false }
                    NavItem { icon: "sensors", label: "Status", active: false }
                    NavItem { icon: "lock", label: "Lock Vault", active: false,
                        onclick: move |_| on_lock.call(()), }
                }
            }

            // ====== NOTE LIST ======
            aside { style: "width: 320px; min-width: 320px; border-right: 1px solid #3b494b; background: #0e0e0e; display: flex; flex-direction: column;",

                // Search
                div { style: "padding: 16px; border-bottom: 1px solid #3b494b;",
                    div { style: "position: relative;",
                        span { class: "material-symbols-outlined", style: "position: absolute; left: 8px; top: 50%; transform: translateY(-50%); font-size: 14px; color: #b9cacb;", "search" }
                        input { r#type: "text", placeholder: "Search notes...", value: "{search_query}",
                            oninput: move |e| search_query.set(e.value()),
                            style: "width: 100%; height: 32px; background: #131313; border: 1px solid #3b494b; border-radius: 4px; padding: 4px 8px 4px 28px; color: #e5e2e1; font-family: Inter; font-size: 13px; outline: none;",
                        }
                    }
                }

                // Note items
                div { style: "flex: 1; overflow-y: auto;",
                    {let db_iter = db.clone(); notes.read().iter().map(move |note| {
                        let is_active = selected_id.read().as_deref() == Some(&note.id);
                        let bg = if is_active { "#2a2a2a" } else { "#0e0e0e" };
                        let title_color = if is_active { "#00dbe9" } else { "#e5e2e1" };
                        let note_id = note.id.clone();
                        let date_str = formatted_date(note);
                        let db_click = db_iter.clone();
                        rsx! {
                            div {
                                key: "{note.id}",
                                style: "padding: 16px; border-bottom: 1px solid #3b494b; background: {bg}; cursor: pointer; position: relative;",
                                onclick: move |_| {
                                    if let Some(ref db) = db_click {
                                        if let Ok(Some(n)) = NoteService::new(db).get(&note_id) {
                                            selected_id.set(Some(note_id.clone()));
                                            title.set(n.title);
                                            content.set(n.content);
                                        }
                                    }
                                },
                                // Active indicator bar
                                if is_active { div { style: "position: absolute; left: 0; top: 0; bottom: 0; width: 4px; background: #00dbe9;" } }
                                div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 4px;",
                                    h3 { style: "font-family: Inter; font-size: 14px; font-weight: 600; color: {title_color}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1;", "{note.title}" }
                                    if note.is_pinned { span { class: "material-symbols-outlined fill", style: "font-size: 14px; color: #00dbe9; min-width: 16px;", "push_pin" } }
                                }
                                p { style: "font-family: Inter; font-size: 13px; color: #b9cacb; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; line-height: 1.4; margin-bottom: 8px;",
                                    if note.content.is_empty() { "Empty note" } else { "{&note.content}" }
                                }
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    span { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; color: #b9cacb;", "{date_str}" }
                                    span { style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; color: #b9cacb; background: #131313; padding: 2px 6px; border-radius: 2px; border: 1px solid #3b494b;", "#note" }
                                }
                            }
                        }
                    })}
                    if notes.read().is_empty() {
                        div { style: "padding: 32px 16px; text-align: center; font-family: Inter; font-size: 13px; color: #849495; line-height: 1.6;",
                            p { "No notes yet" }
                            p { style: "margin-top: 4px; font-size: 12px;", "Create one with the button above" }
                        }
                    }
                }
            }

            // ====== EDITOR ======
            section { style: "flex: 1; display: flex; flex-direction: column; min-width: 0; background: #000;",

                // Editor header
                header { style: "height: 64px; min-height: 64px; background: #0e0e0e; border-bottom: 1px solid #3b494b; display: flex; align-items: center; justify-content: space-between; padding: 0 16px;",

                    div { style: "display: flex; align-items: center; gap: 16px;",
                        div { style: "display: flex; background: #131313; border-radius: 4px; border: 1px solid #3b494b; padding: 2px; font-family: 'JetBrains Mono', monospace; font-size: 11px;",
                            ModeBtn { label: "Prose", active: mode() == "Prose", onclick: move |_| mode.set("Prose") }
                            ModeBtn { label: "Code", active: mode() == "Code", onclick: move |_| mode.set("Code") }
                            ModeBtn { label: "Vim", active: mode() == "Vim", onclick: move |_| mode.set("Vim") }
                        }
                    }

                    div { style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "display: flex; align-items: center; gap: 4px; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; color: #00e475; background: #2a2a2a; padding: 4px 8px; border-radius: 4px; border: 1px solid #3b494b;",
                            span { class: "material-symbols-outlined", style: "font-size: 14px;", "lock" }
                            "Encrypted"
                        }
                        button { style: "color: #00dbe9; background: none; border: none; padding: 4px; cursor: pointer;", span { class: "material-symbols-outlined fill", style: "font-size: 18px;", "push_pin" } }
                        button { style: "color: #b9cacb; background: none; border: none; padding: 4px; cursor: pointer;", span { class: "material-symbols-outlined", style: "font-size: 18px;", "more_vert" } }
                        button { style: "background: #00dbe9; color: #002022; border: none; border-radius: 4px; padding: 6px 12px; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; cursor: pointer; letter-spacing: 0.03em;",
                            onclick: move |_| {
                                if let Some(ref d) = db_save {
                                    if let Some(ref id) = *selected_id.read() {
                                        let _ = NoteService::new(d).update(id, &title.read(), &content.read());
                                        if let Ok(list) = NoteService::new(d).list_active() {
                                            notes.set(list);
                                        }
                                    }
                                }
                            },
                            "Save"
                        }
                    }
                }

                // Editor canvas
                if selected_id.read().is_some() {
                    div { style: "flex: 1; overflow-y: auto; display: flex; justify-content: center;",
                        div { style: "width: 100%; max-width: 840px; padding: 24px 32px; position: relative;",

                            // Line numbers gutter
                            div { style: "position: absolute; left: 0; top: 0; bottom: 0; width: 32px; border-right: 1px solid #3b494b; display: flex; flex-direction: column; align-items: flex-end; padding: 24px 8px 24px 0; font-family: 'JetBrains Mono', monospace; font-size: 14px; color: #3b494b; opacity: 0.3; user-select: none;",
                                { (1..=30).map(|i| rsx! { span { "{i}" } }) }
                            }

                            div { style: "padding-left: 48px; width: 100%;",
                                input { r#type: "text", value: "{title}", oninput: move |e| title.set(e.value()),
                                    style: "width: 100%; background: transparent; border: none; font-family: Inter; font-size: 32px; font-weight: 700; letter-spacing: -0.02em; color: #e5e2e1; margin-bottom: 24px; padding: 0; outline: none;",
                                }
                                if mode() == "Prose" {
                                    textarea { value: "{content}", oninput: move |e| content.set(e.value()),
                                        style: "width: 100%; background: transparent; border: none; color: #e5e2e1; font-family: Inter; font-size: 16px; line-height: 1.7; resize: none; outline: none; min-height: 60vh;",
                                    }
                                } else if mode() == "Code" {
                                    textarea { value: "{content}", oninput: move |e| content.set(e.value()),
                                        spellcheck: false,
                                        style: "width: 100%; min-height: 60vh; background: transparent; border: none; color: #e5e2e1; font-family: 'JetBrains Mono', monospace; font-size: 14px; line-height: 1.6; resize: none; outline: none; tab-size: 2;",
                                    }
                                } else {
                                    textarea { value: "{content}", oninput: move |e| content.set(e.value()),
                                        style: "width: 100%; min-height: 60vh; background: transparent; border: none; color: #e5e2e1; font-family: 'JetBrains Mono', monospace; font-size: 14px; line-height: 1.6; resize: none; outline: none;",
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 16px;",
                        span { class: "material-symbols-outlined", style: "font-size: 48px; color: #3b494b;", "description" }
                        p { style: "font-family: Inter; font-size: 16px; color: #849495;", "Select a note or create a new one" }
                    }
                }

                // Footer
                footer { style: "height: 32px; min-height: 32px; background: #0e0e0e; border-top: 1px solid #3b494b; display: flex; align-items: center; justify-content: space-between; padding: 0 16px; font-family: 'JetBrains Mono', monospace; font-size: 10px;",
                    div { style: "font-weight: 700; color: #00dbe9; display: flex; align-items: center; gap: 4px;",
                        span { class: "material-symbols-outlined fill", style: "font-size: 12px; color: #00e475;", "lock" }
                        "End-to-End Encrypted"
                    }
                    div { style: "display: flex; gap: 16px; color: #849495;",
                        span { "Words: {word_count()}" }
                        span { "Read: {read_time()}" }
                        if mode() == "Vim" { span { style: "color: #00dbe9; font-weight: 500;", "VIM Mode: Enabled" } }
                    }
                }
            }
        }
    }
}

#[component]
fn NavItem(
    icon: String,
    label: String,
    active: bool,
    onclick: Option<EventHandler<()>>,
) -> Element {
    let bg = if active { "#2a2a2a" } else { "transparent" };
    let color = if active { "#00dbe9" } else { "#b9cacb" };
    let border = if active {
        "2px solid #00dbe9"
    } else {
        "2px solid transparent"
    };
    rsx! {
        a {
            style: "display: flex; align-items: center; gap: 12px; padding: 8px 16px; background: {bg}; color: {color}; border-left: {border}; cursor: pointer; transition: all 0.15s; font-size: 11px; letter-spacing: 0.03em;",
            onclick: move |_| { if let Some(cb) = &onclick { cb.call(()); } },
            span { class: "material-symbols-outlined", style: "font-size: 18px;", "{icon}" }
            span { "{label}" }
        }
    }
}

#[component]
fn ModeBtn(label: String, active: bool, onclick: EventHandler<()>) -> Element {
    let bg = if active { "#2a2a2a" } else { "transparent" };
    let color = if active { "#00dbe9" } else { "#b9cacb" };
    let shadow = if active {
        "0 1px 2px rgba(0,0,0,0.3)"
    } else {
        "none"
    };
    rsx! {
        button { style: "padding: 4px 8px; border-radius: 2px; background: {bg}; color: {color}; border: none; cursor: pointer; box-shadow: {shadow}; transition: all 0.1s;",
            onclick: move |_| onclick.call(()),
            "{label}"
        }
    }
}
