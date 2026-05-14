use chrono::Utc;
use dioxus::prelude::*;

use crate::core::note::NoteService;
use crate::storage::models::Note;
use crate::ui::app::SharedDb;
use crate::ui::theme::use_theme_colors;

#[component]
pub fn Workspace(
    db: Option<SharedDb>,
    on_lock: EventHandler<()>,
    on_open_settings: EventHandler<()>,
) -> Element {
    let c = use_theme_colors();
    let mut mode = use_signal(|| "Prose");
    let mut notes: Signal<Vec<Note>> = use_signal(Vec::new);
    let mut selected_id = use_signal(|| None::<String>);
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);

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

    let wc = || content.read().split_whitespace().count();
    let rt = || {
        if wc() == 0 {
            "0m".into()
        } else {
            format!("{}m", (wc() as f64 / 200.0).ceil() as usize)
        }
    };

    let fmt_date = |note: &Note| -> String {
        let diff = Utc::now() - note.updated_at;
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
        div { style: "display: flex; height: 100vh; background: {c.bg_primary};",

            nav { style: "width: 256px; min-width: 256px; border-right: 1px solid {c.border}; background: {c.bg_surface}; display: flex; flex-direction: column; padding: 16px 0; font-family: 'JetBrains Mono', monospace; font-size: 12px;",
                div { style: "padding: 0 16px; margin-bottom: 24px; display: flex; align-items: center; gap: 8px;",
                    span { class: "material-symbols-outlined fill", style: "font-size: 28px; color: {c.accent};", "description" }
                    div { h1 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: {c.accent};", "Midnight Notes" } p { style: "font-size: 11px; color: {c.accent_green};", "Local-first Sync" } }
                }
                button { style: "margin: 0 16px 16px; display: flex; align-items: center; justify-content: center; gap: 6px; padding: 8px; background: {c.accent}; color: {c.bg_primary}; border: none; border-radius: 4px; font-size: 11px; cursor: pointer;",
                    onclick: move |_| { if let Some(ref db) = db_new { if let Ok(note) = NoteService::new(db).create("Untitled", "") { title.set(note.title.clone()); content.set(note.content.clone()); selected_id.set(Some(note.id.clone())); notes.write().insert(0, note); } } },
                    span { class: "material-symbols-outlined", style: "font-size: 16px;", "add" } "New Note"
                }
                div { style: "flex: 1; overflow-y: auto;",
                    NavItem { icon: "description", label: "All Notes", active: true }
                    NavItem { icon: "auto_awesome", label: "Smart Views", active: false }
                    div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }
                    NavItem { icon: "archive", label: "Archived", active: false }
                    NavItem { icon: "delete", label: "Trash", active: false }
                    NavItem { icon: "lock", label: "Encrypted", active: false }
                    NavItem { icon: "settings", label: "Settings", active: false, onclick: move |_| on_open_settings.call(()) }
                }
                div { style: "border-top: 1px solid {c.border}; padding: 8px;",
                    NavItem { icon: "help", label: "Help", active: false }
                    NavItem { icon: "sensors", label: "Status", active: false }
                    NavItem { icon: "lock", label: "Lock Vault", active: false, onclick: move |_| on_lock.call(()) }
                }
            }

            aside { style: "width: 320px; min-width: 320px; border-right: 1px solid {c.border}; background: {c.bg_canvas}; display: flex; flex-direction: column;",
                div { style: "padding: 16px; border-bottom: 1px solid {c.border};",
                    div { style: "position: relative;",
                        span { class: "material-symbols-outlined", style: "position: absolute; left: 8px; top: 50%; transform: translateY(-50%); font-size: 14px; color: {c.text_secondary};", "search" }
                        input { r#type: "text", placeholder: "Search notes...", style: "width: 100%; height: 32px; background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 4px; padding: 4px 8px 4px 28px; color: {c.text_primary}; font-family: Inter; font-size: 13px; outline: none;", }
                    }
                }
                div { style: "flex: 1; overflow-y: auto;",
                    {let di = db.clone(); notes.read().iter().map(move |note| {
                        let act = selected_id.read().as_deref() == Some(&note.id);
                        let bg = if act { c.bg_surface_high } else { c.bg_canvas };
                        let tc = if act { c.accent } else { c.text_primary };
                        let nid = note.id.clone(); let dc = di.clone();
                        rsx! {
                            div { key: "{note.id}", style: "padding: 16px; border-bottom: 1px solid {c.border}; background: {bg}; cursor: pointer; position: relative;",
                                onclick: move |_| { if let Some(ref db) = dc { if let Ok(Some(n)) = NoteService::new(db).get(&nid) { selected_id.set(Some(nid.clone())); title.set(n.title); content.set(n.content); } } },
                                if act { div { style: "position: absolute; left: 0; top: 0; bottom: 0; width: 4px; background: {c.accent};" } }
                                div { style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 4px;",
                                    h3 { style: "font-family: Inter; font-size: 14px; font-weight: 600; color: {tc}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1;", "{note.title}" }
                                    if note.is_pinned { span { class: "material-symbols-outlined fill", style: "font-size: 14px; color: {c.accent};", "push_pin" } }
                                }
                                p { style: "font-family: Inter; font-size: 13px; color: {c.text_secondary}; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; line-height: 1.4; margin-bottom: 8px;",
                                    if note.content.is_empty() { "Empty note" } else { "{&note.content}" }
                                }
                                div { style: "display: flex; justify-content: space-between; align-items: center;",
                                    span { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; color: {c.text_secondary};", "{fmt_date(note)}" }
                                    span { style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; color: {c.text_secondary}; background: {c.bg_surface}; padding: 2px 6px; border-radius: 2px; border: 1px solid {c.border};", "#note" }
                                }
                            }
                        }
                    })}
                    if notes.read().is_empty() {
                        div { style: "padding: 32px; text-align: center; font-family: Inter; font-size: 13px; color: {c.text_muted};", p { "No notes yet" } p { style: "margin-top: 4px; font-size: 12px;", "Create one with the button above" } }
                    }
                }
            }

            section { style: "flex: 1; display: flex; flex-direction: column; min-width: 0; background: {c.bg_primary};",
                header { style: "height: 64px; background: {c.bg_canvas}; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between; padding: 0 16px;",
                    div { style: "display: flex; background: {c.bg_surface}; border-radius: 4px; border: 1px solid {c.border}; padding: 2px; font-family: 'JetBrains Mono', monospace; font-size: 11px;",
                        ModeBtn { label: "Prose", active: mode() == "Prose", onclick: move |_| mode.set("Prose") }
                        ModeBtn { label: "Code", active: mode() == "Code", onclick: move |_| mode.set("Code") }
                        ModeBtn { label: "Vim", active: mode() == "Vim", onclick: move |_| mode.set("Vim") }
                    }
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        span { style: "display: flex; align-items: center; gap: 4px; font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 500; color: {c.accent_green}; background: {c.bg_surface_high}; padding: 4px 8px; border-radius: 4px; border: 1px solid {c.border};",
                            span { class: "material-symbols-outlined", style: "font-size: 14px;", "lock" } "Encrypted"
                        }
                        button { style: "color: {c.accent}; background: none; border: none; padding: 4px; cursor: pointer;", span { class: "material-symbols-outlined fill", style: "font-size: 18px;", "push_pin" } }
                        button { style: "color: {c.text_secondary}; background: none; border: none; padding: 4px; cursor: pointer;", span { class: "material-symbols-outlined", style: "font-size: 18px;", "more_vert" } }
                        button { style: "background: {c.accent}; color: {c.bg_primary}; border: none; border-radius: 4px; padding: 6px 12px; font-family: 'JetBrains Mono', monospace; font-size: 11px; cursor: pointer;",
                            onclick: move |_| { if let Some(ref d) = db_save { if let Some(ref id) = *selected_id.read() { let _ = NoteService::new(d).update(id, &title.read(), &content.read()); if let Ok(list) = NoteService::new(d).list_active() { notes.set(list); } } } },
                            "Save"
                        }
                    }
                }

                if selected_id.read().is_some() {
                    div { style: "flex: 1; overflow-y: auto; display: flex; justify-content: center;",
                        div { style: "width: 100%; max-width: 840px; padding: 24px 32px; position: relative;",
                            div { style: "position: absolute; left: 0; top: 0; bottom: 0; width: 32px; border-right: 1px solid {c.border}; display: flex; flex-direction: column; align-items: flex-end; padding: 24px 8px 24px 0; font-family: 'JetBrains Mono', monospace; font-size: 14px; color: {c.border}; opacity: 0.3; user-select: none;",
                                { (1..=30).map(|i| rsx! { span { "{i}" } }) }
                            }
                            div { style: "padding-left: 48px;",
                                input { r#type: "text", value: "{title}", oninput: move |e| title.set(e.value()),
                                    style: "width: 100%; background: transparent; border: none; font-family: Inter; font-size: 32px; font-weight: 700; letter-spacing: -0.02em; color: {c.text_primary}; margin-bottom: 24px; outline: none;",
                                }
                                textarea { value: "{content}", oninput: move |e| content.set(e.value()),
                                    style: "width: 100%; min-height: 60vh; background: transparent; border: none; color: {c.text_primary}; font-family: Inter; font-size: 16px; line-height: 1.7; resize: none; outline: none;",
                                }
                            }
                        }
                    }
                } else {
                    div { style: "flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 16px;",
                        span { class: "material-symbols-outlined", style: "font-size: 48px; color: {c.border};", "description" }
                        p { style: "font-family: Inter; font-size: 16px; color: {c.text_muted};", "Select a note or create a new one" }
                    }
                }

                footer { style: "height: 32px; background: {c.bg_canvas}; border-top: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between; padding: 0 16px; font-family: 'JetBrains Mono', monospace; font-size: 10px;",
                    div { style: "display: flex; align-items: center; gap: 4px; font-weight: 700; color: {c.accent};",
                        span { class: "material-symbols-outlined fill", style: "font-size: 12px; color: {c.accent_green};", "lock" } "End-to-End Encrypted"
                    }
                    div { style: "display: flex; gap: 16px; color: {c.text_muted};",
                        span { "Words: {wc()}" } span { "Read: {rt()}" }
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
    let c = use_theme_colors();
    let bg = if active {
        c.bg_surface_high
    } else {
        "transparent"
    };
    let color = if active { c.accent } else { c.text_secondary };
    let border = if active {
        "2px solid ".to_string() + c.accent
    } else {
        "2px solid transparent".to_string()
    };
    rsx! {
        a { style: "display: flex; align-items: center; gap: 12px; padding: 8px 16px; background: {bg}; color: {color}; border-left: {border}; cursor: pointer; transition: all 0.15s; font-size: 11px;",
            onclick: move |_| { if let Some(cb) = &onclick { cb.call(()); } },
            span { class: "material-symbols-outlined", style: "font-size: 18px;", "{icon}" }
            span { "{label}" }
        }
    }
}

#[component]
fn ModeBtn(label: String, active: bool, onclick: EventHandler<()>) -> Element {
    let c = use_theme_colors();
    let bg = if active {
        c.bg_surface_high
    } else {
        "transparent"
    };
    let color = if active { c.accent } else { c.text_secondary };
    rsx! {
        button { style: "padding: 4px 8px; border-radius: 2px; background: {bg}; color: {color}; border: none; cursor: pointer; transition: all 0.1s;",
            onclick: move |_| onclick.call(()), "{label}"
        }
    }
}
