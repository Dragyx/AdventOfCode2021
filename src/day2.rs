use crate::helper::{load_input_for_day, out};

pub fn run() {
    // load input
    let input = load_input_for_day(2);
    // convert to better format
    let instructions: Vec<(&str, usize)> = input
        .split('\n')
        .map(|instruction| {
            // extract commands
            let mut instruction = instruction.split(' ');
            let command = instruction.next().expect("Empty instruction!");
            let amount_str = instruction.next().expect("Missing amount to move!");
            // convert the amount to a number
            let amount = amount_str
                .parse::<usize>()
                .expect("Some inputs are not numbers!");
            (command, amount)
        })
        .collect();
    // ---------------------------------- Task 1 ----------------------------------
    // starting position
    let mut position = [0, 0];

    // manipulate position
    for (command, amount) in instructions.iter() {
        match *command {
            "forward" => position[0] += amount,
            "down" => position[1] += amount,
            "up" => position[1] -= amount,
            _ => panic!("Invalid command!"),
        }
    }
    out(1)
        .var("end position", position)
        .var("product", position[0] * position[0])
        .print();
    // ---------------------------------- Task 2 ----------------------------------
    // starting position [aim, horizontal_position, depth]
    let mut depth = 0;
    let mut aim = 0;
    let mut hpos = 0;

    // manipulate position
    //  From the puzzle description:
    // - down X increases your aim by X units.
    // - up X decreases your aim by X units.
    // - forward X does two things:
    //    - It increases your horizontal position by X units.
    //    - It increases your depth by your aim multiplied by X.

    for (command, amount) in instructions.iter() {
        match *command {
            "forward" => {
                // increase horizontal position
                hpos += amount;
                // increase depth
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Invalid command!"),
        }
    }

    out(2)
        .var("aim", aim)
        .var("hpos", hpos)
        .var("depth", depth)
        .var("product", depth * hpos)
        .print();
}
