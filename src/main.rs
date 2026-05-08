//! Wormhole — a terminal-based SSH connection manager with TUI interface.

mod app;
mod config;
mod i18n;
mod ssh;
mod theme;
mod ui;

fn main() {
    let mut app = app::App::new();
    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
