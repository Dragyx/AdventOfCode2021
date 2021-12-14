use std::fs;

fn main() {
    // load input
    let input = fs::read_to_string("input.txt").expect("input file missing!");
    // convert to numbers
    let measurements: Vec<usize> = input
        .split("\n")
        .map(|depth_str| {
            depth_str
                .parse::<usize>()
                .expect("Some inputs are not numbers!")
        })
        .collect();

    // ---------------------------- Task 1 -------------------------------
    // calculate how many values are greater than the last one
    let count1: usize = measurements
        .windows(2)
        .flat_map(<&[usize; 2]>::try_from)
        .filter(|&&[a, b]| b > a)
        .count();
    println!(
        "Task 1: The number of depth measurements that are greater than the preceding one is: {}",
        count1
    );

    // ---------------------------- Task 2 -------------------------------
    // calculate the sum of each sliding window
    let sums: Vec<usize> = measurements
        .windows(3)
        // is necessary so that the compiler knows the slices are of size 3
        .flat_map(<&[usize; 3]>::try_from)
        .map(|[a, b, c]| a + b + c)
        .collect();
    // count the sliding windows that are greater than the last one
    let count2 = sums
        .windows(2)
        .flat_map(<&[usize; 2]>::try_from)
        // compare the sum of each sliding window to the sum of the last one
        .filter(|[a, b]| b > a)
        .count();

    println!(
        "Task 2: The number of depth measurements that are greater than the preceding one is: {}",
        count2
    );
}