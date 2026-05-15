#![allow(clippy::possible_missing_else)]
use dioxus::prelude::*;
use std::time::Duration;

use crate::core::export::ExportService;
use crate::core::history::HistoryService;
use crate::core::note::NoteService;
use crate::core::search::SearchService;
use crate::core::tag::TagService;
use crate::storage::models::Note;
use crate::ui::app::SharedDb;
use crate::ui::theme::use_theme_colors;

#[derive(Clone, PartialEq)]
enum View {
    AllNotes,
    Archived,
    Trash,
    SmartViews,
}

#[component]
pub fn Workspace(db: Option<SharedDb>, on_lock: EventHandler<()>) -> Element {
    let c = use_theme_colors();
    let mut view = use_signal(|| View::AllNotes);
    let mut mode = use_signal(|| "Prose");
    let mut notes = use_signal(Vec::<Note>::new);
    let mut selected_id = use_signal(|| None::<String>);
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut query = use_signal(String::new);
    let mut note_tags = use_signal(Vec::<String>::new);
    let mut all_tags = use_signal(Vec::<(String, String)>::new);
    let mut snapshots = use_signal(Vec::<(String, String)>::new);
    let mut show_snapshots = use_signal(|| false);
    let mut tag_input = use_signal(String::new);
    let mut note_tag_cache = use_signal(std::collections::HashMap::<String, Vec<String>>::new);
    let mut show_calendar = use_signal(|| false);
    let mut show_settings = use_signal(|| false);
    let mut context_note = use_signal(|| None::<String>);

    let _tag_version = use_signal(|| 0u32);

    // Pre-clone db for each closure
    let db_effect = db.clone();
    let db_smart = db.clone();
    let db_smart2 = db.clone();
    let db_new = db.clone();
    let db_pin = db.clone();
    let db_arch = db.clone();
    let db_save = db.clone();
    let db_restore = db.clone();
    let db_del = db.clone();
    let db_side_all = db.clone();
    let db_side_arch = db.clone();
    let db_side_trash = db.clone();
    let db_search = db.clone();
    let db_pin_r = db.clone();
    let db_arch_r = db.clone();
    let db_save_r = db.clone();
    let db_restore_r = db.clone();
    let db_del_r = db.clone();
    let db_auto = db.clone();
    let db_tag = db.clone();
    let db_hist = db.clone();
    let db_tag2 = db.clone();
    let db_tag3 = db.clone();
    let db_tag4 = db.clone();
    let db_tag5 = db.clone();
    let db_tag6 = db.clone();
    let db_tag7 = db.clone();
    let db_side_all2 = db.clone();
    let db_daily = db.clone();
    let db_export = db.clone();
    let db_import = db.clone();

    // Refresh tag cache and all_tags when note_tags changes
    let db_tc = db.clone();
    use_effect(move || {
        let _ = note_tags.read().clone(); // subscribe to note_tags changes
        if let Some(ref d) = db_tc {
            let mut cache = std::collections::HashMap::<String, Vec<String>>::new();
            for note in notes.read().iter() {
                if let Ok(tags) = TagService::new(d).get_tags_for_note(&note.id) {
                    cache.insert(
                        note.id.clone(),
                        tags.iter().map(|t| t.name.clone()).collect(),
                    );
                }
            }
            note_tag_cache.set(cache);
            if let Ok(all) = TagService::new(d).get_all() {
                all_tags.set(all.iter().map(|t| (t.id.clone(), t.name.clone())).collect());
            }
        }
    });

    let db_cache = db.clone();
    use_effect(move || {
        refresh_notes(&db_effect, &view.read(), &notes, &query.read());
        if let Some(ref d) = db_cache {
            let mut cache = std::collections::HashMap::<String, Vec<String>>::new();
            for note in notes.read().iter() {
                if let Ok(tags) = TagService::new(d).get_tags_for_note(&note.id) {
                    cache.insert(
                        note.id.clone(),
                        tags.iter().map(|t| t.name.clone()).collect(),
                    );
                }
            }
            note_tag_cache.set(cache);
        }
    });

    // Auto-save: debounced 2s after content/title changes
    let db_as = db_auto.clone();
    let sel_as = selected_id;
    let t_as = title;
    let c_as = content;
    use_effect(move || {
        let _t = t_as.read().clone();
        let _c = c_as.read().clone();
        let id = sel_as.read().clone();
        if id.is_none() {
            return;
        }
        let d2 = db_as.clone();
        let t2 = _t;
        let c2 = _c;
        let id2 = id.unwrap();
        spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            if let Some(ref d) = d2 {
                let _ = NoteService::new(d).update(&id2, &t2, &c2);
            }
        });
    });

    let fmt_date =
        |note: &Note| -> String { note.updated_at.format("%a %d %b %Y %I:%M %p").to_string() };

    rsx! {
        div { style: "display: flex; height: 100vh; background: {c.bg_primary};",

            // ====== SIDEBAR ======
            nav { style: "width: 256px; min-width: 256px; border-right: 1px solid {c.border}; background: {c.bg_surface}; display: flex; flex-direction: column; padding: 16px 0; font-family: 'JetBrains Mono', monospace; font-size: 12px;",
                div { style: "padding: 0 16px; margin-bottom: 24px; display: flex; align-items: center; gap: 8px;",
                    span { class: "material-symbols-outlined fill", style: "font-size: 28px; color: {c.accent};", "description" }
                    div { h1 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: {c.accent};", "Midnight Notes" } p { style: "font-size: 11px; color: {c.accent_green};", "Local-first Sync" } }
                }
                button { style: "margin: 0 16px 16px; display: flex; align-items: center; justify-content: center; gap: 6px; padding: 8px; background: {c.accent}; color: {c.bg_primary}; border: none; border-radius: 4px; font-size: 11px; cursor: pointer;",
                    onclick: move |_| { if let Some(ref d) = db_new { if let Ok(n) = NoteService::new(d).create("", "") { title.set(n.title.clone()); content.set(n.content.clone()); selected_id.set(Some(n.id.clone())); notes.write().insert(0, n); note_tags.set(vec![]); all_tags.set(vec![]); } } },
                    span { class: "material-symbols-outlined", style: "font-size: 16px;", "add" } "New Note"
                }
                div { style: "flex: 1; overflow-y: auto;",
                    SidebarItem { icon: "description", label: "All Notes", active: matches!(view.read().clone(), View::AllNotes), onclick: move |_| { query.set(String::new()); view.set(View::AllNotes); refresh_notes(&db_side_all, &View::AllNotes, &notes, ""); } }
                    SidebarItem { icon: "auto_awesome", label: "Smart Views", active: matches!(view.read().clone(), View::SmartViews), onclick: move |_| view.set(View::SmartViews) }
                    SidebarItem { icon: "calendar_month", label: "Calendar", active: *show_calendar.read(), onclick: move |_| { let c = *show_calendar.read(); show_calendar.set(!c); } }
                    SidebarItem { icon: "today", label: "Daily Note", active: false, onclick: move |_| {
                        if let Some(ref d) = db_daily {
                            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
                            let note_svc = NoteService::new(d);
                            // Check if today's note exists
                            let existing = note_svc.search(&today).ok().and_then(|r| r.into_iter().find(|(n, _)| n.title.contains(&today)).map(|(n, _)| n));
                            if let Some(note) = existing {
                                selected_id.set(Some(note.id.clone())); title.set(note.title); content.set(note.content);
                                if let Ok(tags) = TagService::new(d).get_tags_for_note(&note.id) { note_tags.set(tags.iter().map(|t2| t2.name.clone()).collect()); }
                                if let Ok(h) = HistoryService::new(d).list(&note.id) { snapshots.set(h.iter().map(|s| (s.id.clone(), s.created_at.format("%b %d %H:%M").to_string())).collect()); }
                            } else {
                                let date_fmt = chrono::Utc::now().format("%a %d %b %Y").to_string();
                                let template = format!("# Daily Note — {}\n\n## Tasks\n- [ ] \n\n## Notes\n\n## Meetings\n\n", date_fmt);
                                if let Ok(note) = note_svc.create(&today, &template) {
                                    title.set(note.title.clone()); content.set(note.content.clone()); selected_id.set(Some(note.id.clone()));
                                    notes.write().insert(0, note);
                                }
                            }
                        }
                    } }
                    div { style: "height: 1px; background: {c.border}; margin: 8px 16px;" }
                    SidebarItem { icon: "archive", label: "Archived", active: matches!(view.read().clone(), View::Archived), onclick: move |_| { view.set(View::Archived); refresh_notes(&db_side_arch, &View::Archived, &notes, ""); } }
                    SidebarItem { icon: "delete", label: "Trash", active: matches!(view.read().clone(), View::Trash), onclick: move |_| { view.set(View::Trash); refresh_notes(&db_side_trash, &View::Trash, &notes, ""); } }

                    // Tags section
                    if selected_id.read().is_some() {
                        div { style: "border-top: 1px solid {c.border}; margin: 8px 16px;" }
                        div { style: "padding: 4px 16px; font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.08em; display: flex; justify-content: space-between; align-items: center;",
                            span { "Tags" }
                            button { style: "background: none; border: none; color: {c.accent}; cursor: pointer; font-size: 14px; padding: 0;",
                                onclick: move |_| { if let Some(ref d) = db_tag { if let Some(ref nid) = *selected_id.read() { let t = tag_input.read().clone(); if !t.is_empty() { if let Ok(Some(tag)) = TagService::new(d).get_by_name(&t) { let _ = TagService::new(d).assign_to_note(&tag.id, nid); } else if let Ok(new_tag) = TagService::new(d).create(&t, None, None) { let _ = TagService::new(d).assign_to_note(&new_tag.id, nid); } tag_input.set(String::new()); if let Ok(tags) = TagService::new(d).get_tags_for_note(nid) {                             note_tags.set(tags.iter().map(|t2| t2.name.clone()).collect());
                        } if let Ok(h) = HistoryService::new(d).list(nid) { snapshots.set(h.iter().map(|s| (s.id.clone(), s.created_at.format("%b %d %H:%M").to_string())).collect()); } } } } },
                                "+"
                            }
                        }
                        input { r#type: "text", placeholder: "Add tag...", value: "{tag_input}",
                            oninput: move |e| tag_input.set(e.value()),
                            onkeydown: move |e| { if e.key() == dioxus::events::Key::Enter { let d = db_tag6.clone(); if let Some(ref d) = d { if let Some(ref nid) = *selected_id.read() { let t = tag_input.read().clone(); if !t.is_empty() { if let Ok(Some(tag)) = TagService::new(d).get_by_name(&t) { let _ = TagService::new(d).assign_to_note(&tag.id, nid); } else if let Ok(new_tag) = TagService::new(d).create(&t, None, None) { let _ = TagService::new(d).assign_to_note(&new_tag.id, nid); } tag_input.set(String::new()); if let Ok(tags) = TagService::new(d).get_tags_for_note(nid) { note_tags.set(tags.iter().map(|t2| t2.name.clone()).collect()); } } } } } },
                            style: "width: calc(100% - 32px); margin: 0 16px 4px; background: {c.bg_surface_container}; border: 1px solid {c.border}; border-radius: 2px; padding: 4px 6px; color: {c.text_primary}; font-size: 11px; outline: none;",
                        }
                        {note_tags.read().iter().map(|t| {
                            let tn = t.clone();
                            let db_rm = db_tag.clone();
                            let sid = selected_id;
                            rsx! {
                                div { key: "{t}", style: "display: flex; align-items: center; gap: 4px; padding: 2px 16px; font-size: 11px; color: {c.text_secondary};",
                                    span { class: "material-symbols-outlined", style: "font-size: 14px; color: {c.accent};", "tag" }
                                    span { style: "flex: 1;", "{t}" }
                                    button { style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; font-size: 12px; padding: 0;",
                                        onclick: move |_| { if let Some(ref d) = db_rm { if let Some(ref nid) = *sid.read() { if let Ok(Some(tag)) = TagService::new(d).get_by_name(&tn) { let _ = TagService::new(d).remove_from_note(&tag.id, nid); } if let Ok(tags) = TagService::new(d).get_tags_for_note(nid) { note_tags.set(tags.iter().map(|t| t.name.clone()).collect()); } } } },
                                        "✕"
                                    }
                                }
                            }
                        })}
                    }
                    // Backlinks panel
                    if selected_id.read().is_some() {
                        div { style: "border-top: 1px solid {c.border}; margin: 8px 16px;" }
                        crate::ui::sidebar::backlinks::BacklinksPanel {
                            note_id: selected_id.read().clone(),
                            db: db_tag4.clone(),
                            on_open: move |note_id: String| {
                                if let Some(ref d) = db_smart2 { if let Ok(Some(n)) = NoteService::new(d).get(&note_id) {
                                    selected_id.set(Some(note_id.clone())); title.set(n.title); content.set(n.content);
                                }}
                            },
                        }
                    }

                    // Calendar panel
                    if *show_calendar.read() {
                        div { style: "border-top: 1px solid {c.border}; margin: 8px 16px;" }
                        crate::ui::sidebar::calendar::CalendarPanel {
                            db: db_tag5.clone(),
                            on_select_date: move |date_str: String| {
                                if let Some(ref d) = db_tag7 {
                                    let svc = NoteService::new(d);
                                    let existing = svc.search(&date_str).ok().and_then(|r| r.into_iter().find(|(n, _)| n.title.contains(&date_str)).map(|(n, _)| n));
                                    if let Some(note) = existing {
                                        selected_id.set(Some(note.id.clone())); title.set(note.title); content.set(note.content);
                                        if let Ok(tags) = TagService::new(d).get_tags_for_note(&note.id) { note_tags.set(tags.iter().map(|t2| t2.name.clone()).collect()); }
                                        if let Ok(h) = HistoryService::new(d).list(&note.id) { snapshots.set(h.iter().map(|s| (s.id.clone(), s.created_at.format("%b %d %H:%M").to_string())).collect()); }
                                    } else {
                                        let parsed = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok();
                                        let date_fmt = parsed.map(|d| d.format("%a %d %b %Y").to_string()).unwrap_or_else(|| date_str.clone());
                                        let template = format!("# Daily Note — {}\n\n## Tasks\n- [ ] \n\n## Notes\n\n## Meetings\n\n", date_fmt);
                                        if let Ok(note) = svc.create(&date_str, &template) {
                                            selected_id.set(Some(note.id.clone())); title.set(note.title.clone()); content.set(note.content.clone());
                                            notes.write().insert(0, note);
                                        }
                                    }
                                }
                            },
                        }
                    }

                    // Tag tree
                    if *view.read() == View::AllNotes {
                        div { style: "border-top: 1px solid {c.border}; margin: 8px 16px;" }
                        crate::ui::sidebar::tag_tree::TagTree { key: "{all_tags.read().len()}", db: db_tag5.clone(), on_search_tag: move |tag_name| { query.set(format!("tag:{}", tag_name)); refresh_notes(&db_side_all2, &View::AllNotes, &notes, &format!("tag:{}", tag_name)); } }
                    }
                }

                // Bottom: export/import + lock
                div { style: "border-top: 1px solid {c.border}; padding: 8px;",
                    SidebarItem { icon: "file_download", label: "Export", active: false, onclick: move |_| { if let Some(ref d) = db_export { let ids: Vec<String> = notes.read().iter().map(|n| n.id.clone()).collect(); let ids_ref: Vec<&str> = ids.iter().map(|s| s.as_str()).collect(); let path = std::path::Path::new("midnight-export.zip"); let _ = ExportService::new(d).export_notes(&ids_ref, path, "export-pass"); } } }
                    SidebarItem { icon: "file_upload", label: "Import", active: false, onclick: move |_| { let path = std::path::Path::new("midnight-export.zip"); if path.exists() { if let Some(ref d) = db_import { let _ = ExportService::new(d).import_notes(path, "export-pass"); let mut n = notes; n.write().clone_from(&vec![]); refresh_notes(&db_import, &View::AllNotes, &notes, ""); } } } }
                    div { style: "height: 1px; background: {c.border}; margin: 4px 0;" }
                    SidebarItem { icon: "settings", label: "Settings", active: *show_settings.read(), onclick: move |_| { let s = *show_settings.read(); show_settings.set(!s); } }
                    SidebarItem { icon: "lock", label: "Lock Vault", active: false, onclick: move |_| on_lock.call(()) }
                }
            }

            // ====== SETTINGS PANEL ======
            if *show_settings.read() {
                crate::ui::settings::Settings {
                    db: db.clone(),
                    on_close: move |_| show_settings.set(false),
                }
            // ====== NOTE LIST ======
            } else if view.read().clone() != View::SmartViews {
                aside { style: "width: 320px; min-width: 320px; border-right: 1px solid {c.border}; background: {c.bg_canvas}; display: flex; flex-direction: column;",
                    div { style: "padding: 12px 16px; border-bottom: 1px solid {c.border};",
                        div { style: "position: relative;",
                            span { class: "material-symbols-outlined", style: "position: absolute; left: 8px; top: 50%; transform: translateY(-50%); font-size: 14px; color: {c.text_secondary};", "search" }
                            input { r#type: "text", placeholder: "Search (e.g. tag:work has:todo)", value: "{query}",
                                oninput: move |e| { query.set(e.value()); refresh_notes(&db_search, &view.read(), &notes, &e.value()); },
                                style: "width: 100%; height: 32px; background: {c.bg_surface}; border: 1px solid {c.border}; border-radius: 4px; padding: 4px 8px 4px 28px; color: {c.text_primary}; font-family: Inter; font-size: 13px; outline: none;",
                            }
                        }
                    }
                    div { style: "flex: 1; overflow-y: auto;",
                        {let di = db.clone(); let db_ctx = db.clone(); notes.read().iter().map(move |note| {
                            let act = selected_id.read().as_deref() == Some(&note.id);
                            let bg = if act { c.bg_surface_high } else { "transparent" };
                            let tc = if act { c.accent } else { c.text_primary };
                            let nid1 = note.id.clone(); let nid2 = note.id.clone(); let nid3 = note.id.clone();
                            let dc1 = di.clone(); let dc2 = di.clone();
                            let dt = db_tag2.clone();
                            let v = view.read().clone();
                            rsx! {
                                 div { key: "{note.id}", style: "padding: 14px 16px; border-bottom: 1px solid {c.border}; background: {bg}; cursor: pointer; position: relative;",
                                     onclick: move |_| { context_note.set(None);
                                        // Clean up whitespace-only title before switching
                                        let cur_title = title.read().clone();
                                        if !cur_title.trim().is_empty() { title.set(cur_title.trim().to_string()); }
                                        if let Some(ref d) = dc1 { if let Ok(Some(n)) = NoteService::new(d).get(&nid1) {
                                            selected_id.set(Some(nid1.clone())); title.set(n.title); content.set(n.content);
                                            if let Some(ref d) = dt { if let Ok(tags) = TagService::new(d).get_tags_for_note(&nid1) { note_tags.set(tags.iter().map(|t2| t2.name.clone()).collect()); } if let Ok(h) = HistoryService::new(d).list(&nid1) { snapshots.set(h.iter().map(|s| (s.id.clone(), s.created_at.format("%b %d %H:%M").to_string())).collect()); } }
                                        }}
                                    },
                                    if act { div { style: "position: absolute; left: 0; top: 0; bottom: 0; width: 3px; background: {c.accent};" } }
                                    div { style: "display: flex; justify-content: space-between; margin-bottom: 4px;",
                                        h3 { style: "font-family: Inter; font-size: 14px; font-weight: 600; color: {tc}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1;", "{note.title}" }
                                        if note.is_pinned { span { class: "material-symbols-outlined fill", style: "font-size: 14px; color: {c.accent};", "push_pin" } }
                                    }
                                    p { style: "font-family: Inter; font-size: 13px; color: {c.text_secondary}; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; margin-bottom: 4px;",
                                        if note.content.is_empty() { "Empty note" } else { "{&note.content}" }
                                    }
                                    // Tags on card — clickable to filter
                                    {let cache = note_tag_cache.read().get(&note.id).cloned().unwrap_or_default();
                                    let db_tc = db_ctx.clone();
                                    if !cache.is_empty() { rsx! {
                                        div { style: "display: flex; gap: 4px; flex-wrap: wrap; margin-bottom: 6px;",
                                            {cache.iter().take(3).map(|t| {
                                                let tn = t.clone();
                                                let d_tc = db_tc.clone();
                                                rsx! {
                                                    span { key: "{t}", style: "font-size: 10px; color: {c.accent}; background: {c.bg_surface_high}; padding: 1px 5px; border-radius: 2px; border: 1px solid {c.accent}; font-family: 'JetBrains Mono', monospace; cursor: pointer;",
                                                        onclick: move |_| { query.set(format!("tag:{}", tn)); refresh_notes(&d_tc, &View::AllNotes, &notes, &format!("tag:{}", tn)); },
                                                        "#{t}"
                                                    }
                                                }
                                            })}
                                            if cache.len() > 3 { span { style: "font-size: 10px; color: {c.text_muted};", "+{cache.len() - 3}" } }
                                        }
                                    }
                                    } else { rsx! {} }}
                                    div { style: "display: flex; justify-content: space-between; align-items: center;",
                                         span { style: "font-family: 'JetBrains Mono', monospace; font-size: 11px; color: {c.text_secondary};", "{fmt_date(note)}" }
                                         button { style: "background: none; border: none; color: {c.text_muted}; cursor: pointer; padding: 2px; font-size: 14px;",
                                             onclick: move |_| context_note.set(Some(nid3.clone())),
                                             span { class: "material-symbols-outlined", style: "font-size: 14px;", "more_vert" }
                                         }
                                         if v == View::Trash {
                                            button { style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; color: {c.error}; background: none; border: none; cursor: pointer;",
                                                onclick: move |_| { if let Some(ref d) = dc2 { let _ = NoteService::new(d).delete_permanently(&nid2); let mut n = notes; n.write().retain(|x| x.id != nid2); } },
                                                "Delete permanently"
                                            }
                                        }
                                    }
                                }
                            }
                        })}
                    if notes.read().is_empty() {
                        div { style: "padding: 32px; text-align: center; font-family: Inter; font-size: 13px; color: {c.text_muted};", p { "{get_empty_msg(&*view.read())}" } }
                    }
                    // Context menu
                    {context_note.read().as_ref().map(move |cid| {
                        let cid_pin = cid.clone(); let cid_arch = cid.clone(); let cid_del = cid.clone();
                        let db_cpin2 = db.clone(); let db_carch2 = db.clone(); let db_cdel2 = db.clone();
                        let db_rp = db.clone(); let db_ra = db.clone(); let db_rd = db.clone();
                        rsx! {
                            div { key: "ctx-overlay", style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; z-index: 999;",
                                onclick: move |_| context_note.set(None),
                                div { style: "position: absolute; top: 10px; left: 10px; background: {c.bg_surface_container}; border: 1px solid {c.border}; border-radius: 4px; padding: 4px 0; min-width: 160px; box-shadow: 0 4px 12px rgba(0,0,0,0.4); z-index: 1000;",
                                    ctx_item { label: "Pin / Unpin", icon: "push_pin", c: c, onclick: move |_| { if let Some(ref d) = db_cpin2 { let _ = NoteService::new(d).toggle_pin(&cid_pin); } context_note.set(None); refresh_notes(&db_rp, &View::AllNotes, &notes, ""); } }
                                    ctx_item { label: "Archive", icon: "archive", c: c, onclick: move |_| { if let Some(ref d) = db_carch2 { let _ = NoteService::new(d).toggle_archive(&cid_arch); } context_note.set(None); refresh_notes(&db_ra, &View::AllNotes, &notes, ""); } }
                                    div { style: "height: 1px; background: {c.border}; margin: 4px 0;" }
                                    ctx_item { label: "Delete permanently", icon: "delete", c: c, onclick: move |_| { if let Some(ref d) = db_cdel2 { let _ = NoteService::new(d).delete_permanently(&cid_del); } context_note.set(None); refresh_notes(&db_rd, &View::AllNotes, &notes, ""); } }
                                }
                            }
                        }
                    })}
                }
            }
            } else if view.read().clone() == View::SmartViews {
                crate::ui::smart_view::SmartViewPanel { db: db.clone(), on_select: move |note_id: String| {
                    if let Some(ref d) = db_smart { if let Ok(Some(n)) = NoteService::new(d).get(&note_id) {
                        view.set(View::AllNotes); selected_id.set(Some(note_id.clone())); title.set(n.title); content.set(n.content);
                        if let Some(ref d) = db_tag3 { if let Ok(tags) = TagService::new(d).get_tags_for_note(&note_id) { note_tags.set(tags.iter().map(|t2| t2.name.clone()).collect()); } if let Ok(h) = HistoryService::new(d).list(&note_id) { snapshots.set(h.iter().map(|s| (s.id.clone(), s.created_at.format("%b %d %H:%M").to_string())).collect()); } }
                    }}
                }}
            }

            // ====== EDITOR ======
            if !*show_settings.read() && view.read().clone() != View::SmartViews {
                section { style: "flex: 1; display: flex; flex-direction: column; min-width: 0; background: {c.bg_primary};",
                    header { style: "height: 56px; background: {c.bg_canvas}; border-bottom: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between; padding: 0 16px;",
                        div { style: "display: flex; background: {c.bg_surface}; border-radius: 4px; border: 1px solid {c.border}; padding: 2px; font-family: 'JetBrains Mono', monospace; font-size: 11px;",
                            ModeBtn { label: "Prose", active: mode() == "Prose", onclick: move |_| mode.set("Prose") }
                            ModeBtn { label: "Code", active: mode() == "Code", onclick: move |_| mode.set("Code") }
                            ModeBtn { label: "Vim", active: mode() == "Vim", onclick: move |_| mode.set("Vim") }
                        }
                        div { style: "display: flex; align-items: center; gap: 6px;",
                            span { style: "display: flex; align-items: center; gap: 4px; font-size: 10px; color: {c.accent_green}; background: {c.bg_surface_high}; padding: 4px 8px; border-radius: 4px; font-family: 'JetBrains Mono', monospace;",
                                span { class: "material-symbols-outlined", style: "font-size: 12px;", "lock" } "Encrypted"
                            }
                            button { style: "color: {c.accent}; background: none; border: none; padding: 4px; cursor: pointer;",
                                onclick: move |_| { if let Some(ref d) = db_pin { let id = selected_id.read().clone(); if let Some(id) = id { let _ = NoteService::new(d).toggle_pin(&id); refresh_notes(&db_pin_r, &view.read(), &notes, ""); } } },
                                span { class: "material-symbols-outlined", style: "font-size: 18px;", "push_pin" }
                            }
                            button { style: "color: {c.text_secondary}; background: none; border: none; padding: 4px; cursor: pointer;",
                                onclick: move |_| { if let Some(ref d) = db_arch { let id = selected_id.read().clone(); if let Some(id) = id { let _ = NoteService::new(d).toggle_archive(&id); selected_id.set(None); title.set(String::new()); content.set(String::new()); refresh_notes(&db_arch_r, &view.read(), &notes, ""); } } },
                                span { class: "material-symbols-outlined", style: "font-size: 18px;", "archive" }
                            }
                            // History button
                            button { style: "color: {c.text_secondary}; background: none; border: none; padding: 4px; cursor: pointer;",
                                onclick: move |_| { let cur = *show_snapshots.read(); show_snapshots.set(!cur); },
                                span { class: "material-symbols-outlined", style: "font-size: 18px;", "history" }
                            }
                            button { style: "background: {c.accent}; color: {c.bg_primary}; border: none; border-radius: 4px; padding: 6px 14px; font-family: 'JetBrains Mono', monospace; font-size: 11px; cursor: pointer;",
                                onclick: move |_| { if let Some(ref d) = db_save { let id = selected_id.read().clone(); if let Some(id) = id { let _ = NoteService::new(d).update(&id, &title.read(), &content.read()); refresh_notes(&db_save_r, &view.read(), &notes, ""); } } },
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
                                    // Version history panel
                                    if *show_snapshots.read() {
                                        div { style: "background: {c.bg_surface_container}; border: 1px solid {c.border}; border-radius: 4px; padding: 12px; margin-bottom: 16px;",
                                            h4 { style: "font-family: Inter; font-size: 14px; font-weight: 600; color: {c.text_primary}; margin-bottom: 8px;", "Version History" }
                                            if snapshots.read().is_empty() { p { style: "font-size: 12px; color: {c.text_muted};", "No previous versions" } }
                                            {snapshots.read().iter().map(|(hid, date)| {
                                                let h = hid.clone();
                                                let db_h = db_hist.clone();
                                                rsx! {
                                                    div { key: "{hid}", style: "display: flex; justify-content: space-between; align-items: center; padding: 6px 0; border-bottom: 1px solid {c.border};",
                                                        span { style: "font-size: 12px; color: {c.text_secondary}; font-family: 'JetBrains Mono', monospace;", "{date}" }
                                                        button { style: "background: {c.accent}; color: {c.bg_primary}; border: none; border-radius: 2px; padding: 2px 8px; font-size: 10px; cursor: pointer;",
                                                            onclick: move |_| { if let Some(ref d) = db_h { if let Ok(n) = HistoryService::new(d).restore(&h) { title.set(n.title); content.set(n.content); show_snapshots.set(false); } } },
                                                            "Restore"
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                    // Trash notification
                                    if view.read().clone() == View::Trash {
                                        div { style: "padding: 12px; background: {c.bg_surface_container}; border: 1px solid {c.border}; border-radius: 4px; margin-bottom: 16px; display: flex; gap: 8px; align-items: center;",
                                            span { style: "font-family: Inter; font-size: 13px; color: {c.text_secondary}; flex: 1;", "This note is in the Trash." }
                                            button { style: "background: {c.accent}; color: {c.bg_primary}; border: none; border-radius: 4px; padding: 4px 12px; font-family: 'JetBrains Mono', monospace; font-size: 10px; cursor: pointer;",
                                                onclick: move |_| { if let Some(ref d) = db_restore { let id = selected_id.read().clone(); if let Some(id) = id { let _ = NoteService::new(d).restore(&id); selected_id.set(None); refresh_notes(&db_restore_r, &View::Trash, &notes, ""); } } },
                                                "Restore"
                                            }
                                            button { style: "background: none; border: 1px solid {c.error}; color: {c.error}; border-radius: 4px; padding: 4px 12px; font-family: 'JetBrains Mono', monospace; font-size: 10px; cursor: pointer;",
                                                onclick: move |_| { if let Some(ref d) = db_del { let id = selected_id.read().clone(); if let Some(id) = id { let _ = NoteService::new(d).delete_permanently(&id); selected_id.set(None); title.set(String::new()); content.set(String::new()); refresh_notes(&db_del_r, &View::Trash, &notes, ""); } } },
                                                "Delete forever"
                                            }
                                        }
                                    }
                                    // Mode-aware editor
                                    if mode() == "Prose" {
                                        crate::ui::editor::prose::ProseEditor {
                                            content: content.read().clone(),
                                            oninput: move |c| content.set(c),
                                            title: title.read().clone(),
                                            ontitleinput: move |t| title.set(t),
                                        }
                                    } else if mode() == "Code" {
                                        crate::ui::editor::code::CodeEditor {
                                            content: content.read().clone(),
                                            oninput: move |c| content.set(c),
                                            title: title.read().clone(),
                                            ontitleinput: move |t| title.set(t),
                                        }
                                    } else {
                                        // Vim mode: title input + textarea + status bar
                                        input { r#type: "text", value: "{title}", oninput: move |e| { let v = e.value(); if v.trim().is_empty() { title.set(String::new()); } else { title.set(v); } },
                                            placeholder: "Untitled",
                                            style: "width: 100%; background: transparent; border: none; font-family: Inter; font-size: 32px; font-weight: 700; letter-spacing: -0.02em; color: {c.text_primary}; margin-bottom: 24px; outline: none;",
                                        }
                                        textarea { value: "{content}", oninput: move |e| content.set(e.value()),
                                            spellcheck: false,
                                            style: "width: 100%; min-height: 60vh; background: transparent; border: none; color: {c.text_primary}; font-family: 'JetBrains Mono', monospace; font-size: 14px; line-height: 1.6; resize: none; outline: none; tab-size: 2;",
                                        }
                                        div { style: "padding: 4px 8px; background: {c.bg_canvas}; border-top: 1px solid {c.border}; display: flex; font-size: 11px; font-family: 'JetBrains Mono', monospace;",
                                            span { style: "color: {c.accent_green}; font-weight: 700;", "INSERT" }
                                            span { style: "color: {c.text_muted}; margin-left: 8px;", "-- INSERT --" }
                                        }
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

                    footer { style: "height: 28px; background: {c.bg_canvas}; border-top: 1px solid {c.border}; display: flex; align-items: center; justify-content: space-between; padding: 0 16px; font-family: 'JetBrains Mono', monospace; font-size: 10px;",
                        div { style: "display: flex; align-items: center; gap: 4px; font-weight: 700; color: {c.accent};",
                            span { class: "material-symbols-outlined fill", style: "font-size: 12px; color: {c.accent_green};", "lock" } "End-to-End Encrypted"
                        }
                        div { style: "display: flex; gap: 16px; color: {c.text_muted};",
                            span { "Words: {content.read().split_whitespace().count()}" }
                            if view.read().clone() == View::Trash { span { style: "color: {c.error};", "Trash" } }
                            if view.read().clone() == View::Archived { span { style: "color: {c.accent_yellow};", "Archived" } }
                        }
                    }
                }
            }
        }
    }
}

fn get_empty_msg(view: &View) -> &'static str {
    match view {
        View::AllNotes => "No notes yet. Create one!",
        View::Archived => "No archived notes",
        View::Trash => "Trash is empty",
        View::SmartViews => "No smart views yet",
    }
}

fn refresh_notes(db: &Option<SharedDb>, view: &View, notes: &Signal<Vec<Note>>, query: &str) {
    if let Some(ref d) = db {
        let result = if !query.is_empty() {
            SearchService::new(d).search(query).map(|r| {
                r.into_iter()
                    .filter_map(|sr| NoteService::new(d).get(&sr.note_id).ok().flatten())
                    .collect()
            })
        } else {
            match view {
                View::Archived => NoteService::new(d).list_archived(),
                View::Trash => NoteService::new(d).list_trashed(),
                _ => NoteService::new(d).list_active(),
            }
        };
        if let Ok(list) = result {
            let mut n = *notes;
            n.write().clone_from(&list);
        }
    }
}

#[component]
fn SidebarItem(icon: String, label: String, active: bool, onclick: EventHandler<()>) -> Element {
    let c = use_theme_colors();
    let bg = if active {
        c.bg_surface_high
    } else {
        "transparent"
    };
    let color = if active { c.accent } else { c.text_secondary };
    let left = if active {
        format!("2px solid {}", c.accent)
    } else {
        "2px solid transparent".to_string()
    };
    rsx! {
        a { style: "display: flex; align-items: center; gap: 12px; padding: 8px 16px; background: {bg}; color: {color}; border-left: {left}; cursor: pointer; font-size: 11px;",
            onclick: move |_| onclick.call(()),
            span { class: "material-symbols-outlined", style: "font-size: 18px;", "{icon}" }
            span { "{label}" }
        }
    }
}

#[component]
#[component]
fn ctx_item(
    label: String,
    icon: String,
    c: crate::ui::theme::ThemeColors,
    onclick: EventHandler<()>,
) -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 8px; padding: 6px 12px; cursor: pointer; font-size: 12px; color: {c.text_secondary}; font-family: Inter; transition: background 0.1s;",
            onclick: move |_| onclick.call(()),
            span { class: "material-symbols-outlined", style: "font-size: 16px;", "{icon}" }
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
        button { style: "padding: 4px 8px; border-radius: 2px; background: {bg}; color: {color}; border: none; cursor: pointer;",
            onclick: move |_| onclick.call(()), "{label}"
        }
    }
}
