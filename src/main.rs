use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {
    match env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(""))
        .as_str()
    {
        "01" | "1" => day_01::run(),
        "02" | "2" => day_02::run(),
        "03" | "3" => day_03::run(),
        "04" | "4" => day_04::run(),
        _ => println!("Invalid day"),
    }
}
