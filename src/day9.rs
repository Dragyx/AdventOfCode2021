use crate::helper::{load_input_for_day, out};

// which adjacent points should be checked
const offsets: [[i64; 2]; 4] = [[0i64, -1], [1, 0], [0, 1], [-1, 0]];

pub struct HeightMap {
    inner_map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl HeightMap {
    pub fn from_string(string: String) -> HeightMap {
        let height_map = string
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect::<Vec<Vec<u8>>>();
        let (w, h) = (height_map[0].len(), height_map.len());
        HeightMap {
            inner_map: height_map,
            width: w,
            height: h,
        }
    }

    pub fn get_adjacent<'a>(
        &'a self,
        x: usize,
        y: usize,
    ) -> Result<impl Iterator<Item = ([usize; 2], &'a u8)>, &'static str> {
        let in_bounds = x < self.width && y < self.height;
        if !in_bounds {
            return Err("Point does not exist");
        }

        let adjacent = offsets.iter().filter_map(move |&[off_x, off_y]| {
            let (x_i64, y_i64) = (x as i64, y as i64);
            let adjacent_x = x_i64 + off_x;
            let adjacent_y = y_i64 + off_y;
            let in_bounds = adjacent_x >= 0
                && adjacent_x < self.width as i64
                && adjacent_y >= 0
                && adjacent_y < self.height as i64;
            if !in_bounds {
                return None;
            }
            let adjacent = &self.inner_map[adjacent_y as usize][adjacent_x as usize];
            Some(([adjacent_x as usize, adjacent_y as usize], adjacent))
        });
        Ok(adjacent)
    }
    pub fn find_low_points<'a>(&'a self) -> impl Iterator<Item = ([usize; 2], u8)> + 'a {
        (0..self.height).flat_map(move |y| {
            (0..self.width).filter_map(move |x| {
                let point = self.inner_map[y as usize][x as usize];
                let mut adjacent_points = self.get_adjacent(x, y).unwrap();
                let is_low_point = adjacent_points.all(|(_pos, &value)| value > point);

                match is_low_point {
                    true => Some(([x, y], point)),
                    false => None,
                }
            })
        })
    }
    fn basin_around_low_point(&self, x: usize, y: usize) -> Vec<([usize; 2], u8)> {
        // where has the algorithm already checked if the point is part of the basin?
        let value = self.inner_map[y][x];
        let mut points_checked = vec![vec![false; self.width]; self.height];
        let mut points_to_be_checked = vec![([x, y], value)];
        let mut basin = Vec::new();
        while points_to_be_checked.len() != 0 {
            // check all neighbours that haven't already been checked
            let (pos, value) = points_to_be_checked.pop().unwrap();
            let (x, y) = (pos[0], pos[1]);
            if points_checked[y][x] {
                continue;
            }
            for (adj_pos, adj_height) in self.get_adjacent(x, y).unwrap() {
                let (ax, ay) = (adj_pos[0], adj_pos[1]);
                // skip all we have already checked
                if points_checked[ay][ax] {
                    continue;
                }
                // println!("Adj of {:?} : {:?}", pos, adj_pos);
                match adj_height {
                    9 => {}
                    _ => {
                        points_to_be_checked.push((adj_pos, *adj_height));
                    }
                }
            }
            basin.push((pos, value));
            points_checked[y][x] = true;
            // println!("Checked point {} {}", x, y);
            // println!("{:?}", points_to_be_checked);
        }
        // println!("------------------- Basin for {} {}", x, y);
        // points_checked.iter().for_each(| line | {
        //     line.iter().for_each( | c | print!("{}", *c as u8));
        //     println!("");
        // });
        // println!("{:?}", basin);
        // println!("-------------------------------------");
        basin
    }
    fn find_basins<'a>(
        &'a self,
        low_points: impl Iterator<Item = ([usize; 2], u8)> + 'a,
    ) -> impl Iterator<Item = Vec<([usize; 2], u8)>> + 'a {
        low_points.map(|(p, _value)| self.basin_around_low_point(p[0], p[1]))
    }
}

pub fn run() {
    let input = load_input_for_day(9);
    let height_map = HeightMap::from_string(input);
    let low_points: Vec<([usize; 2], u8)> = height_map.find_low_points().collect();
    // Task 1

    let risk: u32 = low_points
        .iter()
        .map(|(_pos, value)| 1 + (*value as u32))
        .sum();
    out(1).var("risk", risk).print();
    // Task 2
    let mut basins: Vec<Vec<([usize; 2], u8)>> =
        height_map.find_basins(low_points.into_iter()).collect();
    basins.sort_by_key(|basin| basin.len());
    basins.reverse();
    let prod_of_three_largest: u64 = basins[0..3]
        .iter()
        .map(|basin| basin.len() as u64)
        .product();
    out(2)
        .var("product of three largest basins", prod_of_three_largest)
        .print();
}
