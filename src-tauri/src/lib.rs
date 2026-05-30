use std::fs;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

//include month.rs a days.rs
mod days;
mod months;

#[tauri::command]
fn count_days(month_no: u32, year: i32) -> u8 {
    let chosen_month = match month_no {
        1 => months::Month::January,
        2 => months::Month::February,
        3 => months::Month::March,
        4 => months::Month::April,
        5 => months::Month::May,
        6 => months::Month::June,
        7 => months::Month::July,
        8 => months::Month::August,
        9 => months::Month::September,
        10 => months::Month::October,
        11 => months::Month::November,
        12 => months::Month::December,
        _ => months::Month::January,
    };

    chosen_month.get_days_in_month(year)
}
#[tauri::command]
fn load_settings(app: tauri::AppHandle) -> String {
    if let Ok(app_dir) = app.path().app_data_dir() {
        let path = app_dir.join("settings.json");
        if let Ok(content) = fs::read_to_string(path) {
            return content;
        }
    }
    "{}".to_string()
}
#[tauri::command]
fn save_settings(app: tauri::AppHandle, data: String) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let path = app_dir.join("settings.json");
    fs::write(path, data).map_err(|e| e.to_string())
}

#[tauri::command]
fn load_event(app: tauri::AppHandle) -> String {
    if let Ok(app_dir) = app.path().app_data_dir() {
        let path = app_dir.join("events.json");
        if let Ok(content) = fs::read_to_string(path) {
            return content;
        }
    }
    "{}".to_string()
}

#[tauri::command]
fn save_event(app: tauri::AppHandle, data: String) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let path = app_dir.join("events.json");
    fs::write(path, data).map_err(|e| e.to_string())
}
#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            count_days,
            load_settings,
            save_settings,
            load_event,
            save_event,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
