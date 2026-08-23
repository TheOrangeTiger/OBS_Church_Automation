#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod backend;
mod cli;
mod ui;
// use cli::cli;
use ui::ui;
fn main() {
    // cli();
    let _ = ui();
}
