use std::{
    collections::{hash_map, HashMap},
    error::Error,
};

use crate::helper::{load_input_for_day, out};

pub enum LineStatus {
    Corrupted { illegal_char: char },
    Complete,
    Incomplete { completion_sequence: Vec<char> },
}

pub fn parse_line(
    line: &str,
    symbol_map: &HashMap<char, char>,
    rev_symbol_map: &HashMap<char, char>,
) -> Result<LineStatus, String> {
    let mut chunk_stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match symbol_map.get(&c) {
            // is opening a chunk
            Some(_closing_character) => {
                chunk_stack.push(c);
            }
            // is closing a chunk
            None => {
                if let Some(&matching_opening_char) = rev_symbol_map.get(&c) {
                    // remove the closing_character
                    // if it the stack is of the format '{.....(}' something is obviously corrupted
                    if let Some(last_opening) = chunk_stack.pop() {
                        if last_opening != matching_opening_char {
                            return Ok(LineStatus::Corrupted { illegal_char: c });
                        }
                    }
                } else {
                    return Err(format!("Invalid Character found: '{}'", c).to_string());
                }
            }
        }
    }
    // because each finished chunk is removed as soon as it is closed,
    // the stack should now be empty
    match chunk_stack.len() {
        0 => Ok(LineStatus::Complete),
        // because the chunk stack is just a sequence of all the open, but not closed chunks we can just
        // reverse the stack to get the sequence that would be needed for completion
        _ => {
            let mut reverse_sequence = Vec::with_capacity(chunk_stack.len());
            for c in chunk_stack.into_iter().rev() {
                let inverse_char = *symbol_map
                    .get(&c)
                    .ok_or(format!("Invalid Character found: '{}'", c).to_string())?;
                reverse_sequence.push(inverse_char);
            }
            Ok(LineStatus::Incomplete {
                completion_sequence: reverse_sequence,
            })
        }
    }
}

pub fn run() {
    let _input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
        .to_string();
    let input = load_input_for_day(10);
    let symbol_map = HashMap::from([('{', '}'), ('[', ']'), ('(', ')'), ('<', '>')]);
    let rev_symbol_map: HashMap<char, char> =
        HashMap::from_iter(symbol_map.iter().map(|(k, v)| (*v, *k)));
    let point_map_corrupted: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let point_map_incomplete: HashMap<char, u32> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    fn incomplete_line_score(sequence: Vec<char>, pmap: &HashMap<char, u32>) -> u64 {
        let mut starting_score = 0;
        for c in sequence.into_iter() {
            starting_score *= 5;
            starting_score += *pmap.get(&c).unwrap() as u64;
        }
        starting_score
    }
    let mut score_corrupted: u64 = 0;
    let mut scores_incomplete: Vec<u64> = Vec::new();
    for line in input.lines() {
        match parse_line(line, &symbol_map, &rev_symbol_map).unwrap() {
            LineStatus::Corrupted { illegal_char } => {
                // dbg!(illegal_char);
                let char_score = point_map_corrupted.get(&illegal_char).unwrap();
                score_corrupted += *char_score as u64;
            }
            LineStatus::Complete => {}
            LineStatus::Incomplete {
                completion_sequence,
            } => {
                let line_score = incomplete_line_score(completion_sequence, &point_map_incomplete);
                // println!("{}: {}", line, line_score);
                scores_incomplete.push(line_score);
            }
        }
    }
    scores_incomplete.sort();
    let score_incomplete = scores_incomplete[scores_incomplete.len() / 2];
    out(1).var("corrupted score", score_corrupted).print();
    out(2).var("incomplete score", score_incomplete).print();
}
