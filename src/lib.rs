pub mod core;
pub mod crypto;
pub mod storage;
pub mod ui;

use tracing_subscriber::EnvFilter;

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Passing an empty Vec of contexts — no type annotation needed
    let config = dioxus_desktop::Config::new()
        .with_custom_index(include_str!("../styles/index.html").into());

    dioxus_desktop::launch::launch(ui::app::App, vec![], vec![Box::new(config)]);
}
