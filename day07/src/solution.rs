pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = (u64, Vec<u64>)> + 'a {
    input.lines().map(|l| {
        let (res, vals) = l.split_once(": ").unwrap();
        let vals = vals
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        (res.parse::<u64>().unwrap(), vals)
    })
}

fn get_res_rec(sought: u64, curr: u64, index: usize, vals: &Vec<u64>) -> bool {
    if sought > curr {
        return false;
    }

    if vals.is_empty() {
        return sought == curr;
    }

    let v = vals[index];

    // Try addition first
    let sum = curr + v;
    if get_res_rec(sought, sum, index + 1, vals) {
        return true;
    }

    let product = curr * v;
    if get_res_rec(sought, product, index + 1, vals) {
        return true;
    }

    return false;
}

fn can_get_res(res: u64, vals: Vec<u64>) -> bool {
    let start_val = vals[0];
    get_res_rec(res, start_val, 1, &vals)
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = (u64, Vec<u64>)>) -> String {
    input
        .filter(|(res, vals)| can_get_res(*res, vals.clone()))
        .map(|(res, vals)| (res, vals))
        .map(|(res, _)| res)
        .sum::<u64>()
        .to_string()
}

fn get_res_rec_2(sought: u64, curr: u64, index: usize, vals: &Vec<u64>) -> bool {
    if curr > sought {
        return false;
    }

    if index == vals.len() {
        return sought == curr;
    }

    let v = vals[index];
    let n_index = index + 1;

    // Try addition first
    let sum = curr + v;
    if get_res_rec_2(sought, sum, n_index, vals) {
        return true;
    }

    let product = curr * v;
    if get_res_rec_2(sought, product, n_index, vals) {
        return true;
    }

    let combined_str = format!("{curr}{v}");
    let combined = combined_str.parse::<u64>().unwrap();
    if get_res_rec_2(sought, combined, n_index, vals) {
        return true;
    }

    return false;
}

fn can_get_res_2(res: u64, vals: Vec<u64>) -> bool {
    let start_val = vals[0];
    get_res_rec_2(res, start_val, 1, &vals)
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = (u64, Vec<u64>)>) -> String {
    input
        .filter(|(res, vals)| can_get_res_2(*res, vals.clone()))
        .map(|(res, vals)| (res, vals))
        .map(|(res, _)| res)
        .sum::<u64>()
        .to_string()
}
