use crate::helper::{load_input_for_day, out};

struct Field {
    pub value: usize,
    pub marked: bool,
}

impl Field {
    pub fn new(value: usize) -> Field {
        Field {
            value,
            marked: false,
        }
    }
}

struct Board {
    grid: Vec<Vec<Field>>,
    pub won: bool,
}

trait Bingo {
    fn number_drawn(&mut self, number: usize);
    fn check_win(&mut self) -> bool;
    fn sum_unmarked(&self) -> usize;
}

impl Board {
    /// generates a new bingo board from text input
    pub fn new(in_lines: &[&str]) -> Board {
        let mut grid = Vec::<Vec<Field>>::with_capacity(in_lines.len());
        for line in in_lines {
            let numbers: Vec<Field> = line
                .trim()
                .split_whitespace()
                .map(|num| Field::new(num.parse().unwrap()))
                .collect();
            grid.push(numbers)
        }

        Board { grid, won: false }
    }
}

impl Bingo for Board {
    fn number_drawn(&mut self, number: usize) {
        for row in self.grid.iter_mut() {
            for field in row.iter_mut() {
                if field.value == number {
                    field.marked = true
                }
            }
        }
    }

    fn check_win(&mut self) -> bool {
        // check rows
        let is_winner = self
            // go through rows
            .grid
            .iter()
            // check if all elements in the row are marked
            .any(|row| row.iter().all(|field| field.marked));
        if is_winner {
            self.won = true;
            return true;
        }
        let mut column_register = vec![true; self.grid[0].len()];
        // check columns
        for row in self.grid.iter() {
            for (i, field) in row.iter().enumerate() {
                if !field.marked {
                    column_register[i] = false;
                }
            }
        }
        self.won = column_register.iter().any(|column_win| *column_win);
        self.won
    }
    fn sum_unmarked(&self) -> usize {
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|field| !field.marked)
                    .map(|field| field.value)
                    .sum::<usize>()
            })
            .sum()
    }
}

pub fn run() {
    let input = load_input_for_day(4);
    // read the draws
    let mut lines = input.split('\n');
    let draws = lines.next().unwrap();
    let draws = draws.split(',').map(|s| s.parse::<usize>().unwrap());
    lines.next();
    // list of bingo boards
    let mut boards = Vec::<Board>::new();
    // read the bingo boards
    let mut bingo_lines: Vec<&str> = Vec::new();
    for line in lines {
        match line {
            "" => {
                boards.push(Board::new(&bingo_lines));
                bingo_lines = Vec::new();
            }
            numbers => bingo_lines.push(numbers),
        }
    }
    let mut board_count = 0;
    let num_boards = boards.len();
    for draw in draws {
        // check numbers
        for board in boards.iter_mut() {
            if board.won {
                continue;
            }
            board.number_drawn(draw);
            if board.check_win() {
                let unmarked_sum = board.sum_unmarked();
                if board_count == 0 || board_count == num_boards - 1 {
                    out(1)
                        .var("sum", unmarked_sum)
                        .var("current draw", draw)
                        .var("product", draw * unmarked_sum)
                        .print()
                }
                board_count += 1;
            }
        }
    }
}
