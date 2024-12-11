use std::collections::HashMap;

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = u128> + 'a {
    input.trim().split(" ").map(|s| s.parse::<u128>().unwrap())
}

fn handle_stone(stone: u128) -> Vec<u128> {
    if stone == 0 {
        return vec![1];
    }

    let stone_chars: Vec<char> = stone.to_string().chars().collect();
    if stone_chars.len() % 2 == 0 {
        // Event number of digits
        let left: String = stone_chars[..stone_chars.len() / 2].iter().collect();
        let right: String = stone_chars[stone_chars.len() / 2..].iter().collect();

        return vec![
            left.parse::<u128>().unwrap(),
            right.parse::<u128>().unwrap(),
        ];
    }

    vec![stone * 2024]
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = u128>) -> String {
    let mut stones: Vec<u128> = input.collect();

    for blink in 1..=25 {
        let mut new_stones = vec![];
        for stone in stones.into_iter() {
            let mut new = handle_stone(stone);
            new_stones.append(&mut new);
        }
        println!("Stones after blink {blink}: {new_stones:?}");
        stones = new_stones;
    }

    stones.len().to_string()
}

pub fn solve_stone_rec(
    rounds_remaining: usize,
    stone: u128,
    cache: &mut HashMap<(u128, usize), usize>,
) -> usize {
    // println!("Rounds left {rounds_remaining} stone {stone} cache {cache:?}");

    if rounds_remaining == 0 {
        return 1;
    }

    let new_stones = handle_stone(stone);
    let mut count = 0;
    for stone in new_stones.into_iter() {
        if let Some(c) = cache.get(&(stone, rounds_remaining)) {
            count += c;
            continue;
        }

        let c = solve_stone_rec(rounds_remaining - 1, stone, cache);
        cache.insert((stone, rounds_remaining), c);
        count += c;
    }

    count
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = u128>) -> String {
    let mut cache: HashMap<(u128, usize), usize> = HashMap::new();

    let mut sum = 0;
    for stone in input {
        let count = solve_stone_rec(75, stone, &mut cache);
        sum += count;
    }

    sum.to_string()
}
