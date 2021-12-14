pub mod day1;
pub mod day2;
pub mod day3;
mod helper;

const HEADER_WIDTH: usize = 70;

fn sep(day: usize) {
    let day = format!("( Day {} )", day);
    let remaining = HEADER_WIDTH - day.len();
    println!("");
    println!("{}", "-".repeat(HEADER_WIDTH));
    println!("{}{}{}", "<".repeat(remaining / 2), day, ">".repeat(remaining - remaining / 2));
    println!("{}", "-".repeat(HEADER_WIDTH));
}

pub fn run() {
    sep(1);
    day1::run();
    sep(2);
    day2::run();
    sep(3);
    day3::run();
    
}