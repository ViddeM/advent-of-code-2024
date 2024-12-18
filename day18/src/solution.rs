use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

const SIMULATED_STEPS: usize = 1024;

pub fn parse<'a>(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>()
}

fn get_neighbours(
    deleted: &Vec<(usize, usize)>,
    x: usize,
    y: usize,
    max_steps: usize,
) -> HashSet<(usize, usize)> {
    let mut neighbours = HashSet::new();
    if x > 0 {
        neighbours.insert((x - 1, y));
    }

    if y > 0 {
        neighbours.insert((x, y - 1));
    }

    if x < WIDTH - 1 {
        neighbours.insert((x + 1, y));
    }

    if y < WIDTH - 1 {
        neighbours.insert((x, y + 1));
    }

    for d in deleted.iter().take(max_steps) {
        neighbours.remove(d);
    }

    neighbours
}

fn print_map(path: Vec<(usize, usize)>, end_steps: usize, deleted: &Vec<(usize, usize)>) {
    let path = path.iter().collect::<HashSet<_>>();
    let deleted = deleted.iter().take(end_steps).collect::<HashSet<_>>();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if path.contains(&(x, y)) {
                print!("O");
            } else if deleted.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn find_path(deleted: &Vec<(usize, usize)>, max_steps: usize) -> Option<usize> {
    // Queue of Reversed steps, x, y
    let mut queue: BinaryHeap<Reverse<(usize, (usize, usize))>> = BinaryHeap::new();
    queue.push(Reverse((0, (0, 0))));

    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    dist.insert((0, 0), 0);

    while let Some(Reverse((steps, (x, y)))) = queue.pop() {
        if x == WIDTH - 1 && y == HEIGHT - 1 {
            println!("Found path");
            break;
        }

        let new_steps = steps + 1;
        for (nx, ny) in get_neighbours(&deleted, x, y, max_steps) {
            let prev_steps = dist.get(&(nx, ny)).unwrap_or(&usize::MAX);
            if new_steps < *prev_steps {
                dist.insert((nx, ny), new_steps);
                queue.push(Reverse((new_steps, (nx, ny))));
                came_from.insert((nx, ny), (x, y));
            }
        }
    }

    let res = dist.get(&(WIDTH - 1, HEIGHT - 1))?;

    Some(*res)
}

pub fn solve_part_one<'a>(input: Vec<(usize, usize)>) -> String {
    let res = find_path(&input, SIMULATED_STEPS);

    res.unwrap().to_string()
}

pub fn solve_part_two<'a>(input: Vec<(usize, usize)>) -> String {
    let mut steps = SIMULATED_STEPS;

    loop {
        println!("Checking {steps}");

        if find_path(&input, steps).is_none() {
            break;
        }

        steps += 1;
    }

    let (x, y) = input[steps - 1];

    format!("{x},{y}")
}
