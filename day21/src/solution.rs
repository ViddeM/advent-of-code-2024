use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

pub struct Input {
    num: usize,
    raw: Vec<char>,
}

pub fn parse<'a>(input: &str) -> Vec<Input> {
    input.lines().map(|l| {
        let num = l.strip_suffix("A").unwrap().parse().unwrap();
        let raw = l.chars().collect();

        Input {
            num,
            raw
        }
    }).collect()
}

const DIR_PAD: &[&[char]] = &[&[' ', '^', 'A'], &['<', 'v', '>']];
const NUM_PAD: &[&[char]] = &[&['7', '8', '9'], &['4', '5', '6'], &['1', '2', '3'], &[' ', '0', 'A']];
const DIR_PAD_CHARS: [char; 5] = ['A', '>', 'v', '<', '^'];

fn perform_move(x: usize, y: usize, c: char, pad: &[&[char]]) -> Option<((usize, usize), Option<char>)> {
    // println!("Performing move {x} {y} {c} {pad:?}");
    match c {
        '^' => {
            if y < 1 {
                return None;
            }
            Some(((x, y - 1), None))
        }
        '>' => {
            if x > 2 {
                return None;
            }

            Some(((x + 1, y), None))
        },
        'v' => {
            if y > pad.len() - 2 {
                return None;
            }

            Some(((x, y + 1), None))
        },
        '<' => {
            if x < 1 {
                return None;
            }

            Some(((x - 1, y), None))
        },
        'A' => Some(((x, y), Some(pad[y][x]))),
        c => panic!("Invalid char {c}")
    }
}

/// Get the steps from the top-left corner of the numpad in (x, y)
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
fn steps_to_start(char: char) -> (usize, usize) {
    match char {
        '^' => (1, 0),
        '>' => (2, 1),
        'v' => (1, 1),
        '<' => (0, 1),
        'A' => (2, 0),
        c => panic!("Invalid char {c}")
    }
}

fn find_cost(cache: &mut HashMap<(char, char, usize), usize>, goal: char, prev_char: char, remaining_pads: usize) -> usize {
    // println!("Checking cost to get from {prev_char} to {goal}");
    if remaining_pads == 0 {
        // We've reached the end!
        return 1;
    }

    if let Some(&cached) = cache.get(&(goal, prev_char, remaining_pads)) {
        return cached;
    }

    let to_start = steps_to_start(prev_char);
    
    let mut to_check = BinaryHeap::new();
    to_check.push(Reverse((0, to_start, 'A', ' ')));

    while let Some(Reverse((num, (x, y), prev, res))) = to_check.pop() {
        if res == goal {
            cache.insert((goal, prev_char, remaining_pads), num);
            return num;
        }

        for c in DIR_PAD_CHARS {
            let Some(((nx, ny), new_prev)) = perform_move(x, y, c, DIR_PAD) else { continue; };
            let nc = DIR_PAD.get(ny).and_then(|row| row.get(nx)).unwrap_or(&' ');
            if nc == &' ' {
                continue;
            }

            let new_prev = if let Some(new_prev) = new_prev {
                if new_prev != goal {
                    continue;
                }
                new_prev
            } else {
                ' '
            };

            let new_num = num + find_cost(cache, c, prev, remaining_pads - 1);
            to_check.push(Reverse((new_num, (nx, ny), c, new_prev)));
        }
    }

    panic!("No cost found");
}

fn find_shortest_sequence(cache: &mut HashMap<(char, char, usize), usize>, chars: &Vec<char>, remaining_pads: usize) -> usize {
    let mut to_check = BinaryHeap::new();
    to_check.push(Reverse((0, (2, 3), 'A', 0))); 

    let mut visited = HashSet::new();

    while let Some(Reverse((cost, (x, y), prev, length))) = to_check.pop() {
        // println!("Checking {x} {y} {prev} {length}");
        if length == chars.len() {
            return cost;
        }

        if visited.contains(&((x, y), prev, length)) {
            continue;
        }

        visited.insert(((x, y), prev, length));
        for c in DIR_PAD_CHARS {
            let Some(((nx, ny), new_prev)) = perform_move(x, y, c, NUM_PAD) else { continue; };

            let nc = NUM_PAD.get(ny).and_then(|row| row.get(nx)).unwrap_or(&' ');            
            if nc == &' ' {
                continue;
            }

            let new_length = if let Some(new_prev) = new_prev {
                if new_prev != chars[length] {
                    continue;
                }
                length + 1
            } else {
                length
            };

            let new_cost = cost + find_cost(cache, c, prev, remaining_pads);
            to_check.push(Reverse((new_cost, (nx, ny), c, new_length)));
        }
    }

    panic!("No sequence found");
}

pub fn solve_part_one<'a>(input: Vec<Input>) -> String {
    let mut cache = HashMap::new();
    let mut sum = 0;

    for l in input.iter() {
        sum += l.num * find_shortest_sequence(&mut cache, &l.raw, 2);
    }

    sum.to_string()
}

pub fn solve_part_two<'a>(input: Vec<Input>) -> String {
let mut cache = HashMap::new();
    let mut sum = 0;

    for l in input.iter() {
        sum += l.num * find_shortest_sequence(&mut cache, &l.raw, 25);
    }

    sum.to_string()
}
