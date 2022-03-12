use colored::Colorize;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
mod helper;

const HEADER_WIDTH: usize = 70;

fn sep(day: usize) {
    let day = format!("( Day {} )", day);
    let remaining = HEADER_WIDTH - day.len();
    println!("");
    println!("{}", "-".repeat(HEADER_WIDTH));
    println!(
        "{}{}{}",
        " ".repeat(remaining / 2),
        day.yellow().bold(),
        " ".repeat(remaining - remaining / 2)
    );
    println!("{}", "-".repeat(HEADER_WIDTH));
}

pub fn run() {
    sep(1);
    day1::run();
    sep(2);
    day2::run();
    sep(3);
    day3::run();
    sep(4);
    day4::run();
    sep(5);
    day5::run();
    sep(6);
    day6::run();
    sep(7);
    day7::run();
    sep(8);
    day8::run();
    sep(9);
    day9::run();
    sep(10);
    day10::run();
    sep(11);
    day11::run();
    sep(12);
    day12::run();
}
