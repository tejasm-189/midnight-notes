use pulldown_cmark::{html, Event, Options, Parser};

/// Render markdown text to HTML for display.
pub fn render_markdown(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let mut html_output = String::new();
    let parser = Parser::new_ext(text, options);
    html::push_html(&mut html_output, parser);
    html_output
}

/// Extract plain text summary from markdown (for preview snippets).
pub fn plain_text_summary(text: &str, max_chars: usize) -> String {
    let parser = Parser::new(text);
    let mut out = String::new();
    for event in parser {
        match event {
            Event::Text(t) => {
                out.push_str(&t);
                if out.len() > max_chars {
                    out.truncate(max_chars);
                    out.push('…');
                    break;
                }
            }
            Event::Code(t) => {
                out.push_str(&t);
            }
            _ => {}
        }
    }
    out
}

/// Check if text contains LaTeX math delimiters.
pub fn contains_math(text: &str) -> bool {
    text.contains("$$") || text.contains('$')
}

/// Extract [[wiki-link]] titles from markdown text.
pub fn extract_wiki_links(text: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    re.captures_iter(text)
        .map(|c| c[1].trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_paragraph() {
        let html = render_markdown("Hello **world**");
        assert!(html.contains("<strong>world</strong>"));
    }

    #[test]
    fn test_render_table() {
        let md = "| A | B |\n|---| --- |\n| 1 | 2 |";
        let html = render_markdown(md);
        assert!(html.contains("<th>"));
        assert!(html.contains("<td>"));
    }

    #[test]
    fn test_render_tasklist() {
        let md = "- [x] done\n- [ ] todo";
        let html = render_markdown(md);
        assert!(html.contains("checked"));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn test_plain_text_summary() {
        let md = "# Hello\nThis is a **note**.";
        let summary = plain_text_summary(md, 10);
        assert!(summary.starts_with("HelloThis "));
        assert!(summary.ends_with('…'));
    }

    #[test]
    fn test_plain_text_summary_short() {
        let md = "Hi";
        let summary = plain_text_summary(md, 100);
        assert_eq!(summary, "Hi");
    }

    #[test]
    fn test_plain_text_summary_exact() {
        let summary = plain_text_summary("Hello World", 11);
        assert_eq!(summary, "Hello World");
    }

    #[test]
    fn test_contains_math_inline() {
        assert!(contains_math("The value is $x^2$"));
    }

    #[test]
    fn test_contains_math_block() {
        assert!(contains_math("$$\\int_0^1 x^2 dx$$"));
    }

    #[test]
    fn test_contains_math_none() {
        assert!(!contains_math("Just plain text"));
    }

    #[test]
    fn test_extract_wiki_links() {
        let links = extract_wiki_links("See [[Note A]] and [[Note B]].");
        assert_eq!(links, vec!["Note A", "Note B"]);
    }

    #[test]
    fn test_render_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let html = render_markdown(md);
        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_render_strikethrough() {
        let html = render_markdown("~~strikethrough~~");
        assert!(html.contains("<del>"));
    }
}
