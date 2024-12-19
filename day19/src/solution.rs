use std::collections::HashMap;

pub struct Input {
    pub available: Vec<String>,
    pub requested: Vec<String>,
}

pub fn parse<'a>(input: &str) -> Input {
    let (available, requested) = input.split_once("\n\n").unwrap();

    let available = available
        .split(", ")
        .map(|towel| towel.chars().collect::<String>())
        .collect::<Vec<_>>();
    let requested = requested
        .lines()
        .map(|towel| towel.chars().collect::<String>())
        .collect::<Vec<_>>();

    Input {
        available,
        requested,
    }
}

fn can_solve_for(
    towel: String,
    available: &Vec<String>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if let Some(c) = cache.get(&towel) {
        return *c;
    }

    for t in available.iter() {
        if t.len() > towel.len() {
            continue;
        }

        if t == &towel {
            cache.insert(towel, true);
            return true;
        }

        if let Some(new_towel) = towel.strip_prefix(t) {
            if can_solve_for(new_towel.to_string(), available, cache) {
                cache.insert(towel, true);
                return true;
            }
        }
    }

    cache.insert(towel, false);
    false
}

pub fn solve_part_one(input: Input) -> String {
    let Input {
        available,
        requested,
    } = input;

    let mut num = 0;
    let mut cache: HashMap<String, bool> = HashMap::new();

    for towel in requested {
        if can_solve_for(towel, &available, &mut cache) {
            num += 1;
        }
    }

    num.to_string()
}

fn can_solve_for_2(
    towel: String,
    available: &Vec<String>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(c) = cache.get(&towel) {
        return *c;
    }

    let mut n = 0;

    for t in available.iter() {
        if t.len() > towel.len() {
            continue;
        }

        if t == &towel {
            n += 1;
        }
        if let Some(new_towel) = towel.strip_prefix(t) {
            n += can_solve_for_2(new_towel.to_string(), available, cache);
        }
    }

    cache.insert(towel, n);
    n
}

pub fn solve_part_two(input: Input) -> String {
    let Input {
        available,
        requested,
    } = input;

    let mut cache: HashMap<String, u64> = HashMap::new();

    let mut sum = 0;
    for towel in requested {
        sum += can_solve_for_2(towel, &available, &mut cache);
    }

    sum.to_string()
}
