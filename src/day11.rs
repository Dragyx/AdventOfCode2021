use std::{error::Error, fmt::Octal};

struct OctoField<const SX: usize, const SY: usize> {
    field: [[u8; SY]; SX],
    counter: u64,
}

impl<const SX: usize, const SY: usize> OctoField<SX, SY> {
    pub fn from_str(input: &str) -> Result<OctoField<SX, SY>, Box<dyn Error>> {
        let mut field = [[0u8; SY]; SX];
        for (y, line) in input.lines().enumerate() {
            for (x, character) in line.chars().enumerate() {
                let field_value = character
                    .to_digit(10)
                    .ok_or("Invalid character in input")?
                    .try_into()?;
                field[x][y] = field_value;
            }
        }
        Ok(OctoField { field, counter: 0 })
    }
    pub fn perform_step(&mut self) {}
    fn values_around_pos(&mut self, x: usize, y: usize) -> impl Iterator<&mut u8> {
        let deltas = (-1isize..=1).flat_map(move |x| (-1isize..=1).map(|y| (x, y)));
        deltas.filter_map(| (dx, dy) | {
            let (newx, newy) = (x as isize + dx, y as isize + dy);
            // lower_bounds
            if newx < 0 || newy > 0 {
                return None
            }
            let (newx, newy) = (x as isize + dx, y as isize + dy);
            if  
            self.field.get_mut(newx as usize)
        })
    }
}

pub fn run() {}
