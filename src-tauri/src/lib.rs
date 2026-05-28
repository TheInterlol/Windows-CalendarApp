use chrono::format::Numeric::Month;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

//include month.rs a days.rs
mod days;
mod months;

#[tauri::command]
fn spocitej_dny_v_mesici(mesic_cislo: u32, rok: i32) -> u8 {
    let vybrany_mesic = match mesic_cislo {
        1 => months::TypesOfMonth::January,
        2 => months::TypesOfMonth::February,
        3 => months::TypesOfMonth::March,
        4 => months::TypesOfMonth::April,
        5 => months::TypesOfMonth::May,
        6 => months::TypesOfMonth::June,
        7 => months::TypesOfMonth::July,
        8 => months::TypesOfMonth::August,
        9 => months::TypesOfMonth::September,
        10 => months::TypesOfMonth::October,
        11 => months::TypesOfMonth::November,
        12 => months::TypesOfMonth::December,
        _ => months::TypesOfMonth::January,
    };

    vybrany_mesic.get_days_in_month(rok)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, spocitej_dny_v_mesici])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
