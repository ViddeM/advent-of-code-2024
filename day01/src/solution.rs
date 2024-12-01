pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = (i32, i32)> + 'a {
    input.lines().map(|l| {
        let (a, b) = l.split_once("   ").expect("Failed to split!");
        (
            a.parse().expect("Failed to parse a"),
            b.parse().expect("Failed to parse b"),
        )
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = (i32, i32)>) -> String {
    let all_nums: Vec<(i32, i32)> = input.collect();

    let mut left: Vec<i32> = all_nums.iter().cloned().map(|(a, _)| a).collect();
    let mut right: Vec<i32> = all_nums.into_iter().map(|(_, b)| b).collect();

    left.sort();
    right.sort();

    let val: i32 = left
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, right[i]))
        .map(|(a, b)| (a - b).abs())
        .sum();

    val.to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = (i32, i32)>) -> String {
    let all_nums: Vec<(i32, i32)> = input.collect();

    let mut left: Vec<i32> = all_nums.iter().cloned().map(|(a, _)| a).collect();
    let mut right: Vec<i32> = all_nums.into_iter().map(|(_, b)| b).collect();

    left.into_iter()
        .map(|a| {
            let count = right.iter().filter(|b| b == &&a).count();

            (a as usize) * count
        })
        .sum::<usize>()
        .to_string()
}
