use dioxus::prelude::*;

use crate::ui::theme::ThemeProvider;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    VaultLock,
    Workspace,
    Settings,
}

#[component]
pub fn App() -> Element {
    let mut screen = use_signal(|| Screen::VaultLock);

    rsx! {
        ThemeProvider {
            match *screen.read() {
                Screen::VaultLock => rsx! {
                    crate::ui::vault_lock::VaultLock {
                        on_unlock: move || screen.set(Screen::Workspace),
                    }
                },
                Screen::Workspace => rsx! {
                    crate::ui::workspace::Workspace {
                        on_open_settings: move || screen.set(Screen::Settings),
                        on_lock: move || screen.set(Screen::VaultLock),
                    }
                },
                Screen::Settings => rsx! {
                    crate::ui::settings::Settings {
                        on_close: move || screen.set(Screen::Workspace),
                    }
                },
            }
        }
    }
}
