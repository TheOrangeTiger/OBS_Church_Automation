// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod lib; // for cli
use lib::cli; // for cli
fn main() {
    obs_church_automation_lib::run(); // For tauri UI
    cli(); // for cli
}
