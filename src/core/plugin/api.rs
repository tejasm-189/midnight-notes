use serde::{Deserialize, Serialize};

/// Functions exposed to WASM plugins for interacting with the app.
pub struct PluginApi;

impl Default for PluginApi {
    fn default() -> Self {
        Self
    }
}

impl PluginApi {
    pub fn new() -> Self {
        Self
    }
}

/// Input passed to a plugin's process function.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInput {
    pub content: String,
    pub title: String,
    pub tags: Vec<String>,
}

/// Output returned from a plugin's process function.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginOutput {
    pub content: String,
    pub additions: Vec<String>,
}

impl PluginInput {
    pub fn new(content: &str, title: &str, tags: &[String]) -> Self {
        Self {
            content: content.to_string(),
            title: title.to_string(),
            tags: tags.to_vec(),
        }
    }
}

/// Validate a plugin output before applying it.
pub fn validate_output(output: &PluginOutput) -> Result<(), String> {
    if output.content.is_empty() {
        return Err("plugin returned empty content".into());
    }
    if output.content.len() > 1_000_000 {
        return Err("plugin output exceeds 1MB limit".into());
    }
    Ok(())
}
