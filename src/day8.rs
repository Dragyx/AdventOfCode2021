use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Map;
use std::ops::Index;

use regex::Regex;

use crate::helper::load_input_for_day;
use crate::helper::out;

#[derive(Clone, Debug)]
struct Digit {
    pub segments: HashSet<char>,
    pub unique_len: bool,
    pub i: u8,
}

impl Digit {
    pub fn new(i: u8, s: &str, ul: bool) -> Digit {
        Digit {
            i,
            segments: s.chars().collect(),
            unique_len: ul,
        }
    }
}

struct Mapping {
    pub mappings: Vec<(HashSet<char>, Digit)>,
}

impl Mapping {
    pub fn new() -> Mapping {
        Mapping {
            mappings: Vec::new(),
        }
    }
    pub fn insert(&mut self, chars: HashSet<char>, dig: Digit) {
        match self.mappings.iter_mut().find(|(c, d)| *c == chars) {
            Some(entry) => entry.1 = dig,
            None => self.mappings.push((chars, dig)),
        }
    }
    pub fn get(&self, index: HashSet<char>) -> Option<&Digit> {
        match self.mappings.iter().find(|(chars, d)| *chars == index) {
            Some((_c, digit)) => Some(digit),
            None => None,
        }
    }
}

impl Index<HashSet<char>> for Mapping {
    type Output = Digit;

    fn index(&self, index: HashSet<char>) -> &Self::Output {
        &self
            .mappings
            .iter()
            .find(|(chars, d)| *chars == index)
            .unwrap()
            .1
    }
}

fn count_repetitions(digits: &Vec<Digit>) -> HashMap<char, u32> {
    let mut counts = HashMap::<char, u32>::new();
    for d in digits.iter() {
        for c in &d.segments {
            let counter = counts.entry(*c).or_insert(0);
            *counter += 1;
        }
    }
    counts
}

/// tries to compute a mapping, mapping each wrong segment input to the correct digit
///  this is not always possible and if it fails, the outputted HashMap will have missing
///  keys
fn calculate_mapping<'a>(
    true_digits: &'a [Digit; 10],
    unique_patterns: &Vec<HashSet<char>>,
) -> Mapping {
    let mut mapping = Mapping::new();
    let trans = unique_patterns
        .iter()
        .map(|set| Digit::new(0, &set.iter().collect::<String>(), false))
        .collect::<Vec<Digit>>();
    let mut character_map = HashMap::<char, char>::new();
    // calculate 1, 4, 7, 8 from the pattern length
    let mut remaining_patterns = unique_patterns.clone();
    remaining_patterns.retain(|pattern| {
        for d in true_digits.iter() {
            if d.unique_len && pattern.len() == d.segments.len() {
                mapping.insert(pattern.clone(), d.clone());
                return false;
            }
        }
        true
    });
    // println!("Number of remaining:  {} ", remaining_patterns.len());
    let repetition_map = count_repetitions(&trans);
    let mut digit4data: Option<&(HashSet<char>, Digit)> = None;
    let mut digit1data: Option<&(HashSet<char>, Digit)> = None;
    // we know how often each segment appears in the correct configuration:
    // a: 8, b: 6, c: 8, d: 7, e: 4, f: 9, g: 7
    //
    // e, b and f are unique
    repetition_map.iter().for_each(|(c, r)| match *r {
        4 => {
            character_map.insert(*c, 'e');
        }
        6 => {
            character_map.insert(*c, 'b');
        }
        7 => {
            // two possibilities: g or d
            // as d is present in 4 (which we already computed) we can determine the right mapping
            match &mut digit4data {
                Some(_) => {}
                None => digit4data = mapping.mappings.iter().find(|(_set, d)| d.i == 4),
            }
            let (charset, _digit4) = digit4data.unwrap();
            match charset.contains(c) {
                true => {
                    character_map.insert(*c, 'd');
                }
                false => {
                    character_map.insert(*c, 'g');
                }
            }
        }
        8 => {
            // two possibilities: c or a
            // as c is present in 1 (which we already computed) we can determine the right mapping
            match &mut digit1data {
                Some(_) => {}
                None => digit1data = mapping.mappings.iter().find(|(_set, d)| d.i == 1),
            }
            let (charset, _digit1) = digit1data.unwrap();
            match charset.contains(c) {
                true => {
                    character_map.insert(*c, 'c');
                }
                false => {
                    character_map.insert(*c, 'a');
                }
            }
        }
        9 => {
            character_map.insert(*c, 'f');
        }
        _ => {}
    });
    // translate the remaining patterns using the charactermap
    remaining_patterns.iter().for_each(|pattern| {
        // println!("Remaining : {}", pattern.iter().collect::<String>());
        let charset = pattern
            .iter()
            .map(|segment| character_map[segment])
            .collect::<HashSet<char>>();
        let digit = true_digits.iter().find(|d| d.segments == charset).unwrap();
        mapping.insert(pattern.clone(), digit.clone());
    });
    mapping
}

pub fn run() {
    // The true segment to digit mapping
    let DIGITS: [Digit; 10] = [
        Digit::new(0, "abcefg", false),
        Digit::new(1, "cf", true),
        Digit::new(2, "acdeg", false),
        Digit::new(3, "acdfg", false),
        Digit::new(4, "bcdf", true),
        Digit::new(5, "abdfg", false),
        Digit::new(6, "abdefg", false),
        Digit::new(7, "acf", true),
        Digit::new(8, "abcdefg", true),
        Digit::new(9, "abcdfg", false),
    ];

    // let input =
    //    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    // println!("{}", input);
    let input = load_input_for_day(8);
    let digit_pattern = r"([a-g]+) ?";
    let extraction_pattern = format!(
        r"{}\| {}",
        digit_pattern.repeat(10),
        digit_pattern.repeat(4)
    );
    let extraction_re = Regex::new(&extraction_pattern).unwrap();
    let mut count1478 = 0;
    let mut count = 0;
    // extract information from line
    for line in input.split("\n") {
        let captures = extraction_re.captures(line).expect("Invalid input line.");
        // reminder: first capture is always the whole match
        let capture_strings: Vec<&str> = captures
            .iter()
            .map(|m| m.unwrap().as_str())
            .skip(1)
            .collect();
        let unique_patterns = &capture_strings[0..10]
            .iter()
            .map(|s| s.chars().collect())
            .collect();
        let digits = &capture_strings[10..14];
        // calculate mapping
        let mapping = calculate_mapping(&DIGITS, &unique_patterns);
        let number_of_1478 = digits
            .iter()
            .map(|d| mapping.get(d.chars().collect()))
            .filter(|d| d.is_some() && [1, 4, 7, 8].contains(&d.unwrap().i))
            .count();
        let mut number = String::new();
        // println!("{}", line);
        for d in digits {
            // println!("{}", d.chars().collect::<String>());
            let result = mapping.get(d.chars().collect()).unwrap().i.to_string();
            number.push_str(&result)
        }

        let number = number.parse::<u32>().unwrap();
        count1478 += number_of_1478;
        count += number;
    }
    out(1)
        .var("number of times 1, 4, 7 or 8 appeared", count1478)
        .print();
    out(2).var("all output values summed up", count).print();
}
