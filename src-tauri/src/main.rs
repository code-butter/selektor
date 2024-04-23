// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, io};
use std::error::Error;
use std::io::Write;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use tauri::api::cli::Matches;

#[derive(Serialize,Deserialize,Clone)]
struct Option {
    label: String,
    value: String
}

#[derive(Serialize,Deserialize,Clone)]
struct AppConfig {
    options: Vec<Option>,
    prompt:  String
}

fn blank_default() -> Result<String, Box<dyn Error>> {
    Ok("".to_owned())
}

fn get_opt_string(matches: &Matches, name: &str, default: fn() -> Result<String, Box<dyn Error>>) -> Result<String, Box<dyn Error>> {
    Ok(if matches.args[name].occurrences > 0 {
        matches.args[name].value.to_string()
    } else {
        match env::var(format!("SELEKTOR_{}", name.to_uppercase())) {
            Ok(v) => v,
            Err(_) => default()?
        }
    })
}

fn get_options(matches: &Matches) -> Result<Vec<Option>, Box<dyn Error>> {
    let default = || {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        Ok(buffer)
    };
    let options_string = get_opt_string(&matches, "options", default)?;
    let options: Vec<Option> = serde_json::from_str(options_string.as_str())?;
    Ok(options)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let matches = app.get_cli_matches()?;
            let options = get_options(&matches)?;
            let prompt = get_opt_string(&matches, "prompt", blank_default)?;
            handle.manage(Mutex::new(AppConfig { options, prompt }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![stdout,get_config])
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
fn get_config(app_state: State<'_, Mutex<AppConfig>>) -> AppConfig {
    let state = app_state.lock()
        .expect("could not unlock app state");
    state.clone()
}
