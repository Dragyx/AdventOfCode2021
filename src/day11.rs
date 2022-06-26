use std::{
    error::Error,
    fmt::Display,
};

use crate::helper::{load_input_for_day, out};

struct OctoField<const SX: usize, const SY: usize> {
    field: [[u8; SY]; SX],
    counter: u64,
    flash_count: u64,
}

impl<const SX: usize, const SY: usize> OctoField<SX, SY> {
    pub fn from_str(input: &str) -> Result<OctoField<SX, SY>, Box<dyn Error>> {
        let mut field = [[0u8; SY]; SX];
        for (y, line) in input.lines().enumerate() {
            for (x, character) in line.trim().chars().enumerate() {
                let field_value = character
                    .to_digit(10)
                    .ok_or("Invalid character in input")?
                    .try_into()?;
                *field
                    .get_mut(x)
                    .ok_or("Invalid Input size")?
                    .get_mut(y)
                    .ok_or("Invalid Input size")? = field_value;
            }
        }
        Ok(OctoField {
            field,
            counter: 0,
            flash_count: 0,
        })
    }
    pub fn perform_step(&mut self) -> bool {
        // You can model the energy levels and flashes of light in steps. During a single step, the following occurs:
        //
        //    First, the energy level of each octopus increases by 1.
        //    Then, any octopus with an energy level greater than 9 flashes. This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent. If this causes an octopus to have an energy level greater than 9, it also flashes. This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
        //    Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
        //
        // Adjacent flashes can cause an octopus to flash on a step even if it begins that step with very little energy. Consider the middle octopus with 1 energy in this situation:

        let mut positions_to_flash: Vec<(usize, usize)> = Vec::new();
        let mut has_flashed = [[false; SY]; SX];
        self.counter += 1;
        // detect initial flashes
        #[allow(clippy::needless_range_loop)]
        for x in 0..SX {
            for y in 0..SY {
                // we know that this is in bounds because we literally use the array size
                // to create the iterator over x and y
                let octopus = &mut self.field[x][y];
                *octopus += 1;
                if *octopus > 9 {
                    has_flashed[x][y] = true;
                    positions_to_flash.push((x, y));
                }
            }
        }
        // continue flashing until the flashing stops
        while let Some((x, y)) = positions_to_flash.pop() {
            for ((adj_x, adj_y), adj_value) in self.values_around_pos(x, y) {
                if !has_flashed[adj_x][adj_y] {
                    *adj_value += 1;
                    if *adj_value > 9 {
                        has_flashed[adj_x][adj_y] = true;
                        positions_to_flash.push((adj_x, adj_y));
                    }
                }
            }
        }
        let mut flash_count_this_step = 0;
        #[allow(clippy::needless_range_loop)]
        for x in 0..SX {
            for y in 0..SY {
                if has_flashed[x][y] {
                    flash_count_this_step += 1;
                    self.field[x][y] = 0;
                }
            }
        }
        self.flash_count += flash_count_this_step;
        // did all the values flash?
        flash_count_this_step >= (SX * SY).try_into().unwrap()
    }

    // returns an iterator over adjacent values
    fn values_around_pos(
        &mut self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = ((usize, usize), &mut u8)> {
        // Vec<&mut u8> {
        // let mut refs = Vec::new();
        let deltas = (-1isize..=1)
            .flat_map(move |x| (-1isize..=1).map(move |y| (x, y)))
            .filter(|(x, y)| !(*x == 0 && x == y));
        let positions = deltas.filter_map(move |(dx, dy)| {
            let (newx, newy) = (x as isize + dx, y as isize + dy);
            // lower_bounds
            if newx < 0 || newy < 0 {
                return None;
            }
            // upper bounds
            if newx as usize >= SX || newy as usize >= SY {
                return None;
            }
            Some((newx as usize, newy as usize))
        });
        // unsafe needed / more convenient because I know that there will never be two (x, y)
        // that are the same and that (x, y) is in bounds. (look at how the iterator is generated
        // above). Without using unsafe it would be very hard to get mutable references to single elements
        positions.map(|(x, y)| unsafe {
            let column: &mut [u8; SY] = &mut *(self.field.get_unchecked_mut(x) as *mut _);
            ((x, y), column.get_unchecked_mut(y))
        })
    }
}

impl<const SX: usize, const SY: usize> Display for OctoField<SX, SY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "-".repeat(SX))?;
        for y in 0..SY {
            for x in 0..SX {
                let value = unsafe { self.field.get_unchecked(x).get_unchecked(y) };
                write!(f, "{}", value)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", "-".repeat(SX))?;
        writeln!(f, "STEPS: {}", self.counter)?;
        writeln!(f, "FLASHES: {}", self.flash_count)?;
        writeln!(f, "{}", "-".repeat(SX))?;
        Ok(())
    }
}

pub fn run() {
    /*
    let input = "11111
    19991
    19191
    19991
    11111";

    let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526";
    */
    let input = &load_input_for_day(11);
    let mut octofield = OctoField::<10, 10>::from_str(input).unwrap();
    // println!("{}", octofield);
    for _ in 0..100 {
        octofield.perform_step();
        // println!("{}", octofield);
    }
    out(1).var("flashes", octofield.flash_count).print();
    let mut octofield = OctoField::<10, 10>::from_str(input).unwrap();
    let mut first_time_full_flash: Option<u64> = None;
    loop {
        if octofield.perform_step() && first_time_full_flash.is_none() {
            first_time_full_flash = Some(octofield.counter);
            break;
        }
    }
    out(2)
        .var("first full flash", first_time_full_flash)
        .print();
}
