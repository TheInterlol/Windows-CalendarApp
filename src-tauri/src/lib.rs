use std::fs;

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
fn load_event() -> String {
    match fs::read_to_string("events.json") {
        //Zkusí přečíst .json, jestli tam není tak pošle do Svelte prázdný objekt {}
        Ok(content) => content,
        Err(_) => "{}".to_string(),
    }
}

#[tauri::command]
fn save_event(data: String) -> Result<(), String> {
    // Vezme text z .json a zapíše
    fs::write("events.json", data).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, count_days, load_event, save_event
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
