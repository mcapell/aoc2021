use std::env;

mod day_01;

fn main() {
    match env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(""))
        .as_str()
    {
        "01" | "1" => day_01::run(),
        _ => println!("Invalid day"),
    }
}
