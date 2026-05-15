use crate::core::note::NoteService;
use crate::ui::theme::use_theme_colors;
use chrono::{Datelike, NaiveDate, Utc};
use dioxus::prelude::*;

#[component]
pub fn CalendarPanel(
    db: Option<std::sync::Arc<crate::storage::Database>>,
    on_select_date: EventHandler<String>, // callback with "YYYY-MM-DD"
) -> Element {
    let c = use_theme_colors();
    let today = Utc::now().date_naive();
    let mut view_year = use_signal(|| today.year());
    let mut view_month = use_signal(|| today.month());

    // Compute calendar grid
    let first_day = NaiveDate::from_ymd_opt(*view_year.read(), *view_month.read(), 1).unwrap();
    let last_day = {
        let (y, m) = if *view_month.read() == 12 {
            (*view_year.read() + 1, 1)
        } else {
            (*view_year.read(), *view_month.read() + 1)
        };
        NaiveDate::from_ymd_opt(y, m, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };
    let days_in_month = last_day.day();
    let start_weekday = first_day.weekday().num_days_from_monday(); // 0=Mon, 6=Sun

    let month_name = match *view_month.read() {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    };

    // Check which dates have notes
    let mut note_dates = std::collections::HashSet::<String>::new();
    if let Some(ref d) = db {
        let svc = NoteService::new(d);
        if let Ok(notes) = svc.list_active() {
            for note in notes {
                let date_str = note.updated_at.format("%Y-%m-%d").to_string();
                note_dates.insert(date_str);
            }
        }
    }

    let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    rsx! {
        div { style: "padding: 8px 12px;",
            h4 { style: "font-size: 10px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 6px; font-family: 'JetBrains Mono', monospace;", "Calendar" }

            // Month/Year header with navigation
            div { style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px;",
                button { style: "background: none; border: none; color: {c.accent}; cursor: pointer; font-size: 14px; padding: 2px 4px;",
                    onclick: move |_| { let m = *view_month.read(); let y = *view_year.read(); if m == 1 { view_month.set(12); view_year.set(y - 1); } else { view_month.set(m - 1); } },
                    span { class: "material-symbols-outlined", style: "font-size: 16px;", "chevron_left" }
                }
                div { style: "font-size: 11px; font-weight: 600; color: {c.text_primary}; font-family: Inter;", "{month_name} {view_year.read()}" }
                button { style: "background: none; border: none; color: {c.accent}; cursor: pointer; font-size: 14px; padding: 2px 4px;",
                    onclick: move |_| { let m = *view_month.read(); let y = *view_year.read(); if m == 12 { view_month.set(1); view_year.set(y + 1); } else { view_month.set(m + 1); } },
                    span { class: "material-symbols-outlined", style: "font-size: 16px;", "chevron_right" }
                }
            }

            // Weekday headers
            div { style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 1px; text-align: center; margin-bottom: 4px;",
                {weekdays.iter().map(|d| rsx! {
                    div { key: "{d}", style: "font-size: 9px; color: {c.text_muted}; font-family: 'JetBrains Mono', monospace; padding: 2px 0;", "{d}" }
                })}
            }

            // Day grid
            div { style: "display: grid; grid-template-columns: repeat(7, 1fr); gap: 1px;",
                // Empty cells before first day
                {(0..start_weekday as usize).map(|_| rsx! {
                    div { style: "padding: 3px 0;", "" }
                })}
                // Day cells
                {(1..=days_in_month).map(|day| {
                    let is_today = today.year() == *view_year.read() && today.month() == *view_month.read() && today.day() == day;
                    let date_str = format!("{:04}-{:02}-{:02}", *view_year.read(), *view_month.read(), day);
                    let has_notes = note_dates.contains(&date_str);
                    let bg = if is_today { c.accent } else { "transparent" };
                    let color = if is_today { c.bg_primary } else { c.text_secondary };
                    let dot = if has_notes && !is_today { c.accent_green } else { "transparent" };
                    rsx! {
                        div {
                            key: "{day}",
                            style: "padding: 3px 0; text-align: center; font-size: 10px; font-family: 'JetBrains Mono', monospace; background: {bg}; color: {color}; border-radius: 2px; cursor: pointer; position: relative;",
                            onclick: move |_| on_select_date.call(date_str.clone()),
                            span { "{day}" }
                            if has_notes && !is_today {
                                div { style: "position: absolute; bottom: 1px; left: 50%; transform: translateX(-50%); width: 3px; height: 3px; border-radius: 50%; background: {dot};" }
                            }
                        }
                    }
                })}
            }

            // Today button
            button { style: "width: 100%; margin-top: 8px; background: none; border: 1px solid {c.border}; color: {c.accent}; font-size: 10px; font-family: 'JetBrains Mono', monospace; padding: 4px; border-radius: 2px; cursor: pointer;",
                onclick: move |_| { view_year.set(today.year()); view_month.set(today.month()); on_select_date.call(today.format("%Y-%m-%d").to_string()); },
                "Today: {today.format(\"%b %d, %Y\")}"
            }
        }
    }
}
