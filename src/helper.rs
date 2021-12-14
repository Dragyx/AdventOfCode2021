use std::fs;

pub fn load_input_for_day(day: usize) -> String {
    // load input
    fs::read_to_string(format!("inputs/day{}.txt", day)).expect("input file missing!")
}