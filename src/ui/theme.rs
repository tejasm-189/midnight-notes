use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeColors {
    pub bg_primary: &'static str,
    pub bg_canvas: &'static str,
    pub bg_surface: &'static str,
    pub bg_surface_low: &'static str,
    pub bg_surface_container: &'static str,
    pub bg_surface_high: &'static str,
    pub bg_surface_highest: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,
    pub accent: &'static str,
    pub accent_green: &'static str,
    pub accent_yellow: &'static str,
    pub border: &'static str,
    pub border_light: &'static str,
    pub error: &'static str,
}

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

    pub fn colors(&self) -> ThemeColors {
        match self {
            Theme::Midnight => ThemeColors {
                bg_primary: "#000000",
                bg_canvas: "#0e0e0e",
                bg_surface: "#131313",
                bg_surface_low: "#1c1b1b",
                bg_surface_container: "#201f1f",
                bg_surface_high: "#2a2a2a",
                bg_surface_highest: "#353534",
                text_primary: "#e5e2e1",
                text_secondary: "#b9cacb",
                text_muted: "#849495",
                accent: "#00dbe9",
                accent_green: "#00e475",
                accent_yellow: "#ffe179",
                border: "#3b494b",
                border_light: "#849495",
                error: "#ffb4ab",
            },
            Theme::Dark => ThemeColors {
                bg_primary: "#1e1e1e",
                bg_canvas: "#252526",
                bg_surface: "#2d2d2d",
                bg_surface_low: "#333333",
                bg_surface_container: "#3c3c3c",
                bg_surface_high: "#454545",
                bg_surface_highest: "#505050",
                text_primary: "#d4d4d4",
                text_secondary: "#aaaaaa",
                text_muted: "#808080",
                accent: "#569cd6",
                accent_green: "#6a9955",
                accent_yellow: "#dcdcaa",
                border: "#404040",
                border_light: "#606060",
                error: "#f44747",
            },
            Theme::Light => ThemeColors {
                bg_primary: "#ffffff",
                bg_canvas: "#f5f5f5",
                bg_surface: "#ffffff",
                bg_surface_low: "#f0f0f0",
                bg_surface_container: "#e8e8e8",
                bg_surface_high: "#e0e0e0",
                bg_surface_highest: "#d0d0d0",
                text_primary: "#1c1b1b",
                text_secondary: "#616161",
                text_muted: "#9e9e9e",
                accent: "#006970",
                accent_green: "#2e7d32",
                accent_yellow: "#f9a825",
                border: "#c0c0c0",
                border_light: "#e0e0e0",
                error: "#d32f2f",
            },
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

/// Get the current theme colors.
pub fn use_theme_colors() -> ThemeColors {
    use_theme_signal().read().colors()
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
    let c = use_theme_colors();

    rsx! {
        div {
            style: "width: 100vw; height: 100vh; background: {c.bg_primary}; color: {c.text_primary};",
            {children}
        }
    }
}
