use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Midnight,
    Dark,
    Light,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::Midnight => "Midnight",
            Theme::Dark => "Dark",
            Theme::Light => "Light",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Theme::Midnight => "nightlight",
            Theme::Dark => "dark_mode",
            Theme::Light => "light_mode",
        }
    }

    pub fn css_vars(&self) -> &'static str {
        match self {
            Theme::Midnight => MIDNIGHT_VARS,
            Theme::Dark => DARK_VARS,
            Theme::Light => LIGHT_VARS,
        }
    }
}

/// Must be called at the root of the component tree to provide the theme.
pub fn use_theme_provider() {
    use_context_provider(|| Signal::new(Theme::Midnight));
}

/// Get the current theme signal from context.
pub fn use_theme_signal() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
}

const MIDNIGHT_VARS: &str = "\
--bg-primary: #000000;\
--bg-canvas: #0e0e0e;\
--bg-surface: #131313;\
--bg-surface-low: #1c1b1b;\
--bg-surface-container: #201f1f;\
--bg-surface-high: #2a2a2a;\
--bg-surface-highest: #353534;\
--text-primary: #e5e2e1;\
--text-secondary: #b9cacb;\
--text-muted: #849495;\
--accent: #00dbe9;\
--accent-green: #00e475;\
--accent-yellow: #ffe179;\
--border: #3b494b;\
--border-light: #849495;\
--error: #ffb4ab;\
--selection: rgba(0, 219, 233, 0.3);";

const DARK_VARS: &str = "\
--bg-primary: #1e1e1e;\
--bg-canvas: #252526;\
--bg-surface: #2d2d2d;\
--bg-surface-low: #333333;\
--bg-surface-container: #3c3c3c;\
--bg-surface-high: #454545;\
--bg-surface-highest: #505050;\
--text-primary: #d4d4d4;\
--text-secondary: #aaaaaa;\
--text-muted: #808080;\
--accent: #569cd6;\
--accent-green: #6a9955;\
--accent-yellow: #dcdcaa;\
--border: #404040;\
--border-light: #606060;\
--error: #f44747;\
--selection: rgba(86, 156, 214, 0.3);";

const LIGHT_VARS: &str = "\
--bg-primary: #ffffff;\
--bg-canvas: #f5f5f5;\
--bg-surface: #ffffff;\
--bg-surface-low: #f0f0f0;\
--bg-surface-container: #e8e8e8;\
--bg-surface-high: #e0e0e0;\
--bg-surface-highest: #d0d0d0;\
--text-primary: #1c1b1b;\
--text-secondary: #616161;\
--text-muted: #9e9e9e;\
--accent: #006970;\
--accent-green: #2e7d32;\
--accent-yellow: #f9a825;\
--border: #c0c0c0;\
--border-light: #e0e0e0;\
--error: #d32f2f;\
--selection: rgba(0, 105, 112, 0.2);";

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    use_theme_provider();
    let theme = use_theme_signal();

    rsx! {
        div {
            style: "{theme.read().css_vars()} width: 100vw; height: 100vh;
                   background: var(--bg-primary); color: var(--text-primary);",
            {children}
        }
    }
}
