use dioxus::prelude::*;
use std::sync::Arc;

use crate::storage::Database;
use crate::ui::theme::ThemeProvider;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    VaultLock,
    Workspace,
    Settings,
}

pub type SharedDb = Arc<Database>;

#[component]
pub fn App() -> Element {
    let mut theme = crate::ui::theme::use_init_theme();
    let mut screen = use_signal(|| Screen::VaultLock);
    let mut db: Signal<Option<SharedDb>> = use_signal(|| None);

    // Load theme from DB when it becomes available
    use_effect(move || {
        if let Some(ref db_val) = *db.read() {
            let conn = db_val.conn();
            if let Ok(saved) =
                conn.query_row("SELECT value FROM meta WHERE key = 'theme'", [], |row| {
                    row.get::<_, String>(0)
                })
            {
                let t = match saved.as_str() {
                    "dark" => crate::ui::theme::Theme::Dark,
                    "light" => crate::ui::theme::Theme::Light,
                    _ => crate::ui::theme::Theme::Midnight,
                };
                theme.set(t);
            }
        }
    });

    rsx! {
        ThemeProvider {
            match *screen.read() {
                Screen::VaultLock => rsx! {
                    crate::ui::vault_lock::VaultLock {
                        on_unlock: move |_password: String| {
                            // Attempt to open/create the vault
                            let vault_dir = dirs::data_dir()
                                .map(|p| p.join("MidnightNotes"))
                                .unwrap_or_else(|| std::path::PathBuf::from("./vault"));
                            std::fs::create_dir_all(&vault_dir).ok();
                            let db_path = vault_dir.join("vault.db");

                            match Database::open(&db_path) {
                                Ok(database) => {
                                    let shared = Arc::new(database);
                                    db.set(Some(shared));
                                    screen.set(Screen::Workspace);
                                }
                                Err(e) => {
                                    tracing::error!("failed to open vault: {e}");
                                }
                            }
                        },
                    }
                },
                Screen::Workspace => rsx! {
                    crate::ui::workspace::Workspace {
                        db: db.read().clone(),
                        on_open_settings: move || screen.set(Screen::Settings),
                        on_lock: move || {
                            db.set(None);
                            screen.set(Screen::VaultLock);
                        },
                    }
                },
                Screen::Settings => rsx! {
                    crate::ui::settings::Settings {
                        db: db.read().clone(),
                        on_close: move || screen.set(Screen::Workspace),
                    }
                },
            }
        }
    }
}
