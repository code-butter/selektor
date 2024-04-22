// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, io};
use std::io::Write;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

#[derive(Serialize,Deserialize,Clone)]
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
            let options_string = if matches.args["options"].occurrences > 0 {
                matches.args["options"].value.to_string()
            } else {
                match env::var("SELEKTOR_OPTIONS") {
                    Ok(v) => v,
                    Err(_) => {
                        let mut buffer = String::new();
                        let stdin = io::stdin();
                        stdin.read_line(&mut buffer)?;
                        buffer
                    }
                }

            };
            // TODO: better error message here
            let options: Vec<Option> = serde_json::from_str(options_string.as_str())?;
            handle.manage(Mutex::new(AppState { options }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![stdout,get_options])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn stdout(value: String) {
    let mut output_value = value.clone();
    output_value.push_str("\n");
    let mut stdout = io::stdout().lock();
    stdout.write_all(output_value.as_bytes())
        .expect("unable to write to stdout");
}

#[tauri::command]
fn get_options(app_state: State<'_, Mutex<AppState>>) -> Vec<Option> {
    let state = app_state.lock()
        .expect("could not unlock app state");
    state.options.clone()
}
