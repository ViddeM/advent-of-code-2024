use std::collections::HashSet;

pub struct Map {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Map {
    fn find_trailheads(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &n)| if n == 0 { Some((x, y)) } else { None })
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect()
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if x > 0 {
            neighbours.push((x - 1, y));
        }

        if y > 0 {
            neighbours.push((x, y - 1));
        }

        if x < self.width - 1 {
            neighbours.push((x + 1, y));
        }

        if y < self.height - 1 {
            neighbours.push((x, y + 1));
        }

        neighbours
    }
}

pub fn parse<'a>(input: &'a str) -> Map {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let width = map[0].len();
    let height = map.len();

    Map { map, width, height }
}

pub fn solve_part_one<'a>(input: Map) -> String {
    let trailheads = input.find_trailheads();

    let mut sum = 0;
    for (th_x, th_y) in trailheads.into_iter() {
        // println!("Checking trailhead at {th_x} {th_y}");
        let mut has_checked: HashSet<(usize, usize)> = HashSet::new();
        let mut to_check = vec![(th_x, th_y)];

        let mut score = 0;
        while let Some((x, y)) = to_check.pop() {
            has_checked.insert((x, y));
            let n = input.map[y][x];

            // println!("Checking {x} {y} ({n})");
            if n == 9 {
                score += 1;
                continue;
            }

            for (nx, ny) in input.get_neighbours(x, y) {
                // println!("Checking neighbour {nx} {ny}");
                if has_checked.contains(&(nx, ny)) {
                    continue;
                }

                let nn = input.map[ny][nx];
                if nn == n + 1 {
                    to_check.push((nx, ny));
                }
            }
        }

        sum += score;
    }

    sum.to_string()
}

pub fn solve_part_two<'a>(input: Map) -> String {
    let trailheads = input.find_trailheads();

    let mut sum = 0;
    for (th_x, th_y) in trailheads.into_iter() {
        // println!("Checking trailhead at {th_x} {th_y}");
        let mut to_check = vec![(th_x, th_y)];

        let mut score = 0;
        while let Some((x, y)) = to_check.pop() {
            let n = input.map[y][x];

            // println!("Checking {x} {y} ({n})");
            if n == 9 {
                score += 1;
                continue;
            }

            for (nx, ny) in input.get_neighbours(x, y) {
                // println!("Checking neighbour {nx} {ny}");
                let nn = input.map[ny][nx];
                if nn == n + 1 {
                    to_check.push((nx, ny));
                }
            }
        }

        sum += score;
    }

    sum.to_string()
}
