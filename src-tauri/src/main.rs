// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{Datelike, Local};

mod days;
mod months;

use days::TypesOfDay;
use months::Month;

/*pub fn write_out_current_day_and_month(day: u32, month: u32){
    println!("Dnes je {}. {}.",day, month);
}

pub fn get_current_date(){
    //pro aktualni čas
    let time = Local::now();
    let month_number = time.month();
    let day_number = time.weekday().number_from_monday();

    write_out_current_day_and_month(day_number, month_number);
}*/

fn main() {
    let feb = months::Month::February;

    let days_in_feb = feb.get_days_in_month(2024);

    println!("v únoru je: {} dnů", days_in_feb);

    windows_calendarapp_lib::run();
}
