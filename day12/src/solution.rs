use std::collections::HashSet;

pub struct Garden {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Garden {
    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut n = vec![];
        if x > 0 {
            n.push((x - 1, y));
        }

        if y > 0 {
            n.push((x, y - 1));
        }

        if x < self.width - 1 {
            n.push((x + 1, y));
        }

        if y < self.height - 1 {
            n.push((x, y + 1));
        }

        n
    }

    pub fn get_neighbours_opt(&self, x: usize, y: usize) -> Vec<Option<(usize, usize)>> {
        let mut n = vec![];
        if x > 0 {
            n.push(Some((x - 1, y)));
        } else {
            n.push(None);
        }

        if y > 0 {
            n.push(Some((x, y - 1)));
        } else {
            n.push(None);
        }

        if x < self.width - 1 {
            n.push(Some((x + 1, y)));
        } else {
            n.push(None);
        }

        if y < self.height - 1 {
            n.push(Some((x, y + 1)));
        } else {
            n.push(None);
        }

        n
    }

    fn get_at(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x > self.width - 1 || y > self.height - 1 {
            return None;
        }

        Some(self.map[y][x])
    }
}

pub fn parse<'a>(input: &str) -> Garden {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let height = map.len();
    let width = map[0].len();

    Garden { map, width, height }
}

fn calculate_perimiter(map: &Garden, region: &Vec<(usize, usize)>) -> usize {
    let mut perimiter = 0;
    for (x, y) in region.iter() {
        for neigh in map.get_neighbours_opt(*x, *y) {
            if let Some((nx, ny)) = neigh {
                if !region.contains(&(nx, ny)) {
                    perimiter += 1;
                }
            } else {
                // On the edge of the map.
                perimiter += 1;
            }
        }
    }

    perimiter
}

pub fn solve_part_one<'a>(input: Garden) -> String {
    let mut regions: Vec<Vec<(usize, usize)>> = vec![];

    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    let mut future_to_check: Vec<(usize, usize)> = vec![(0, 0)];

    while let Some((x, y)) = future_to_check.pop() {
        if checked.contains(&(x, y)) {
            continue;
        }

        let mut current_region = vec![];
        let region_char = input.map[y][x];

        // println!("Checking new region starting at {x} {y} with char {region_char}");

        let mut to_check = vec![(x, y)];

        while let Some((x, y)) = to_check.pop() {
            // println!("Checking region tile at {x} {y}");
            checked.insert((x, y));
            current_region.push((x, y));

            for (nx, ny) in input.get_neighbours(x, y) {
                if checked.contains(&(nx, ny)) {
                    continue;
                }

                let nc = input.map[ny][nx];

                if nc == region_char {
                    if !to_check.contains(&(nx, ny)) {
                        to_check.push((nx, ny));
                    }
                    continue;
                }

                if !future_to_check.contains(&(nx, ny)) {
                    future_to_check.push((nx, ny));
                }
            }
        }

        regions.push(current_region);
    }

    let mut price = 0;
    while let Some(region) = regions.pop() {
        let perimiter = calculate_perimiter(&input, &region);
        let area = region.len();

        let region_cost = area * perimiter;

        // println!("Region {region:?} => {perimiter} * {area} = {region_cost} ");

        price += region_cost;
    }

    price.to_string()
}

fn is_corner(c: char, first: Option<char>, second: Option<char>, diagonal: Option<char>) -> bool {
    match (first, second, diagonal) {
        (None, None, _) => true,
        (None, Some(c2), _) => c2 != c,
        (Some(c2), None, _) => c2 != c,
        (Some(c2), Some(c3), Some(c4)) => (c2 != c && c3 != c) || (c == c2 && c == c3 && c != c4),
        _ => panic!("Invalid case"),
    }
}

fn calculate_perimiter_2(map: &Garden, region: &Vec<(usize, usize)>) -> usize {
    let c = map.map[region[0].1][region[0].0];

    let mut edges = 0;
    for (x, y) in region.iter() {
        // println!("{x} {y}:");
        let x = (*x) as i32;
        let y = (*y) as i32;
        let above = map.get_at(x, y - 1);
        let below = map.get_at(x, y + 1);
        let left = map.get_at(x - 1, y);
        let right = map.get_at(x + 1, y);

        if is_corner(c, above, right, map.get_at(x + 1, y - 1)) {
            // println!("Top-right is a corner ({c}, {above:?}, {right:?})");
            edges += 1;
        }
        if is_corner(c, right, below, map.get_at(x + 1, y + 1)) {
            // println!("Bottom-right is a corner");
            edges += 1;
        }
        if is_corner(c, below, left, map.get_at(x - 1, y + 1)) {
            // println!("Bottom-left is a corner");
            edges += 1;
        }
        if is_corner(c, left, above, map.get_at(x - 1, y - 1)) {
            // println!("Top-left is a corner");
            edges += 1;
        }
    }

    edges
}

pub fn solve_part_two<'a>(input: Garden) -> String {
    let mut regions: Vec<Vec<(usize, usize)>> = vec![];

    let mut checked: HashSet<(usize, usize)> = HashSet::new();
    let mut future_to_check: Vec<(usize, usize)> = vec![(0, 0)];

    while let Some((x, y)) = future_to_check.pop() {
        if checked.contains(&(x, y)) {
            continue;
        }

        let mut current_region = vec![];
        let region_char = input.map[y][x];

        // println!("Checking new region starting at {x} {y} with char {region_char}");

        let mut to_check = vec![(x, y)];

        while let Some((x, y)) = to_check.pop() {
            // println!("Checking region tile at {x} {y}");
            checked.insert((x, y));
            current_region.push((x, y));

            for (nx, ny) in input.get_neighbours(x, y) {
                if checked.contains(&(nx, ny)) {
                    continue;
                }

                let nc = input.map[ny][nx];

                if nc == region_char {
                    if !to_check.contains(&(nx, ny)) {
                        to_check.push((nx, ny));
                    }
                    continue;
                }

                if !future_to_check.contains(&(nx, ny)) {
                    future_to_check.push((nx, ny));
                }
            }
        }

        regions.push(current_region);
    }

    let mut price = 0;
    while let Some(region) = regions.pop() {
        let perimiter = calculate_perimiter_2(&input, &region);
        let area = region.len();

        let region_cost = area * perimiter;

        // println!("Region {region:?} => {perimiter} * {area} = {region_cost} ");

        price += region_cost;
    }

    price.to_string()
}
