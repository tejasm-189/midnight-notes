use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    VaultLock,
    Workspace,
}

#[component]
pub fn App() -> Element {
    let mut screen = use_signal(|| Screen::VaultLock);

    rsx! {
        match *screen.read() {
            Screen::VaultLock => rsx! {
                crate::ui::vault_lock::VaultLock {
                    on_unlock: move || {
                        screen.set(Screen::Workspace);
                    }
                }
            },
            Screen::Workspace => rsx! {
                crate::ui::workspace::Workspace {
                    on_lock: move || {
                        screen.set(Screen::VaultLock);
                    }
                }
            },
        }
    }
}
