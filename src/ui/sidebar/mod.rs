pub mod backlinks;
pub mod calendar;
pub mod smart_views;
pub mod tag_tree;

use crate::ui::theme::use_theme_colors;
use dioxus::prelude::*;

#[component]
pub fn SidebarItem(
    icon: String,
    label: String,
    active: bool,
    onclick: EventHandler<()>,
) -> Element {
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
        a { style: "display: flex; align-items: center; gap: 12px; padding: 8px 16px; background: {bg}; color: {color}; border-left: {left}; cursor: pointer; font-size: 11px; letter-spacing: 0.03em; font-family: 'JetBrains Mono', monospace;",
            onclick: move |_| onclick.call(()),
            span { class: "material-symbols-outlined", style: "font-size: 18px;", "{icon}" }
            span { "{label}" }
        }
    }
}
