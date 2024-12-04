pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = char> + 'a> + 'a {
    input.lines().map(|l| l.chars())
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = impl Iterator<Item = char>>) -> String {
    let matrix: Vec<Vec<char>> = input.map(|ls| ls.collect()).collect();

    let height = matrix.len();
    let width = matrix[0].len();
    let mut count = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'X' {
                // Check outwards in all directions to see if we have 'XMAS'

                // Up
                if y >= 3 {
                    if matrix[y - 1][x] == 'M' && matrix[y - 2][x] == 'A' && matrix[y - 3][x] == 'S'
                    {
                        count += 1;
                    }
                }

                // Down
                if y < height - 3 {
                    if matrix[y + 1][x] == 'M' && matrix[y + 2][x] == 'A' && matrix[y + 3][x] == 'S'
                    {
                        count += 1;
                    }
                }

                // Left
                if x >= 3 {
                    if matrix[y][x - 1] == 'M' && matrix[y][x - 2] == 'A' && matrix[y][x - 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Right
                if x < width - 3 {
                    if matrix[y][x + 1] == 'M' && matrix[y][x + 2] == 'A' && matrix[y][x + 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Up-Left
                if y >= 3 && x >= 3 {
                    if matrix[y - 1][x - 1] == 'M'
                        && matrix[y - 2][x - 2] == 'A'
                        && matrix[y - 3][x - 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Up-Right
                if y >= 3 && x < width - 3 {
                    if matrix[y - 1][x + 1] == 'M'
                        && matrix[y - 2][x + 2] == 'A'
                        && matrix[y - 3][x + 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Down-Left
                if y < height - 3 && x >= 3 {
                    if matrix[y + 1][x - 1] == 'M'
                        && matrix[y + 2][x - 2] == 'A'
                        && matrix[y + 3][x - 3] == 'S'
                    {
                        count += 1;
                    }
                }

                // Down-Right
                if y < height - 3 && x < width - 3 {
                    if matrix[y + 1][x + 1] == 'M'
                        && matrix[y + 2][x + 2] == 'A'
                        && matrix[y + 3][x + 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count.to_string()
}

pub fn solve_part_two<'a>(input: impl Iterator<Item = impl Iterator<Item = char>>) -> String {
    let matrix: Vec<Vec<char>> = input.map(|l| l.collect()).collect();

    let height = matrix.len();
    let width = matrix[0].len();

    let mut count = 0;

    for (y, l) in matrix.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if c == &'A' && y > 0 && x > 0 && x < width - 1 && y < width - 1 {
                let words = vec![
                    format!(
                        "{}{}{}",
                        matrix[y - 1][x - 1],
                        matrix[y][x],
                        matrix[y + 1][x + 1]
                    ),
                    format!(
                        "{}{}{}",
                        matrix[y - 1][x + 1],
                        matrix[y][x],
                        matrix[y + 1][x - 1]
                    ),
                    format!(
                        "{}{}{}",
                        matrix[y + 1][x + 1],
                        matrix[y][x],
                        matrix[y - 1][x - 1]
                    ),
                    format!(
                        "{}{}{}",
                        matrix[y + 1][x - 1],
                        matrix[y][x],
                        matrix[y - 1][x + 1]
                    ),
                ];

                if words.into_iter().filter(|w| w.as_str() == "MAS").count() == 2 {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}
