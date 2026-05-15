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
                bg_surface_low: "#f0efee",
                bg_surface_container: "#e8e6e4",
                bg_surface_high: "#dddcd9",
                bg_surface_highest: "#d0cecb",
                text_primary: "#1c1b1b",
                text_secondary: "#5f5e5e",
                text_muted: "#939090",
                accent: "#006970",
                accent_green: "#2e7d32",
                accent_yellow: "#b8860b",
                border: "#c4c2bf",
                border_light: "#dddcd9",
                error: "#ba1a1a",
            },
        }
    }

    pub fn css_vars(&self) -> String {
        let c = self.colors();
        format!(
            "--bg-primary: {};\
             --bg-canvas: {};\
             --bg-surface: {};\
             --bg-surface-low: {};\
             --bg-surface-container: {};\
             --bg-surface-high: {};\
             --bg-surface-highest: {};\
             --text-primary: {};\
             --text-secondary: {};\
             --text-muted: {};\
             --accent: {};\
             --accent-green: {};\
             --accent-yellow: {};\
             --border: {};\
             --border-light: {};\
             --error: {};\
             --selection: {};",
            c.bg_primary,
            c.bg_canvas,
            c.bg_surface,
            c.bg_surface_low,
            c.bg_surface_container,
            c.bg_surface_high,
            c.bg_surface_highest,
            c.text_primary,
            c.text_secondary,
            c.text_muted,
            c.accent,
            c.accent_green,
            c.accent_yellow,
            c.border,
            c.border_light,
            c.error,
            match self {
                Theme::Midnight => "rgba(0, 219, 233, 0.3)",
                Theme::Dark => "rgba(86, 156, 214, 0.3)",
                Theme::Light => "rgba(0, 105, 112, 0.2)",
            }
        )
    }
}

/// Must be called at the root of the component tree to provide the theme.
pub fn use_init_theme() -> Signal<Theme> {
    use_context_provider(|| Signal::new(Theme::Midnight))
}

/// Get the current theme signal from context.
pub fn use_theme_signal() -> Signal<Theme> {
    use_context::<Signal<Theme>>()
}

/// Get the current theme colors.
pub fn use_theme_colors() -> ThemeColors {
    use_theme_signal().read().colors()
}

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let t_signal = use_theme_signal();
    let c = t_signal.read().colors();
    let vars = t_signal.read().css_vars();

    rsx! {
        div {
            id: "theme-root",
            style: "width: 100vw; height: 100vh; background: {c.bg_primary}; color: {c.text_primary}; {vars}",
            {children}
        }
    }
}
