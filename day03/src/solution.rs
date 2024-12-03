pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = char> + 'a {
    input.chars()
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = char>) -> String {
    let mut curr = 0;
    let mut first_num: Option<Vec<char>> = None;
    let mut second_num: Option<Vec<char>> = None;

    let mut nums: Vec<(u32, u32)> = vec![];

    for c in input {
        match (c, curr, first_num.as_mut(), second_num.as_mut()) {
            ('m', 0, None, None) => {
                curr = 1;
            }
            ('u', 1, None, None) => {
                curr = 2;
            }
            ('l', 2, None, None) => {
                curr = 3;
            }
            ('(', 3, None, None) => {
                curr = 4;
            }
            (a, 4, None, None) if a.is_digit(10) => {
                first_num = Some(vec![a]);
            }
            (a, 4, Some(nums), None) if a.is_digit(10) => {
                nums.push(a);
            }
            (',', 4, Some(_), None) => {
                curr = 5;
            }
            (a, 5, Some(_), None) => {
                second_num = Some(vec![a]);
            }
            (a, 5, Some(_), Some(nums)) if a.is_digit(10) => {
                nums.push(a);
            }
            (')', 5, Some(first), Some(second)) => {
                // We're done!
                let first = first.iter().collect::<String>();
                let first: u32 = first.parse().unwrap();

                let second = second.iter().collect::<String>();
                let second: u32 = second.parse().unwrap();
                nums.push((first, second));

                curr = 0;
                first_num = None;
                second_num = None;
            }
            _ => {
                curr = 0;
                first_num = None;
                second_num = None;
            }
        }
    }

    nums.into_iter()
        .map(|(a, b)| a * b)
        .sum::<u32>()
        .to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = char>) -> String {
    let mut curr_mul = 0;
    let mut first_num: Option<Vec<char>> = None;
    let mut second_num: Option<Vec<char>> = None;

    let mut nums: Vec<(u32, u32)> = vec![];

    let mut enabled = true;

    let mut curr_do_dont = 0;

    for c in input {
        match (
            c,
            curr_mul,
            curr_do_dont,
            first_num.as_mut(),
            second_num.as_mut(),
            enabled,
        ) {
            ('m', 0, _, None, None, true) => {
                curr_do_dont = 0;
                curr_mul = 1;
            }
            ('u', 1, _, None, None, true) => {
                curr_mul = 2;
            }
            ('l', 2, _, None, None, true) => {
                curr_mul = 3;
            }
            ('(', 3, _, None, None, true) => {
                curr_mul = 4;
            }
            (a, 4, _, None, None, true) if a.is_digit(10) => {
                first_num = Some(vec![a]);
            }
            (a, 4, _, Some(nums), None, true) if a.is_digit(10) => {
                nums.push(a);
            }
            (',', 4, _, Some(_), None, true) => {
                curr_mul = 5;
            }
            (a, 5, _, Some(_), None, true) => {
                second_num = Some(vec![a]);
            }
            (a, 5, _, Some(_), Some(nums), true) if a.is_digit(10) => {
                nums.push(a);
            }
            (')', 5, _, Some(first), Some(second), true) => {
                // We're done!
                let first = first.iter().collect::<String>();
                let first: u32 = first.parse().unwrap();

                let second = second.iter().collect::<String>();
                let second: u32 = second.parse().unwrap();
                nums.push((first, second));

                curr_mul = 0;
                first_num = None;
                second_num = None;
            }
            ('d', _, 0, _, _, _) => {
                curr_mul = 0;
                first_num = None;
                second_num = None;
                curr_do_dont = 1;
            }
            ('o', _, 1, _, _, _) => {
                curr_do_dont = 2;
            }
            ('(', _, 2, _, _, _) => {
                curr_do_dont = 10; // 10 is for do xd
            }
            (')', _, 10, _, _, _) => {
                curr_do_dont = 0;
                enabled = true;
            }
            ('n', _, 2, _, _, _) => {
                curr_do_dont = 3;
            }
            ('\'', _, 3, _, _, _) => {
                curr_do_dont = 4;
            }
            ('t', _, 4, _, _, _) => {
                curr_do_dont = 5;
            }
            ('(', _, 5, _, _, _) => {
                curr_do_dont = 6;
            }
            (')', _, 6, _, _, _) => {
                curr_do_dont = 0;
                enabled = false;
            }
            _ => {
                curr_do_dont = 0;
                curr_mul = 0;
                first_num = None;
                second_num = None;
            }
        }
    }

    nums.into_iter()
        .map(|(a, b)| a * b)
        .sum::<u32>()
        .to_string()
}
