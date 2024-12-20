use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

pub struct Input {
    map: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn parse<'a>(input: &str) -> Input {
    let mut start = None;
    let mut end = None;

    let mut map = vec![];
    for (y, row) in input.lines().enumerate() {
        let mut r = vec![];
        for (x, char) in row.chars().enumerate() {
            match char {
                '.' => {
                    r.push(false);
                }
                '#' => {
                    r.push(true);
                }
                'S' => {
                    r.push(false);
                    start = Some((x, y));
                }
                'E' => {
                    r.push(false);
                    end = Some((x, y));
                }
                c => {
                    panic!("Invalid char '{c}'");
                }
            }
        }
        map.push(r);
    }

    let height = map.len();
    let width = map[0].len();

    Input {
        map,
        width,
        height,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

impl Input {
    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ns = vec![];

        if x > 0 && !self.map[y][x - 1] {
            ns.push((x - 1, y));
        }
        if y > 0 && !self.map[y - 1][x] {
            ns.push((x, y - 1));
        }
        if x < self.width - 1 && !self.map[y][x + 1] {
            ns.push((x + 1, y));
        }
        if y < self.height - 1 && !self.map[y + 1][x] {
            ns.push((x, y + 1));
        }

        ns
    }
}

pub fn solve_part_one<'a>(input: Input) -> String {
    let mut to_check = BinaryHeap::new();
    let mut path: HashMap<(usize, usize), usize> = HashMap::new();
    to_check.push((0, input.start));

    while let Some((n, (x, y))) = to_check.pop() {
        path.insert((x, y), n);

        if x == input.end.0 && y == input.end.1 {
            break;
        }

        for (nx, ny) in input.get_neighbours(x, y) {
            if !path.contains_key(&(nx, ny)) {
                to_check.push((n + 1, (nx, ny)));
            }
        }
    }

    let mut count = 0;

    for ((&(ax, ay), &an), (&(bx, by), &bn)) in path.iter().tuple_combinations() {
        let dist = ax.abs_diff(bx) + ay.abs_diff(by);
        let saved_picoseconds = an.abs_diff(bn);

        if dist <= 2 && saved_picoseconds >= dist + 100 {
            count += 1;
        }
    }

    count.to_string()
}

pub fn solve_part_two<'a>(input: Input) -> String {
    let mut to_check = BinaryHeap::new();
    let mut path: HashMap<(usize, usize), usize> = HashMap::new();
    to_check.push((0, input.start));

    while let Some((n, (x, y))) = to_check.pop() {
        path.insert((x, y), n);

        if x == input.end.0 && y == input.end.1 {
            break;
        }

        for (nx, ny) in input.get_neighbours(x, y) {
            if !path.contains_key(&(nx, ny)) {
                to_check.push((n + 1, (nx, ny)));
            }
        }
    }

    let mut count = 0;

    for ((&(ax, ay), &an), (&(bx, by), &bn)) in path.iter().tuple_combinations() {
        let dist = ax.abs_diff(bx) + ay.abs_diff(by);
        let saved_picoseconds = an.abs_diff(bn);

        if dist <= 20 && saved_picoseconds >= dist + 100 {
            count += 1;
        }
    }

    count.to_string()
}
