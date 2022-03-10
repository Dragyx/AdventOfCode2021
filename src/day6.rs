use std::{collections::VecDeque, ops::AddAssign};

use crate::helper::{load_input_for_day, out};

/// simulates a the flock of laternfish
///
/// To improve performance, a VecDeque is used where the
/// the index is the timer and the value is the number of fish
/// with that timer
fn simulate_fish<T>(fish: &mut VecDeque<T>, num_iter: usize)
where
    T: AddAssign + Copy,
{
    for _ in 0..num_iter {
        // spawn new fish if timer reached 0
        let reached_timer_reset = fish.pop_front().unwrap();
        // spawn new fish
        fish.push_back(reached_timer_reset);
        // reset old fish timers
        let timer6 = fish.get_mut(6).expect("Timer state 6 does not exist!");
        *timer6 += reached_timer_reset;
    }
}

/// Converts a list of strings representing fish timers
/// to a VecDeque
fn instring_to_deque(fishin: &str, timer_states: usize) -> VecDeque<u32> {
    let fish_list_seperate: Vec<u32> = fishin
        .split(",")
        .map(|timer| timer.parse().expect("Input timer is not an integer!"))
        .collect();
    let mut fish_list_fused: VecDeque<u32> = vec![0; timer_states].into();
    for value in fish_list_seperate {
        fish_list_fused[value as usize] += 1;
    }
    fish_list_fused
}

pub fn run() {
    let input = "3,4,3,1,2";
    let input = load_input_for_day(6);
    let mut fused_fish = instring_to_deque(&input, 9);
    let mut fused_fish_task2: VecDeque<u64> = fused_fish
        .clone()
        .iter()
        .map(|num_fish| *num_fish as u64)
        .collect();
    simulate_fish(&mut fused_fish, 80);
    let sum: u32 = fused_fish.iter().sum();
    out(1).var("sum of fish after 80 days", sum).print();
    simulate_fish(&mut fused_fish_task2, 256);

    let mut sum_task_2: u64 = 0;
    fused_fish_task2
        .iter()
        .for_each(|num_fish| sum_task_2 += *num_fish as u64);
    out(2).var("sum of fish after 256 days", sum_task_2).print();
}
