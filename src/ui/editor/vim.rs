use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl VimMode {
    pub fn label(&self) -> &'static str {
        match self {
            VimMode::Normal => "NORMAL",
            VimMode::Insert => "INSERT",
            VimMode::Visual => "VISUAL",
            VimMode::Command => "COMMAND",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            VimMode::Normal => "#00dbe9",
            VimMode::Insert => "#00e475",
            VimMode::Visual => "#ffb4ab",
            VimMode::Command => "#ffe179",
        }
    }
}

/// Process a key event and return the new mode and whether to handle the key.
pub fn process_vim_key(current: VimMode, key: &str) -> (VimMode, bool) {
    match current {
        VimMode::Insert => {
            if key == "Escape" {
                (VimMode::Normal, false)
            } else {
                (VimMode::Insert, true)
            }
        }
        VimMode::Normal => match key {
            "i" | "a" | "o" => (VimMode::Insert, false),
            "v" => (VimMode::Visual, false),
            ":" => (VimMode::Command, false),
            "j" | "k" | "h" | "l" => (VimMode::Normal, false),
            _ => (VimMode::Normal, false),
        },
        VimMode::Visual => {
            if key == "Escape" {
                (VimMode::Normal, false)
            } else {
                (VimMode::Visual, false)
            }
        }
        VimMode::Command => {
            if key == "Enter" || key == "Escape" {
                (VimMode::Normal, false)
            } else {
                (VimMode::Command, false)
            }
        }
    }
}

#[component]
pub fn VimStatusBar(mode: VimMode) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; padding: 2px 8px;
                    font-family: 'JetBrains Mono', monospace; font-size: 11px; font-weight: 700;",
            span {
                style: "color: {mode.color()};",
                "{mode.label()}"
            }
            span {
                style: "color: #849495; font-weight: 400;",
                if mode == VimMode::Insert { "-- INSERT --" } else { "" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressing_i_switches_normal_to_insert_mode() {
        assert_eq!(
            process_vim_key(VimMode::Normal, "i"),
            (VimMode::Insert, false)
        );
        assert_eq!(
            process_vim_key(VimMode::Normal, "a"),
            (VimMode::Insert, false)
        );
    }

    #[test]
    fn pressing_escape_switches_insert_to_normal_mode() {
        assert_eq!(
            process_vim_key(VimMode::Insert, "Escape"),
            (VimMode::Normal, false)
        );
    }

    #[test]
    fn character_keys_pass_through_in_insert_mode() {
        assert_eq!(
            process_vim_key(VimMode::Insert, "h"),
            (VimMode::Insert, true)
        );
    }

    #[test]
    fn pressing_v_switches_to_visual_mode() {
        assert_eq!(
            process_vim_key(VimMode::Normal, "v"),
            (VimMode::Visual, false)
        );
    }

    #[test]
    fn pressing_escape_returns_to_normal_mode() {
        assert_eq!(
            process_vim_key(VimMode::Visual, "Escape"),
            (VimMode::Normal, false)
        );
    }

    #[test]
    fn pressing_colon_switches_to_command_mode() {
        assert_eq!(
            process_vim_key(VimMode::Normal, ":"),
            (VimMode::Command, false)
        );
    }

    #[test]
    fn pressing_enter_in_command_mode_returns_to_normal() {
        assert_eq!(
            process_vim_key(VimMode::Command, "Enter"),
            (VimMode::Normal, false)
        );
    }

    #[test]
    fn each_vim_mode_has_correct_label() {
        assert_eq!(VimMode::Normal.label(), "NORMAL");
        assert_eq!(VimMode::Insert.label(), "INSERT");
        assert_eq!(VimMode::Visual.label(), "VISUAL");
        assert_eq!(VimMode::Command.label(), "COMMAND");
    }
}
