use dioxus::prelude::*;

const SIDEBAR_STYLE: &str = "width: 256px; min-width: 256px; height: 100vh; background: #131313;
    border-right: 1px solid #3b494b; display: flex; flex-direction: column; padding: 16px 0;";

const NAV_ITEM: &str = "display: flex; align-items: center; gap: 12px; padding: 8px 16px;
    font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500;
    letter-spacing: 0.05em; color: #b9cacb; cursor: pointer; border-left: 2px solid transparent;
    transition: all 0.15s;";

const NAV_ITEM_ACTIVE: &str = "display: flex; align-items: center; gap: 12px; padding: 8px 16px;
    font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500;
    letter-spacing: 0.05em; color: #00dbe9; cursor: pointer; border-left: 2px solid #00dbe9;
    background: #2a2a2a;";

#[component]
pub fn Workspace(on_lock: EventHandler<()>) -> Element {
    let mut mode = use_signal(|| "Prose");

    rsx! {
        div {
            style: "display: flex; height: 100vh; background: #131313;",

            // Sidebar
            nav {
                style: "{SIDEBAR_STYLE}",
                div {
                    style: "padding: 0 16px; margin-bottom: 24px; display: flex; align-items: center; gap: 8px;",
                    span {
                        class: "material-symbols-outlined fill",
                        style: "font-size: 24px; color: #00dbe9;",
                        "description"
                    }
                    div {
                        h1 { style: "font-family: Inter; font-size: 20px; font-weight: 600; color: #00dbe9;", "Midnight Notes" }
                        p { style: "font-family: 'JetBrains Mono', monospace; font-size: 12px; color: #b9cacb;", "Local-first Sync" }
                    }
                }

                // New Note button
                button {
                    style: "margin: 0 16px 16px; display: flex; align-items: center; justify-content: center;
                            gap: 8px; padding: 8px; background: #00dbe9; color: #002022; border: none; border-radius: 4px;
                            font-family: 'JetBrains Mono', monospace; font-size: 12px; font-weight: 500; cursor: pointer;
                            letter-spacing: 0.05em;",
                    span { class: "material-symbols-outlined", style: "font-size: 16px;", "add" }
                    "New Note"
                }

                // Nav items
                div { style: "flex: 1; display: flex; flex-direction: column; gap: 2px; padding: 0 8px;",
                    NavItem { icon: "description", label: "All Notes", active: true }
                    NavItem { icon: "auto_awesome", label: "Smart Views", active: false }
                    div { style: "height: 1px; background: #3b494b; margin: 8px 16px;" }
                    NavItem { icon: "archive", label: "Archived", active: false }
                    NavItem { icon: "delete", label: "Trash", active: false }
                    NavItem { icon: "lock", label: "Encrypted", active: false }
                    NavItem { icon: "settings", label: "Settings", active: false }
                }

                // Bottom nav
                div {
                    style: "border-top: 1px solid #3b494b; padding: 8px 8px 0; display: flex; flex-direction: column; gap: 2px;",
                    NavItem { icon: "help", label: "Help", active: false }
                    NavItem { icon: "sensors", label: "Status", active: false }
                }
            }

            // Note list (hidden on small screens)
            aside {
                style: "width: 320px; min-width: 320px; border-right: 1px solid #3b494b;
                        background: #0e0e0e; display: flex; flex-direction: column;",

                // Search bar
                div {
                    style: "padding: 16px; border-bottom: 1px solid #3b494b;",
                    div {
                        style: "position: relative;",
                        span {
                            class: "material-symbols-outlined",
                            style: "position: absolute; left: 8px; top: 50%; transform: translateY(-50%);
                                    font-size: 14px; color: #b9cacb;",
                            "search"
                        }
                        input {
                            r#type: "text",
                            placeholder: "Search notes...",
                            style: "width: 100%; height: 32px; background: #131313; border: 1px solid #3b494b;
                                    border-radius: 4px; padding: 4px 8px 4px 28px; color: #e5e2e1;
                                    font-family: Inter; font-size: 14px;",
                        }
                    }
                }

                // Note list
                div { style: "flex: 1; overflow-y: auto;",
                    NoteListItem {
                        title: "API Endpoint Refactoring",
                        preview: "Need to update the v2 endpoints to handle the new authentication token format...",
                        date: "Today, 10:42 AM",
                        tag: "#work",
                        pinned: true,
                        active: true,
                    }
                    NoteListItem {
                        title: "Weekly Sync Notes",
                        preview: "Discussed the upcoming roadmap for Q3...",
                        date: "Yesterday",
                        tag: "#meetings",
                        pinned: false,
                        active: false,
                    }
                    NoteListItem {
                        title: "Docker Config Snippets",
                        preview: "docker-compose.yml setup for the local redis and postgres instances...",
                        date: "Oct 12",
                        tag: "#devops",
                        pinned: false,
                        active: false,
                    }
                }
            }

            // Editor
            section {
                style: "flex: 1; display: flex; flex-direction: column; min-width: 0; background: #000;",

                // Editor header
                header {
                    style: "height: 64px; min-height: 64px; background: #0e0e0e; border-bottom: 1px solid #3b494b;
                            display: flex; align-items: center; justify-content: space-between; padding: 0 16px;",

                    // Mode switcher
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        div {
                            style: "display: flex; background: #131313; border-radius: 4px; border: 1px solid #3b494b; padding: 2px;",
                            ModeButton { label: "Prose", active: mode() == "Prose", onclick: move |_| mode.set("Prose") }
                            ModeButton { label: "Code", active: mode() == "Code", onclick: move |_| mode.set("Code") }
                            ModeButton { label: "Vim", active: mode() == "Vim", onclick: move |_| mode.set("Vim") }
                        }
                    }

                    // Right side actions
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span {
                            style: "display: flex; align-items: center; gap: 4px; font-family: 'JetBrains Mono', monospace;
                                    font-size: 12px; font-weight: 500; color: #00e475; background: #2a2a2a;
                                    padding: 4px 8px; border-radius: 4px; border: 1px solid #3b494b;",
                            span { class: "material-symbols-outlined", style: "font-size: 14px;", "lock" }
                            "Encrypted"
                        }
                        IconButton { icon: "push_pin" }
                        IconButton { icon: "more_vert" }
                        button {
                            style: "background: #00dbe9; color: #002022; border: none; border-radius: 4px;
                                    padding: 6px 12px; font-family: 'JetBrains Mono', monospace; font-size: 12px;
                                    font-weight: 500; cursor: pointer; letter-spacing: 0.05em;",
                            "Save"
                        }
                    }
                }

                // Editor canvas
                div {
                    style: "flex: 1; overflow-y: auto; display: flex; justify-content: center; padding-bottom: 40px;",
                    div {
                        style: "width: 100%; max-width: 840px; padding: 24px 32px; position: relative;",

                        // Line numbers
                        div {
                            style: "position: absolute; left: 0; top: 0; bottom: 0; width: 32px;
                                    border-right: 1px solid #3b494b; display: flex; flex-direction: column;
                                    align-items: flex-end; padding: 24px 8px 24px 0;
                                    font-family: 'JetBrains Mono', monospace; font-size: 14px;
                                    color: #3b494b; opacity: 0.3; user-select: none;",
                            { (1..=15).map(|i| rsx! { span { "{i}" } }) }
                        }

                        // Editor content
                        div {
                            style: "padding-left: 48px; width: 100%;",
                            input {
                                r#type: "text",
                                value: "API Endpoint Refactoring",
                                style: "width: 100%; background: transparent; border: none;
                                        font-family: Inter; font-size: 32px; font-weight: 700;
                                        letter-spacing: -0.02em; color: #e5e2e1;
                                        margin-bottom: 16px; padding: 0;",
                            }
                            div {
                                style: "font-family: Inter; font-size: 16px; line-height: 24px; color: #e5e2e1;",
                                p {
                                    style: "line-height: 1.6; margin-bottom: 16px;",
                                    "Need to update the v2 endpoints to handle the new authentication token format before the weekend deployment."
                                }
                                h3 {
                                    style: "font-size: 24px; font-weight: 600; line-height: 32px;
                                            letter-spacing: -0.01em; margin: 24px 0 8px;",
                                    "Tasks"
                                }
                                ul {
                                    style: "display: flex; flex-direction: column; gap: 8px; list-style: none; padding: 0;",
                                    li {
                                        style: "display: flex; align-items: flex-start; gap: 8px;",
                                        div {
                                            style: "width: 16px; height: 16px; min-width: 16px; border-radius: 4px;
                                                    background: #00dbe9; display: flex; align-items: center; justify-content: center; margin-top: 4px;",
                                            span { class: "material-symbols-outlined fill", style: "font-size: 12px; color: #002022;", "check" }
                                        }
                                        span { style: "text-decoration: line-through; color: #b9cacb;", "Update middleware validation logic" }
                                    }
                                    li {
                                        style: "display: flex; align-items: flex-start; gap: 8px;",
                                        div { style: "width: 16px; height: 16px; min-width: 16px; border-radius: 4px; border: 1px solid #3b494b; margin-top: 4px;" }
                                        span { "Modify AuthService.ts to decode new payload" }
                                    }
                                    li {
                                        style: "display: flex; align-items: flex-start; gap: 8px;",
                                        div { style: "width: 16px; height: 16px; min-width: 16px; border-radius: 4px; border: 1px solid #3b494b; margin-top: 4px;" }
                                        span { "Write unit tests for scope checking" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavItem(icon: String, label: String, active: bool) -> Element {
    rsx! {
        a {
            style: if active { NAV_ITEM_ACTIVE } else { NAV_ITEM },
            span {
                class: "material-symbols-outlined",
                style: "font-size: 18px;",
                "{icon}"
            }
            span { "{label}" }
        }
    }
}

#[component]
fn NoteListItem(
    title: String,
    preview: String,
    date: String,
    tag: String,
    pinned: bool,
    active: bool,
) -> Element {
    let bg = if active { "#2a2a2a" } else { "#0e0e0e" };
    let title_color = if active { "#00dbe9" } else { "#e5e2e1" };

    rsx! {
        div {
            style: "padding: 16px; border-bottom: 1px solid #3b494b; background: {bg}; cursor: pointer; position: relative;",
            if active {
                div { style: "position: absolute; left: 0; top: 0; bottom: 0; width: 4px; background: #00dbe9;" }
            }
            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 4px;",
                h3 {
                    style: "font-family: Inter; font-size: 14px; font-weight: 600; color: {title_color}; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                    "{title}"
                }
                if pinned {
                    span { class: "material-symbols-outlined fill", style: "font-size: 14px; color: #00dbe9;", "push_pin" }
                }
            }
            p {
                style: "font-family: Inter; font-size: 14px; color: #b9cacb; display: -webkit-box;
                        -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; margin-bottom: 8px;",
                "{preview}"
            }
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                span { style: "font-family: 'JetBrains Mono', monospace; font-size: 12px; color: #b9cacb;", "{date}" }
                span {
                    style: "font-family: 'JetBrains Mono', monospace; font-size: 10px; color: #b9cacb;
                            background: #131313; padding: 2px 6px; border-radius: 2px; border: 1px solid #3b494b;",
                    "{tag}"
                }
            }
        }
    }
}

#[component]
fn ModeButton(label: String, active: bool, onclick: EventHandler<()>) -> Element {
    let bg = if active { "#2a2a2a" } else { "transparent" };
    let color = if active { "#00dbe9" } else { "#b9cacb" };

    rsx! {
        button {
            style: "padding: 4px 8px; border-radius: 2px; background: {bg}; color: {color};
                    border: none; cursor: pointer; font-family: 'JetBrains Mono', monospace;
                    font-size: 12px; font-weight: 500; letter-spacing: 0.05em; transition: all 0.15s;",
            onclick: move |_| onclick.call(()),
            "{label}"
        }
    }
}

#[component]
fn IconButton(icon: String) -> Element {
    rsx! {
        button {
            style: "color: #b9cacb; background: none; border: none; padding: 4px; cursor: pointer;
                    border-radius: 4px; display: flex; align-items: center; justify-content: center;
                    transition: all 0.15s;",
            span { class: "material-symbols-outlined", style: "font-size: 20px;", "{icon}" }
        }
    }
}
