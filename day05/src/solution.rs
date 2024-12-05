use std::{cmp::Ordering, collections::HashMap};

pub struct Input {
    before_map: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

pub fn parse<'a>(input: &str) -> Input {
    let (order, updates) = input.split_once("\n\n").unwrap();

    let mut before_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (l, r) in order.lines().map(|l| {
        let (l, r) = l.split_once("|").unwrap();
        (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
    }) {
        before_map.entry(l).or_insert_with(|| vec![]).push(r);
    }

    let updates = updates
        .lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    Input {
        before_map,
        updates,
    }
}

pub fn solve_part_one<'a>(input: Input) -> String {
    let Input {
        before_map,
        updates,
    } = input;

    let mut sum = 0;
    'update_loop: for update in updates.into_iter() {
        for (index, num) in update.iter().enumerate() {
            for later_num in update[index..].iter() {
                if let Some(after_nums) = before_map.get(later_num) {
                    if after_nums.contains(num) {
                        continue 'update_loop;
                    }
                }
            }
        }

        let middle_index = ((update.len() as f32) / 2.0).floor() as usize;
        sum += update[middle_index];
    }

    sum.to_string()
}

pub fn solve_part_two<'a>(input: Input) -> String {
    let Input {
        before_map,
        updates,
    } = input;

    let mut incorrect_updates = vec![];
    'update_loop: for update in updates.into_iter() {
        for (index, num) in update.iter().enumerate() {
            for later_num in update[index..].iter() {
                if let Some(after_nums) = before_map.get(later_num) {
                    if after_nums.contains(num) {
                        incorrect_updates.push(update);
                        continue 'update_loop;
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for incorrect_update in incorrect_updates.iter_mut() {
        incorrect_update.sort_by(|a, b| {
            if let Some(a_befores) = before_map.get(a) {
                if a_befores.contains(b) {
                    return Ordering::Less;
                }
            }

            if let Some(b_befores) = before_map.get(b) {
                if b_befores.contains(a) {
                    return Ordering::Greater;
                }
            }

            Ordering::Equal
        });

        let middle_index = ((incorrect_update.len() as f32) / 2.0).floor() as usize;
        sum += incorrect_update[middle_index];
    }

    sum.to_string()
}
