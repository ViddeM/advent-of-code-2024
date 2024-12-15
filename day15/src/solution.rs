use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Box,
    Wall,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

pub struct Input {
    pub start_pos: (usize, usize),
    pub map: Map,
    pub movements: Vec<Direction>,
}

pub fn parse<'a>(input: &str) -> Input {
    let (map, movements) = input.split_once("\n\n").unwrap();

    let mut start_x = 0;
    let mut start_y = 0;

    let map = map
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => {
                        start_x = x;
                        start_y = y;
                        Tile::Empty
                    }
                    c => panic!("Unexpected char {c}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = map[0].len();
    let height = map.len();

    let movements = movements
        .chars()
        .filter(|c| c != &'\n')
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            c => panic!("Unexpected movements {c}"),
        })
        .collect::<Vec<_>>();

    Input {
        start_pos: (start_x, start_y),
        map: Map {
            tiles: map,
            width,
            height,
        },
        movements,
    }
}

fn check_move(x: usize, y: usize, dir: &Direction, map: &Map) -> Option<usize> {
    let (dx, dy) = dir.delta();

    let mut test_x = x;
    let mut test_y = y;
    let mut steps = 0;
    loop {
        test_x = ((test_x as i32) + dx) as usize;
        test_y = ((test_y as i32) + dy) as usize;

        if test_x < 1 || test_y < 1 || test_x >= map.width - 1 || test_y >= map.height - 1 {
            return None;
        }

        steps += 1;

        match map.tiles[test_y][test_x] {
            Tile::Box => continue,
            Tile::Wall => return None,
            Tile::Empty => return Some(steps),
        }
    }
}

fn perform_move(curr_x: i32, curr_y: i32, dir: &Direction, steps: i32, map: &mut Map) {
    let (dx, dy) = dir.delta();

    let mut x = curr_x + dx * (steps + 1);
    let mut y = curr_y + dy * (steps + 1);
    for _ in 0..steps {
        x -= dx;
        y -= dy;

        let next_x = (x - dx) as usize;
        let next_y = (y - dy) as usize;

        map.tiles[y as usize][x as usize] = map.tiles[next_y][next_x].clone();
    }
}

pub fn solve_part_one<'a>(input: Input) -> String {
    let mut map = input.map;

    let mut curr_x = input.start_pos.0;
    let mut curr_y = input.start_pos.1;

    for dir in input.movements {
        if let Some(steps) = check_move(curr_x, curr_y, &dir, &map) {
            perform_move(curr_x as i32, curr_y as i32, &dir, steps as i32, &mut map);

            curr_x = ((curr_x as i32) + dir.delta().0) as usize;
            curr_y = ((curr_y as i32) + dir.delta().1) as usize;
        }
    }

    let mut gps_sum = 0;
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if t == &Tile::Box {
                gps_sum += (y * 100) + x;
            }
        }
    }

    gps_sum.to_string()
}

impl Input {
    fn widen(self) -> Input2 {
        let Input {
            start_pos: (sx, sy),
            map,
            movements,
        } = self;

        let mut boxes = HashSet::new();
        let mut walls = HashSet::new();
        for (y, row) in map.tiles.into_iter().enumerate() {
            for (x, t) in row.into_iter().enumerate() {
                let real_x = x * 2;

                match t {
                    Tile::Box => {
                        boxes.insert((real_x, y));
                    }
                    Tile::Wall => {
                        walls.insert((real_x, y));
                        walls.insert((real_x + 1, y));
                    }
                    Tile::Empty => continue,
                }
            }
        }

        Input2 {
            start_pos: (sx * 2, sy),
            map: Map2 { boxes, walls },
            movements: movements,
        }
    }
}

struct Map2 {
    boxes: HashSet<(usize, usize)>,
    walls: HashSet<(usize, usize)>,
}

struct Input2 {
    start_pos: (usize, usize),
    map: Map2,
    movements: Vec<Direction>,
}

fn push_box(
    bx: usize,
    by: usize,
    dir: &Direction,
    boxes: &mut HashSet<(usize, usize)>,
    walls: &HashSet<(usize, usize)>,
) -> bool {
    let nx = ((bx as i32) + dir.delta().0) as usize;
    let ny = ((by as i32) + dir.delta().1) as usize;

    let lnx = nx - 1;
    let rnx = nx + 1;

    if walls.contains(&(nx, ny)) || walls.contains(&(rnx, ny)) {
        return false;
    }

    match dir {
        Direction::Up | Direction::Down => {
            if boxes.contains(&(nx, ny)) {
                if !push_box(nx, ny, dir, boxes, walls) {
                    return false;
                }
            }

            if boxes.contains(&(lnx, ny)) {
                if !push_box(lnx, ny, dir, boxes, walls) {
                    return false;
                }
            }

            if boxes.contains(&(rnx, ny)) {
                if !push_box(rnx, ny, dir, boxes, walls) {
                    return false;
                }
            }
        }
        Direction::Right => {
            if boxes.contains(&(rnx, ny)) {
                if !push_box(rnx, ny, dir, boxes, walls) {
                    return false;
                }
            }
        }
        Direction::Left => {
            if boxes.contains(&(lnx, ny)) {
                if !push_box(lnx, ny, dir, boxes, walls) {
                    return false;
                }
            }
        }
    }

    let remove = boxes.remove(&(bx, by));
    let insert = boxes.insert((nx, ny));
    true
}

fn print_map(
    px: usize,
    py: usize,
    boxes: &HashSet<(usize, usize)>,
    walls: &HashSet<(usize, usize)>,
) {
    let mut width: usize = 0;
    let mut height: usize = 0;
    for (x, y) in walls.iter() {
        if x > &width {
            width = *x;
        }
        if y > &height {
            height = *y;
        }
    }

    for y in 0..=height {
        for x in 0..=width {
            if walls.contains(&(x, y)) {
                print!("#");
            } else if boxes.contains(&(x, y)) {
                print!("[");
            } else if x > 0 && boxes.contains(&(x - 1, y)) {
                print!("]");
            } else if x == px && y == py {
                print!("@");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

pub fn solve_part_two<'a>(input: Input) -> String {
    let Input2 {
        start_pos: (sx, sy),
        map: Map2 { mut boxes, walls },
        movements,
    } = input.widen();

    let mut x = sx;
    let mut y = sy;

    for dir in movements.into_iter() {
        // print_map(x, y, &boxes, &walls);
        // println!("\nMove {}:", dir.char());

        let nx = ((x as i32) + dir.delta().0) as usize;
        let ny = ((y as i32) + dir.delta().1) as usize;

        let lnx = nx - 1;

        if walls.contains(&(nx, ny)) {
            continue;
        }

        if boxes.contains(&(nx, ny)) {
            let mut new_boxes = boxes.clone();
            if !push_box(nx, ny, &dir, &mut new_boxes, &walls) {
                continue;
            }

            boxes = new_boxes;
        } else if boxes.contains(&(lnx, ny)) {
            let mut new_boxes = boxes.clone();
            if !push_box(lnx, ny, &dir, &mut new_boxes, &walls) {
                continue;
            }

            boxes = new_boxes;
        }

        // println!("Updating from {x} {y} -> {nx} {ny}");

        x = nx;
        y = ny;
    }
    // print_map(x, y, &boxes, &walls);

    let mut sum = 0;
    for (bx, by) in boxes.into_iter() {
        sum += by * 100 + bx;
    }

    sum.to_string()
}
