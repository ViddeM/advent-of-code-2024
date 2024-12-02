pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = i32> + 'a> + 'a {
    input
        .lines()
        .map(|l| l.split(" ").map(|n| n.parse().expect("Parse failed")))
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = impl Iterator<Item = i32>> + 'a) -> String {
    let count = input.map(|report| is_safe(report)).filter(|t| *t).count();
    count.to_string()
}

fn is_safe<'a>(report: impl Iterator<Item = i32> + 'a) -> bool {
    let mut is_inc = None;
    let mut prev = None;
    for num in report {
        if let Some(p) = prev {
            let delta = p - num;
            if delta < -3 || delta == 0 || delta > 3 {
                return false;
            }

            if is_inc.is_none() {
                if delta > 0 {
                    is_inc = Some(false)
                } else {
                    is_inc = Some(true);
                }
            }

            let inc = is_inc.as_ref().expect("is_inc should not be none");

            if delta > 0 && *inc || delta < 0 && !inc {
                return false;
            }
        }
        prev = Some(num);
    }

    true
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = impl Iterator<Item = i32>> + 'a) -> String {
    let count = input.map(|report| is_safe_2(report)).filter(|t| *t).count();
    count.to_string()
}

fn is_safe_2<'a>(report: impl Iterator<Item = i32> + 'a) -> bool {
    let report: Vec<i32> = report.collect();
    if is_safe_2_inner(&report) {
        return true;
    }

    for (i, _) in report.iter().enumerate() {
        let mut new = report.clone();
        new.remove(i);
        if is_safe_2_inner(&new) {
            return true;
        }
    }

    false
}

fn is_safe_2_inner(report: &Vec<i32>) -> bool {
    let first = report[0];
    let second = report[1];

    let is_inc = if first < second {
        true
    } else if second < first {
        false
    } else {
        // Delta of 0
        return false;
    };

    let mut prev = first - if is_inc { 1 } else { -1 };
    for num in report {
        let delta = num - prev;

        let delta = if is_inc { delta } else { -delta };

        if delta < 1 || delta > 3 {
            return false;
        }
        prev = num.clone();
    }

    true
}
