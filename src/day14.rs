use std::collections::HashMap;

use crate::helper::{load_input_for_day, out};

#[derive(Debug)]
struct Polymer {
    rules: HashMap<[char; 2], ([char; 2], [char; 2])>,
    polymer: HashMap<[char; 2], u64>,
    rightmost_element: Option<char>,
}

impl Polymer {
    pub fn from_input<'a>(mut lines: impl Iterator<Item = &'a str>) -> Self {
        let mut rules = HashMap::new();
        let polymer_line = lines.next().unwrap().as_bytes();
        let mut polymer = HashMap::new();
        for window in polymer_line.windows(2) {
            let window = [window[0] as char, window[1] as char];
            *polymer.entry(window).or_insert(0) += 1;
        }
        lines.next();
        for rule in lines {
            let (pair, insert) = rule.trim().split_once(" -> ").unwrap();
            let pair = pair.as_bytes();
            let [e1, e2] = [pair[0] as char, pair[1] as char];
            let insert = insert.as_bytes()[0] as char;
            rules.insert([e1, e2], ([e1, insert], [insert, e2]));
        }
        let rightmost = match polymer_line.len() >= 1 {
            true => Some(*polymer_line.last().unwrap() as char),
            false => None,
        };

        Self {
            rules,
            polymer,
            rightmost_element: rightmost,
        }
    }
    pub fn polymerization_step(&mut self) {
        let mut added_elements = HashMap::new();
        for (pair, (to_increase1, to_increase2)) in &self.rules {
            if let Some(count) = self.polymer.get(pair) {
                *added_elements.entry(*to_increase1).or_insert(0) += count;
                *added_elements.entry(*to_increase2).or_insert(0) += count;
            }
        }
        self.polymer = added_elements;
    }
    pub fn count_elements(&self) -> HashMap<char, u64> {
        let mut counts: HashMap<char, u64> = HashMap::new();
        for (&[element, _], count) in &self.polymer {
            *counts.entry(element).or_insert(0) += count;
        }
        if let Some(last_element) = self.rightmost_element {
            *counts.entry(last_element).or_insert(0) += 1;
        }
        counts
    }
}

pub fn run() {
    let _input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let input = load_input_for_day(14);
    let mut polymer = Polymer::from_input(input.lines());
    for _ in 0..10 {
        polymer.polymerization_step();
    }
    let counts: Vec<(char, u64)> = polymer.count_elements().into_iter().collect();
    let most_common_after10 = counts.iter().max_by_key(|(_c, count)| count).unwrap();
    let least_common_after10 = counts.iter().min_by_key(|(_c, count)| count).unwrap();
    for _ in 0..30 {
        polymer.polymerization_step();
    }
    let counts: Vec<(char, u64)> = polymer.count_elements().into_iter().collect();
    let most_common_after40 = counts.iter().max_by_key(|(_c, count)| count).unwrap();
    let least_common_after40 = counts.iter().min_by_key(|(_c, count)| count).unwrap();
    out(1)
        .var(
            "most common - least common after 10 steps",
            most_common_after10.1 - least_common_after10.1,
        )
        .print();
    out(3)
        .var(
            "most common - least common after 40 steps",
            most_common_after40.1 - least_common_after40.1,
        )
        .print();
}
