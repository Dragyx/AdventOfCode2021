use crate::helper::{load_input_for_day, out};

/// just a simple struct to hold the data of a possible crab
/// position
struct Target {
    pub pos: u32,
    pub cost: u32,
}

pub fn run() {
    // let input = "16,1,2,0,4,2,7,1,2,14";
    let input = load_input_for_day(7);
    let input: Vec<u32> = input
        .split(",")
        .map(|crab_pos| crab_pos.parse().expect("Input number is not a number"))
        .collect();
    let max = input.iter().max().unwrap();
    let mut min_cost_t1: Option<Target> = None;
    let mut min_cost_t2: Option<Target> = None;
    // for each possible position, calculate the cost and
    // see if it is smaller than the cost of the last value
    for v in 0..=*max {
        let mut cost_t1: u32 = 0;
        let mut cost_t2: u32 = 0;
        for value in input.iter() {
            // added cost for task 1 is just the distance
            let added_cost = (v as i32 - *value as i32).abs() as u32;
            cost_t1 += added_cost;
            // for task 2 each step costs 1 fuel more
            // e.g. for distance 5, the cost would be 5 + 4 + 3 + 2 + 1 = 15
            // if you draw these numbers as a triangle, you can calculate its are
            // 5*5/2 = 12.5. Afterwards, you still need to add the remaining
            // half squares: + 5/2. This boils down to 5*6/2
            cost_t2 += added_cost * (added_cost + 1) / 2;
        }
        match &mut min_cost_t1 {
            Some(target) => {
                if target.cost > cost_t1 {
                    min_cost_t1 = Some(Target {
                        pos: v,
                        cost: cost_t1,
                    });
                }
            }
            None => {
                min_cost_t1 = Some(Target {
                    pos: v,
                    cost: cost_t1,
                })
            }
        }
        match &mut min_cost_t2 {
            Some(target) => {
                if target.cost > cost_t2 {
                    min_cost_t2 = Some(Target {
                        pos: v,
                        cost: cost_t2,
                    });
                }
            }
            None => {
                min_cost_t2 = Some(Target {
                    pos: v,
                    cost: cost_t2,
                })
            }
        }
    }
    if let Some(target) = min_cost_t1 {
        out(1)
            .var("target position", target.pos)
            .var("cost", target.cost)
            .print();
    } else {
        out(1).var("No target value could be found", ":(").print();
    }
    if let Some(target) = min_cost_t2 {
        out(2)
            .var("target position", target.pos)
            .var("cost", target.cost)
            .print();
    } else {
        out(1).var("No target value could be found", ":(").print();
    }
}
