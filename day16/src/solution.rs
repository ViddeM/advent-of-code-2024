use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn move_in(&self, pos_x: usize, pos_y: usize) -> (usize, usize) {
        match self {
            Direction::North => (pos_x, pos_y - 1),
            Direction::East => (pos_x + 1, pos_y),
            Direction::South => (pos_x, pos_y + 1),
            Direction::West => (pos_x - 1, pos_y),
        }
    }

    fn move_from(&self, pos_x: usize, pos_y: usize) -> (usize, usize) {
        match self {
            Direction::North => (pos_x, pos_y + 1),
            Direction::East => (pos_x - 1, pos_y),
            Direction::South => (pos_x, pos_y - 1),
            Direction::West => (pos_x + 1, pos_y),
        }
    }

    fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

pub struct Map {
    layout: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn parse<'a>(input: &str) -> Map {
    let mut start = None;
    let mut end = None;

    let mut layout = vec![];
    for (y, row) in input.lines().enumerate() {
        let mut r = vec![];
        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    r.push(Tile::Wall);
                }
                '.' => {
                    r.push(Tile::Empty);
                }
                'S' => {
                    r.push(Tile::Empty);
                    start = Some((x, y));
                }
                'E' => {
                    r.push(Tile::Empty);
                    end = Some((x, y));
                }
                o => {
                    panic!("Unknown char {o}");
                }
            }
        }
        layout.push(r);
    }

    Map {
        layout,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn find_cheapest_path(input: Map) -> u64 {
    let Map { layout, start, end } = input;

    let mut tile_cost: HashMap<(usize, usize, Direction), u64> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start, Direction::East)));

    while let Some(Reverse((cost, (x, y), dir))) = queue.pop() {
        tile_cost.insert((x, y, dir.clone()), cost);

        if (x, y) == end {
            return cost;
        }

        let (nx, ny) = dir.move_in(x, y);
        if layout[ny][nx] == Tile::Empty {
            let prev_cost = tile_cost.get(&(nx, ny, dir.clone())).unwrap_or(&u64::MAX);
            let new_cost = cost + 1;
            if &new_cost < prev_cost {
                queue.push(Reverse((new_cost, (nx, ny), dir.clone())));
            }
        }

        let rot_dir = dir.rotate_clockwise();
        let (nx, ny) = rot_dir.move_in(x, y);
        if layout[ny][nx] == Tile::Empty {
            let prev_cost = tile_cost
                .get(&(nx, ny, rot_dir.clone()))
                .unwrap_or(&u64::MAX);
            let new_cost = cost + 1001;
            if &new_cost < prev_cost {
                queue.push(Reverse((new_cost, (nx, ny), rot_dir.clone())));
            }
        }

        let rot_dir = dir.rotate_counter_clockwise();
        let (nx, ny) = rot_dir.move_in(x, y);
        if layout[ny][nx] == Tile::Empty {
            let prev_cost = tile_cost
                .get(&(nx, ny, rot_dir.clone()))
                .unwrap_or(&u64::MAX);
            let new_cost = cost + 1001;
            if &new_cost < prev_cost {
                queue.push(Reverse((new_cost, (nx, ny), rot_dir.clone())));
            }
        }
    }
    panic!("No path found!");
}

pub fn solve_part_one<'a>(input: Map) -> String {
    find_cheapest_path(input).to_string()
}

fn find_tiles(input: Map) -> u64 {
    let Map { layout, start, end } = input;

    let mut tile_cost: HashMap<(usize, usize, Direction), u64> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start, Direction::East)));

    let mut best_path_cost = u64::MAX;

    while let Some(Reverse((cost, (x, y), dir))) = queue.pop() {
        tile_cost.insert((x, y, dir.clone()), cost);

        if cost > best_path_cost {
            continue;
        }

        if (x, y) == end {
            best_path_cost = best_path_cost.min(cost);
        }

        let (nx, ny) = dir.move_in(x, y);
        if layout[ny][nx] == Tile::Empty {
            let prev_cost = tile_cost.get(&(nx, ny, dir.clone())).unwrap_or(&u64::MAX);
            let new_cost = cost + 1;
            if &new_cost < prev_cost {
                queue.push(Reverse((new_cost, (nx, ny), dir.clone())));
            }
        }

        let rot_dir = dir.rotate_clockwise();
        let (nx, ny) = rot_dir.move_in(x, y);
        if layout[ny][nx] == Tile::Empty {
            let prev_cost = tile_cost
                .get(&(nx, ny, rot_dir.clone()))
                .unwrap_or(&u64::MAX);
            let new_cost = cost + 1000;
            if &new_cost < prev_cost {
                queue.push(Reverse((new_cost, (x, y), rot_dir.clone())));
            }
        }

        let rot_dir = dir.rotate_counter_clockwise();
        let (nx, ny) = rot_dir.move_in(x, y);
        if layout[ny][nx] == Tile::Empty {
            let prev_cost = tile_cost
                .get(&(nx, ny, rot_dir.clone()))
                .unwrap_or(&u64::MAX);
            let new_cost = cost + 1000;
            if &new_cost < prev_cost {
                queue.push(Reverse((new_cost, (x, y), rot_dir.clone())));
            }
        }
    }

    let mut to_check = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
    .filter_map(|d| tile_cost.get(&(end.0, end.1, d.clone())).map(|c| (d, *c)))
    .filter(|(_, c)| c == &best_path_cost)
    .map(|(d, _)| ((best_path_cost, end, d)))
    .collect::<Vec<_>>();

    let mut visited = HashSet::new();

    while let Some((cost, pos, dir)) = to_check.pop() {
        visited.insert(pos);

        if cost == 0 {
            continue;
        }

        let prev = dir.move_from(pos.0, pos.1);
        if let Some(&prev_cost) = tile_cost.get(&(prev.0, prev.1, dir.clone())) {
            if prev_cost == cost - 1 {
                to_check.push((prev_cost, prev, dir.clone()));
            }
        }

        if cost >= 1000 {
            let rot_dir = dir.rotate_clockwise();
            let prev = rot_dir.move_from(pos.0, pos.1);

            if let Some(&prev_cost) = tile_cost.get(&(prev.0, prev.1, rot_dir.clone())) {
                if prev_cost == cost - 1001 {
                    to_check.push((prev_cost, prev, rot_dir.clone()));
                }
            }

            let rot_dir = dir.rotate_counter_clockwise();
            let prev = rot_dir.move_from(pos.0, pos.1);

            if let Some(&prev_cost) = tile_cost.get(&(prev.0, prev.1, rot_dir.clone())) {
                if prev_cost == cost - 1001 {
                    to_check.push((prev_cost, prev, rot_dir.clone()));
                }
            }
        }
    }

    // // Print map
    // for (y, row) in layout.iter().enumerate() {
    //     for (x, t) in row.iter().enumerate() {
    //         match t {
    //             Tile::Wall => print!("#"),
    //             Tile::Empty => {
    //                 if visited.contains(&(x, y)) {
    //                     print!("O");
    //                 } else {
    //                     print!(".");
    //                 }
    //             }
    //         }
    //     }
    //     print!("\n");
    // }

    visited.len() as u64
}

pub fn solve_part_two<'a>(input: Map) -> String {
    find_tiles(input).to_string()
}
