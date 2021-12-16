use crate::helper::{load_input_for_day, out};

/// Before the conversion, the string is a file of the format
/// ```text
/// 101000001100
/// 011111100111
/// 111100001110
/// 110000011001
/// 001001001011
/// 010011101000
/// 011001110011
/// ```
/// Afterwards it is a vector, containing the lines
/// Each line stores a list of digits (as chars)
fn parse_input(input: String) -> Vec<Vec<char>> {
    let lines = input.split("\n");

    lines.map(|l| l.chars().collect()).collect()
}

const BASE: usize = 2;
/// Converts a slice with booleans to a usize
///
/// the least important bit is on the right
fn bitvec_to_usize(bitvec: &[bool]) -> usize {
    let mut output: usize = 0;
    for (i, bit) in bitvec.iter().rev().enumerate() {
        if *bit {
            output += BASE.pow(i as u32)
        }
    }
    output
}

fn charvec_to_bitvec(int_string: &Vec<char>) -> Vec<bool> {
    let mut output = Vec::with_capacity(int_string.len());
    for digit in int_string.iter() {
        // check the digit
        match digit {
            '0' => output.push(false),
            '1' => output.push(true),
            _ => panic!("Invalid input data! Contains characters that are not eather 1 or 0."),
        }
    }
    output
}

fn find_oxygen_generator(bitvec: &mut Vec<Vec<bool>>) {
    for column_pos in 0..bitvec[0].len() {
        // determine if 1 or 0 is more common in this column
        let column = bitvec.iter().filter(|row| row[column_pos]);
        let column_sum = column.count();
        let most_common = 2 * column_sum >= bitvec.len();
        // remove all elements that have the most commont element
        bitvec.retain(|row| row[column_pos] == most_common);
        // element has been found
        if bitvec.len() == 1 {
            break;
        }
    }
}
fn find_co2_scrubber(bitvec: &mut Vec<Vec<bool>>) {
    for column_pos in 0..bitvec[0].len() {
        // determine if 1 or 0 is more common in this column
        let column = bitvec.iter().filter(|row| row[column_pos]);
        let column_sum = column.count();
        let least_common = 2 * column_sum < bitvec.len();

        // remove all elements that have the most commont element
        bitvec.retain(|row| row[column_pos] == least_common);
        // element has been found
        if bitvec.len() == 1 {
            break;
        }
    }
}

pub fn run() {
    // load data
    let input = load_input_for_day(3);
    let data = parse_input(input);
    let half_height = data.len() / 2;
    // --------------------------- Task 1 ------------------------------
    // calculate the line width
    let width = data[0].len();
    let mut sums: Vec<usize> = vec![0; width];
    // got through each line and add to the sum
    for line in data.iter() {
        for (i, digit) in line.iter().enumerate() {
            // check the digit
            match digit {
                '0' => {}
                '1' => {
                    let sum = sums.get_mut(i).expect("Lines not of the same length!");
                    *sum += 1;
                }
                _ => panic!("Invalid input data! Contains characters that are not eather 1 or 0."),
            }
        }
    }
    // check if 1 is the most occuring number by checking if the sum is greater than half of the numbers
    let epsilon_vec: Vec<bool> = sums
        .iter()
        .map(|column_sum| *column_sum > half_height)
        .collect();
    let gamma_vec: Vec<bool> = epsilon_vec.iter().map(|bit| !bit).collect();
    // convert from to integer
    let gamma = bitvec_to_usize(&gamma_vec);
    let epsilon = bitvec_to_usize(&epsilon_vec);
    out(1)
        .var("gamma", gamma)
        .var("epsilon", epsilon)
        .var("product", gamma * epsilon)
        .print();
    // --------------------------- Task 2 ----------------------------
    // convert input data to bitvecs
    let bitvec_input: Vec<Vec<bool>> = data.iter().map(|row| charvec_to_bitvec(row)).collect();
    let mut oxygen_generator_data = bitvec_input.clone();
    let mut co2_scrubber_data = bitvec_input;
    find_oxygen_generator(&mut oxygen_generator_data);
    find_co2_scrubber(&mut co2_scrubber_data);
    let oxygen_generator_rating = bitvec_to_usize(&oxygen_generator_data[0]);
    let co2_scrubber_data = bitvec_to_usize(&co2_scrubber_data[0]);
    out(2)
        .var("Oxygen Generator Data", oxygen_generator_rating)
        .var("CO2 Scrubber Data", co2_scrubber_data)
        .var("Product", oxygen_generator_rating * co2_scrubber_data)
        .print();
}
