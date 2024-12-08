use std::collections::HashSet;

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = char> + 'a> + 'a {
    input.lines().map(|l| l.chars())
}

pub fn solve_part_one<'a>(
    input: impl Iterator<Item = impl Iterator<Item = char> + 'a> + 'a,
) -> String {
    let map: Vec<Vec<char>> = input.map(|l| l.collect()).collect();
    let height = map.len();
    let width = map[0].len();

    let mut antinode_locations: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'.' {
                // Skip dots
                continue;
            }

            for (y2, row2) in map.iter().enumerate() {
                if y2 < y {
                    continue;
                }

                for (x2, c2) in row2.iter().enumerate() {
                    if y2 == y && x2 <= x {
                        // Skip chars before x
                        continue;
                    }

                    if c == c2 {
                        match calculate_antinode(x, y, x2, y2, width, height) {
                            (None, None) => {}
                            (None, Some(pos)) => {
                                antinode_locations.insert(pos);
                            }
                            (Some(pos), None) => {
                                antinode_locations.insert(pos);
                            }
                            (Some(pos1), Some(pos2)) => {
                                antinode_locations.insert(pos1);
                                antinode_locations.insert(pos2);
                            }
                        }
                    }
                }
            }
        }
    }

    antinode_locations.len().to_string()
}

fn calculate_antinode(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    width: usize,
    height: usize,
) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let x1 = x1 as i64;
    let x2 = x2 as i64;
    let y1 = y1 as i64;
    let y2 = y2 as i64;
    let width = width as i64;
    let height = height as i64;

    let delta_x = x1 - x2;
    let delta_y = y1 - y2;

    // println!("{x1} {y1} | {x2} {y2} | {delta_x} {delta_y}");

    let a1_x = x1 + delta_x;
    let a1_y = y1 + delta_y;

    let a2_x = x1 - 2 * delta_x;
    let a2_y = y1 - 2 * delta_y;

    let antinode_1 = if a1_x < 0 || a1_x >= width || a1_y < 0 || a1_y >= height {
        None
    } else {
        Some((a1_x as usize, a1_y as usize))
    };

    let antinode_2 = if a2_x < 0 || a2_x >= width || a2_y < 0 || a2_y >= height {
        None
    } else {
        Some((a2_x as usize, a2_y as usize))
    };

    (antinode_1, antinode_2)
}

pub fn solve_part_two<'a>(
    input: impl Iterator<Item = impl Iterator<Item = char> + 'a> + 'a,
) -> String {
    let map: Vec<Vec<char>> = input.map(|l| l.collect()).collect();
    let height = map.len();
    let width = map[0].len();

    let mut antinode_locations: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'.' {
                // Skip dots
                continue;
            }

            for (y2, row2) in map.iter().enumerate() {
                if y2 < y {
                    continue;
                }

                for (x2, c2) in row2.iter().enumerate() {
                    if y2 == y && x2 <= x {
                        // Skip chars before x
                        continue;
                    }

                    if c == c2 {
                        for antinode in calculate_antinode2(x, y, x2, y2, width, height) {
                            antinode_locations.insert(antinode);
                        }
                    }
                }
            }
        }
    }

    antinode_locations.len().to_string()
}

fn calculate_antinode2(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let x1 = x1 as i64;
    let x2 = x2 as i64;
    let y1 = y1 as i64;
    let y2 = y2 as i64;
    let width = width as i64;
    let height = height as i64;

    let delta_x = x1 - x2;
    let delta_y = y1 - y2;

    // Work backwards
    let mut antinodes = vec![];
    let mut ax = x1;
    let mut ay = y1;
    while ax >= 0 && ax < width && ay >= 0 && ay < height {
        // println!("1: ax: {ax} ay: {ay}");
        antinodes.push((ax as usize, ay as usize));
        ax += delta_x;
        ay += delta_y;
    }

    ax = x1 - delta_x;
    ay = y1 - delta_y;
    while ax >= 0 && ax < width && ay >= 0 && ay < height {
        // println!("2: ax: {ax} ay: {ay}");
        antinodes.push((ax as usize, ay as usize));
        ax -= delta_x;
        ay -= delta_y;
    }

    antinodes
}
