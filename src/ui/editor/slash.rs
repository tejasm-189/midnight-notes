use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SlashCommand {
    pub trigger: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub insert: &'static str,
}

pub const SLASH_COMMANDS: &[SlashCommand] = &[
    SlashCommand {
        trigger: "table",
        label: "Table",
        icon: "table",
        insert: "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |",
    },
    SlashCommand {
        trigger: "code",
        label: "Code Block",
        icon: "code",
        insert: "```\n\n```",
    },
    SlashCommand {
        trigger: "image",
        label: "Image",
        icon: "image",
        insert: "![alt text](url)",
    },
    SlashCommand {
        trigger: "link",
        label: "Link",
        icon: "link",
        insert: "[text](url)",
    },
    SlashCommand {
        trigger: "math",
        label: "Math (inline)",
        icon: "functions",
        insert: "$$",
    },
    SlashCommand {
        trigger: "blockmath",
        label: "Math (block)",
        icon: "functions",
        insert: "$$\n\n$$",
    },
    SlashCommand {
        trigger: "todo",
        label: "Todo List",
        icon: "checklist",
        insert: "- [ ] ",
    },
    SlashCommand {
        trigger: "list",
        label: "Bullet List",
        icon: "list",
        insert: "- ",
    },
    SlashCommand {
        trigger: "numbered",
        label: "Numbered List",
        icon: "format_list_numbered",
        insert: "1. ",
    },
    SlashCommand {
        trigger: "quote",
        label: "Blockquote",
        icon: "format_quote",
        insert: "> ",
    },
    SlashCommand {
        trigger: "hr",
        label: "Horizontal Rule",
        icon: "horizontal_rule",
        insert: "\n---\n",
    },
    SlashCommand {
        trigger: "heading",
        label: "Heading",
        icon: "title",
        insert: "## ",
    },
];

pub fn filter_commands(query: &str) -> Vec<&'static SlashCommand> {
    let q = query.to_lowercase();
    SLASH_COMMANDS
        .iter()
        .filter(|c| c.trigger.contains(&q) || c.label.to_lowercase().contains(&q))
        .collect()
}

pub fn find_exact_command(trigger: &str) -> Option<&'static SlashCommand> {
    SLASH_COMMANDS.iter().find(|c| c.trigger == trigger)
}

#[component]
pub fn SlashMenu(
    query: String,
    onselect: EventHandler<String>,
    onclose: EventHandler<()>,
) -> Element {
    let results = filter_commands(&query);
    let selection = use_signal(|| 0usize);

    rsx! {
        div {
            style: "position: absolute; bottom: 100%; left: 0; background: #1c1b1b;
                    border: 1px solid #3b494b; border-radius: 8px; padding: 8px 0;
                    min-width: 240px; max-height: 320px; overflow-y: auto; z-index: 100;
                    box-shadow: 0 8px 24px rgba(0,0,0,0.5);",

            div {
                style: "padding: 4px 12px 8px; font-family: 'JetBrains Mono', monospace;
                        font-size: 10px; color: #849495; border-bottom: 1px solid #3b494b; margin-bottom: 4px;",
                "Commands"
            }

            {results.iter().enumerate().map(|(i, cmd)| {
                let bg = if i == *selection.read() { "#2a2a2a" } else { "transparent" };
                let insert = cmd.insert.to_string();
                rsx! {
                    div {
                        key: "{cmd.trigger}",
                        style: "display: flex; align-items: center; gap: 8px; padding: 6px 12px;
                                cursor: pointer; font-family: Inter; font-size: 14px; color: #e5e2e1;
                                background: {bg};",
                        onclick: move |_| onselect.call(insert.clone()),
                        span {
                            class: "material-symbols-outlined",
                            style: "font-size: 18px; color: #00dbe9;",
                            "{cmd.icon}"
                        }
                        span { "{cmd.label}" }
                    }
                }
            })}

            if results.is_empty() {
                div {
                    style: "padding: 12px; text-align: center; font-family: Inter; font-size: 13px; color: #849495;",
                    "No matching commands"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_returns_all_commands() {
        let results = filter_commands("");
        assert_eq!(results.len(), SLASH_COMMANDS.len());
    }

    #[test]
    fn tab_query_matches_table_command() {
        let results = filter_commands("tab");
        assert!(results.iter().any(|c| c.trigger == "table"));
    }

    #[test]
    fn code_query_matches_code_block_command() {
        let results = filter_commands("code");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].trigger, "code");
    }

    #[test]
    fn unknown_query_returns_empty_results() {
        let results = filter_commands("xyznonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn exact_trigger_match_finds_command() {
        assert!(find_exact_command("table").is_some());
        assert!(find_exact_command("nonexistent").is_none());
    }
}
