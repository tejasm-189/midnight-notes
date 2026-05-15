use pulldown_cmark::{html, Event, Options, Parser};

/// Render markdown text to HTML for display.
///
/// ```
/// use midnight_notes::core::markdown::render_markdown;
/// let html = render_markdown("# Hello\n**world**");
/// assert!(html.contains("<h1>"));
/// assert!(html.contains("<strong>"));
/// ```
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
    fn rendering_bold_markdown_produces_strong_tag() {
        let html = render_markdown("Hello **world**");
        assert!(html.contains("<strong>world</strong>"));
    }

    #[test]
    fn rendering_table_produces_th_and_td_tags() {
        let md = "| A | B |\n|---| --- |\n| 1 | 2 |";
        let html = render_markdown(md);
        assert!(html.contains("<th>"));
        assert!(html.contains("<td>"));
    }

    #[test]
    fn rendering_task_list_shows_checked_items() {
        let md = "- [x] done\n- [ ] todo";
        let html = render_markdown(md);
        assert!(html.contains("checked"));
        assert!(html.contains("disabled"));
    }

    #[test]
    fn extracting_plain_text_truncates_at_limit() {
        let md = "# Hello\nThis is a **note**.";
        let summary = plain_text_summary(md, 10);
        assert!(summary.starts_with("HelloThis "));
        assert!(summary.ends_with('…'));
    }

    #[test]
    fn short_text_does_not_get_truncated() {
        let md = "Hi";
        let summary = plain_text_summary(md, 100);
        assert_eq!(summary, "Hi");
    }

    #[test]
    fn exact_length_text_does_not_add_ellipsis() {
        let summary = plain_text_summary("Hello World", 11);
        assert_eq!(summary, "Hello World");
    }

    #[test]
    fn dollar_signs_detect_inline_math() {
        assert!(contains_math("The value is $x^2$"));
    }

    #[test]
    fn double_dollar_signs_detect_block_math() {
        assert!(contains_math("$$\\int_0^1 x^2 dx$$"));
    }

    #[test]
    fn plain_text_returns_false_for_math_check() {
        assert!(!contains_math("Just plain text"));
    }

    #[test]
    fn extracting_wiki_links_from_markdown_works() {
        let links = extract_wiki_links("See [[Note A]] and [[Note B]].");
        assert_eq!(links, vec!["Note A", "Note B"]);
    }

    #[test]
    fn rendering_code_block_produces_code_tag() {
        let md = "```rust\nfn main() {}\n```";
        let html = render_markdown(md);
        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn rendering_strikethrough_produces_del_tag() {
        let html = render_markdown("~~strikethrough~~");
        assert!(html.contains("<del>"));
    }
}
