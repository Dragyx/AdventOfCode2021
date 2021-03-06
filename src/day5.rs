use std::collections::HashMap;

use crate::helper::{load_input_for_day, out};
use regex;

#[derive(PartialEq, Debug)]
enum Orient {
    Vertical,
    Horizontal,
    Diagonal,
}
#[derive(Debug)]
struct Line {
    pub points: Vec<[u32; 2]>,
    pub orient: Orient,
}
impl Line {
    pub fn new(p1: [u32; 2], p2: [u32; 2]) -> Line {
        // Lines are only vertical or horizontal at the moment
        let orient = if p1[0] == p2[0] {
            Orient::Vertical
        } else if p2[1] == p1[1] {
            Orient::Horizontal
        } else {
            Orient::Diagonal
        };
        let mut points;
        // make sure the first value is smaller than the second one (for
        // the ranges)
        let mut x_range = [p1[0], p2[0]];
        x_range.sort_unstable();
        let mut y_range = [p1[1], p2[1]];
        y_range.sort_unstable();
        match orient {
            Orient::Horizontal | Orient::Vertical => {
                // interpolate the points
                points = (x_range[0]..=x_range[1]).flat_map(|x| (y_range[0]..=y_range[1]).map(move |y| [x, y]))
                    .collect();
            }
            Orient::Diagonal => {
                points = Vec::new();
                // has 45° angle

                // for each coordinate, determine if
                // it has to be incremented or decremented
                // to reach the end position.
                let mut position = p1;
                points.push(position);
                let xstep_positive = p1[0] == x_range[0];
                let ystep_positive = p1[1] == y_range[0];
                loop {
                    // advance
                    match xstep_positive {
                        true => position[0] += 1,
                        false => position[0] -= 1,
                    }
                    match ystep_positive {
                        true => position[1] += 1,
                        false => position[1] -= 1,
                    }
                    points.push(position);
                    if position == p2 {
                        break;
                    }
                }
            }
        }

        Line { points, orient }
    }
}

#[derive(Debug)]
struct CoordinateSystem {
    lines: Vec<Line>,
}
impl CoordinateSystem {
    pub fn from_string(lines: String) -> CoordinateSystem {
        let line_regex = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        let points = lines.split('\n').map(|line| {
            // extract all coordinates
            let re_match = line_regex.captures(line).expect("Invalid line in input!");
            let mut re_match = re_match.iter();
            re_match.next().unwrap();
            let captures: Vec<u32> = re_match
                .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
                .collect();
            Line::new([captures[0], captures[1]], [captures[2], captures[3]])
        });
        CoordinateSystem {
            lines: points.collect(),
        }
    }
    /// returns a HashMap of all the intersections with the key
    /// being the coordinate and the value being the number of
    /// lines on this position
    pub fn find_intersections(&self, ignore_diagonals: bool) -> HashMap<[u32; 2], u32> {
        let mut intersections = HashMap::<[u32; 2], u32>::new();
        for l in self.lines.iter() {
            if ignore_diagonals && l.orient == Orient::Diagonal {
                continue;
            }
            for p in l.points.iter() {
                *intersections.entry(*p).or_insert(0) += 1;
            }
        }
        intersections
    }
}

pub fn run() {
    let input = load_input_for_day(5);
    let coordinate_system = CoordinateSystem::from_string(input);
    // println!("{:?}", coordinate_system);
    let intersections_no_diagonals = coordinate_system.find_intersections(true);
    // println!("{:?}", intersections);
    let count1 = intersections_no_diagonals
        .values()
        .filter(|intersection_count| **intersection_count > 1)
        .count();
    let intersections = coordinate_system.find_intersections(false);
    // println!("{:?}", intersections);
    let count2 = intersections
        .values()
        .filter(|intersection_count| **intersection_count > 1)
        .count();
    out(1)
        .var("line overlaps (2 or more, without diagonals)", count1)
        .print();
    out(2).var("line overlaps (2 or more)", count2).print();
}
