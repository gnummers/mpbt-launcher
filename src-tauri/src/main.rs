// Tauri 2 entry point — delegates to lib.rs so the library crate
// can also be tested independently.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    mpbt_launcher_lib::run();
}
