use std::collections::HashSet;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Obstruction,
    GuardStart(Direction),
}

pub struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<Tile>>,
}

impl Map {
    pub fn translate_in(&self, pos: &Pos, dir: &Direction) -> Option<Pos> {
        let (delta_x, delta_y) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };

        let updated_x = (pos.x as i32) + delta_x;
        let updated_y = (pos.y as i32) + delta_y;

        if updated_x < 0 || updated_y < 0 {
            return None;
        }

        let updated_x = updated_x as usize;
        let updated_y = updated_y as usize;

        if updated_x >= self.width || updated_y >= self.height {
            return None;
        }

        return Some(Pos {
            x: updated_x,
            y: updated_y,
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn rotate_90(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn parse<'a>(input: &str) -> Map {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '^' => Tile::GuardStart(Direction::Up),
                    '.' => Tile::Empty,
                    '#' => Tile::Obstruction,
                    _ => panic!("Unknown tile {c}"),
                })
                .collect()
        })
        .collect();

    let height = map.len();
    let width = map[0].len();

    Map { map, height, width }
}

pub fn solve_part_one<'a>(map: Map) -> String {
    let mut guard_pos = Pos { x: 0, y: 0 };
    let mut guard_dir = Direction::Down;

    let mut visited: HashSet<Pos> = HashSet::new();

    for (y, row) in map.map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Tile::GuardStart(d) = c {
                guard_pos = Pos { x, y };
                guard_dir = d.clone();
            }
        }
    }

    visited.insert(guard_pos.clone());

    loop {
        let Some(new_pos) = map.translate_in(&guard_pos, &guard_dir) else {
            break;
        };

        // println!("new_pos {new_pos:?} {guard_dir:?} ({guard_pos:?})");

        match map.map[new_pos.y][new_pos.x] {
            Tile::Obstruction => {
                // println!("Obstruction!");
                guard_dir = guard_dir.rotate_90();
                continue;
            }
            ref c => {
                // println!("Got tile {c:?}");
                visited.insert(new_pos.clone());
                guard_pos = new_pos;
            }
        }
    }

    visited.len().to_string()
}

fn is_loop(map: &Map, start_pos: Pos, start_dir: Direction) -> bool {
    let mut curr_pos = start_pos;
    let mut curr_dir = start_dir;

    let mut visited: HashSet<(Pos, Direction)> = HashSet::new();
    visited.insert((curr_pos.clone(), curr_dir.clone()));

    loop {
        let Some(new_pos) = map.translate_in(&curr_pos, &curr_dir) else {
            return false;
        };

        if visited.contains(&(new_pos.clone(), curr_dir.clone())) {
            return true;
        }

        match map.map[new_pos.y][new_pos.x] {
            Tile::Obstruction => {
                curr_dir = curr_dir.rotate_90();
                continue;
            }
            _ => {
                visited.insert((new_pos.clone(), curr_dir.clone()));
                curr_pos = new_pos;
            }
        }
    }
}

pub fn solve_part_two<'a>(input: Map) -> String {
    let mut guard_pos = Pos { x: 0, y: 0 };
    let mut guard_dir = Direction::Down;

    for (y, row) in input.map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Tile::GuardStart(d) = c {
                guard_pos = Pos { x, y };
                guard_dir = d.clone();
            }
        }
    }

    let mut tiles = input.map.clone();
    let mut loop_counts = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            // println!("Checking ({x}, {y})");
            match input.map[y][x] {
                Tile::Empty => {
                    tiles[y][x] = Tile::Obstruction;
                    if is_loop(
                        &Map {
                            width: input.width,
                            height: input.height,
                            map: tiles.clone(),
                        },
                        guard_pos.clone(),
                        guard_dir.clone(),
                    ) {
                        loop_counts += 1;
                    }
                    tiles[y][x] = Tile::Empty;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    loop_counts.to_string()
}
