use std::{
    collections::{HashMap, HashSet},
    ops::{BitXor, Div},
};

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = u128> + 'a {
    input.lines().map(|l| l.parse().unwrap())
}

#[inline(always)]
fn mix_prune(secret: u128, val: u128) -> u128 {
    let new_secret = secret.bitxor(val);
    new_secret % 16777216
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = u128>) -> String {
    let mut sum = 0;

    for secret in input {
        let mut prev = secret;
        for _ in 0..2000 {
            prev = calc_next(prev);
        }
        sum += prev;
    }

    sum.to_string()
}

#[inline(always)]
fn calc_next(secret: u128) -> u128 {
    let val = secret * 64;
    let mut prev = mix_prune(secret, val);

    let val = prev.div(32);
    prev = mix_prune(prev, val);

    let val = prev * 2048;
    prev = mix_prune(prev, val);
    prev
}

const ITERS: usize = 2000;

pub fn solve_part_two<'a>(input: impl Iterator<Item = u128>) -> String {
    let mut sequences_map: HashMap<(i8, i8, i8, i8), i64> = HashMap::new();

    for secret in input {
        let mut seqs: HashSet<(i8, i8, i8, i8)> = HashSet::new();
        let mut prev = secret;
        let mut prev_v = (prev % 10) as i8;
        let mut window = vec![];

        for i in 0..ITERS {
            let next = calc_next(prev);
            let v = (next % 10) as i8;
            let delta = v - prev_v;
            window.push(delta);

            if i >= 3 {
                let key = (window[i - 3], window[i - 2], window[i - 1], window[i]);
                // println!("KEY {key:?}");
                if seqs.contains(&key) {
                    prev = next;
                    prev_v = v;
                    // Ensure that we only count the first occurance of the sequence for each secret.
                    continue;
                }

                seqs.insert(key);

                if let Some(n) = sequences_map.get_mut(&key) {
                    *n = *n + (v as i64);
                } else {
                    sequences_map.insert(key, v as i64);
                }
            }

            prev = next;
            prev_v = v;

            // println!("Delta {delta} val {v}");
        }
    }

    // println!("{}", sequences_map.get(&(-2, 1, -1, 3)).unwrap());

    sequences_map.values().max().unwrap().to_string()
}
