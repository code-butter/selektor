// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::io::Write;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{Error, Manager, State};

#[derive(Serialize,Deserialize)]
struct Option {
    label: String,
    value: String
}

#[derive(Serialize,Deserialize)]
struct AppState {
    options: Vec<Option>
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let matches = app.get_cli_matches()?;
            let options_string = if matches.args.contains_key("options") {
                matches.args["options"].value.to_string()
            } else {
                let mut buffer = String::new();
                let stdin = io::stdin();
                stdin.read_line(&mut buffer)?;
                buffer
            };
            let options: Vec<Option> = serde_json::from_str(options_string.as_str())?;
            handle.manage(Mutex::new(AppState { options }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![stdout,get_options])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn stdout(str: String) {
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(str.as_bytes())
        .expect("unable to write to stdout");
}

#[tauri::command]
fn get_options(app_state: State<'_, Mutex<AppState>>) -> Result<Vec<Option>, Error> {
    let mut state = app_state.lock()?;
    Ok(state.options.clone())
}
