use std::f64::consts::{PI, FRAC_1_PI};

struct PathFinder<'a> {
    map: &'a Vec<Vec<u8>>,
    dist_map: Vec<Vec<Option<u32>>>,
    width: usize,
    height: usize,
    current_pos: (usize, usize),
    dest: (usize, usize),
    priority_queue: Vec<((usize, usize), u32)>
}

#[derive(Debug)]
enum PathFinderError {
    InvalidDestination,
    EmptyMap
}

impl<'a> PathFinder<'a> {
    pub fn new(map: &'a Vec<Vec<u8>>, dest: (usize, usize)) -> Result<Self, PathFinderError> {
        let first_row = map.get(0).ok_or(PathFinderError::EmptyMap)?;
        let width = first_row.len();
        let height = map.len();
        if !(dest.0 < width && dest.0 < height) {
            return Err(PathFinderError::InvalidDestination)
        }
        Ok(Self {
            map,
            current_pos: (0, 0),
            dest,
            width,
            height,
            dist_map: vec![vec![None; width]; height],
            priority_queue: Vec::new()
        })
    }
    // returns an iterator over adjacent values
    fn values_around_pos(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = ((usize, usize), &u8)> {
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
            if newx as usize >= self.width || newy as usize >= self.height {
                return None;
            }
            Some((newx as usize, newy as usize))
        });
        // unsafe needed / more convenient because I know that there will never be two (x, y)
        // that are the same and that (x, y) is in bounds. (look at how the iterator is generated
        // above). Without using unsafe it would be very hard to get mutable references to single elements
        positions.map(|(x, y)| unsafe {
            let column: &Vec<u8> = self.map.get_unchecked(x);
            ((x, y), column.get_unchecked(y))
        })
    }
    pub fn traverse_path(&mut self, start_pos: (usize, usize)) -> Result<Vec<(usize, usize)>, PathFinderError> {
        if start_pos.0 >= self.width || start_pos.1 >= self.height{
            return Err(PathFinderError::InvalidDestination)
        }
        while let Some((highest_priority_node, cost)) = self.priority_queue.pop() {
            let (x, y) = highest_priority_node;
            let distance = self.map[x][y];
            self.priority_queue.push(((self.width, self.height), cost + ))

        }
        todo!()
    }
}


impl<'a> Iterator for PathFinder<'a> {
    type Item = ((usize, usize), u32);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub fn run() {
    let input = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    let map: Vec<Vec<u8>> = input.lines().map(
        | line | line.trim().chars().map(
            | c | c.to_digit(10).unwrap() as u8
        ).collect()
    ).collect();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let path = PathFinder::new(&map, (width - 1, height - 1));
}

