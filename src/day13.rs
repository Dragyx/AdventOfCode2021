use std::fmt::Display;

use crate::helper::{out, load_input_for_day};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Dot {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct TransparentPaper {
    dots: Vec<Dot>,
    width: usize,
    height: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FoldAxis {
    X,
    Y
}

impl TransparentPaper {
    pub fn new() -> Self {
        Self {
            dots: Vec::new(),
            width: 0,
            height: 0,
        }
    }
    pub fn add_dot(&mut self, x: i32, y: i32) {
        if x >= self.width as i32{
            self.width = x as usize + 1;
        }
        if y >= self.height as i32{
            self.height = y as usize + 1;
        }
        self.dots.push(Dot {x, y})
    }
    // pub fn parse_dot_string(&mut self)
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    pub fn get_num_dots(&self) -> usize {
        self.dots.len()
    }
    pub fn fold(&mut self, fold_pos: u32, axis: FoldAxis) {
        let mut offset = 0;
        let mut folded_dots : Vec<(usize, Dot)> = vec![];
        for (i, dot) in self.dots.iter_mut().enumerate() {
            let coord = match axis {
                FoldAxis::X => &mut dot.x,
                FoldAxis::Y => &mut dot.y,
            };
            // check if it is beyond the fold
            let dist_beyond_fold = *coord - fold_pos as i32;
            if dist_beyond_fold > 0 {
                // and fold it over
                *coord = fold_pos as i32 - dist_beyond_fold;
                // keep track of the maximum negative coordinate
                // to later offset all coordinates
                if *coord < offset {
                    offset = *coord;
                }
                folded_dots.push((i, *dot));
            }
        }
        // in addition, duplicate points are removed
        let mut i: usize = 0;
        self.dots.retain(| dot | {
            let retain = !folded_dots.iter()
                .any(
                    | (index, folded_dot) | *index != i && dot == folded_dot
                );
            i += 1;
            retain
        });
        // update the paper size
        let coord_to_update = match axis {
            FoldAxis::X => &mut self.width,
            FoldAxis::Y => &mut self.height,
        };
        // this can be quite tricky as the fold can be chosen in
        // way which creates points that are folded beyond the paper
        // 
        // the fold line itself is not included in the output
        *coord_to_update = (fold_pos + (-offset) as u32) as usize;
        // now the dot positions need to be updated using the offset
        // to ensure no dot has a negative connection
        for dot in &mut self.dots {
            let coord_to_update = match axis {
                FoldAxis::X => &mut dot.x,
                FoldAxis::Y => &mut dot.y,
            };
            *coord_to_update -= offset;
        }
    }
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut document = vec![vec!['.'; self.width as usize]; self.height as usize];
        for Dot {x, y} in &self.dots {
            document[*y as usize][*x as usize] = '#';   
        }
        for line in document {
            writeln!(f, "{}", line.into_iter().collect::<String>())?;
        }
        Ok(())
    }
}



pub fn run() {
    let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let input = load_input_for_day(13);
    let mut line_iter = input.lines();
    let dots_str = line_iter
        .by_ref()
        .take_while(| line | !line.starts_with('\n') && !line.is_empty());

    let mut paper = TransparentPaper::new();
    for dot in dots_str {
        let (x, y) = dot.split_once(',').unwrap();
        let (x, y): (i32, i32) = (x.parse().unwrap(), y.parse().unwrap());
        paper.add_dot(x, y);
    }
    let mut first_fold_count = 0;
    for (i, instruction) in line_iter.enumerate() {
        let instruction = &instruction[11..];
        let (axis_str, fold_pos) = instruction.split_once('=').unwrap();
        let axis = match axis_str {
            "x" => FoldAxis::X,
            "y" => FoldAxis::Y,
            _ => panic!()
        };
        let fold_pos = fold_pos.parse::<u32>().unwrap();
        paper.fold(fold_pos, axis);
        if i == 0 {
            first_fold_count = paper.get_num_dots();
        }
        
    }
    out(1)
        .var("number of dots", first_fold_count)
        .var("ASCII-Art output code", "")
        .print();
    print!("{}", paper);
}