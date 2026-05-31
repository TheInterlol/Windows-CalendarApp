// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{Datelike, Local};

mod days;
mod months;

use days::TypesOfDay;
use months::Month;

fn main() {
    let feb = months::Month::February;

    let days_in_feb = feb.get_days_in_month(2024);

    println!("v únoru je: {} dnů", days_in_feb);

    windows_calendarapp_lib::run();
}
