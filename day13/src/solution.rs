pub struct Game {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Game> + 'a {
    input.trim().split("\n\n").map(|l| {
        let mut lines = l.split("\n");

        let first = lines.next().unwrap();
        let (a_x, a_y) = first.split_once(", ").unwrap();
        let a_x = a_x
            .strip_prefix("Button A: X+")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let a_y = a_y.strip_prefix("Y+").unwrap().parse::<i64>().unwrap();

        let second = lines.next().unwrap();
        let (b_x, b_y) = second.split_once(", ").unwrap();
        let b_x = b_x
            .strip_prefix("Button B: X+")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let b_y = b_y.strip_prefix("Y+").unwrap().parse::<i64>().unwrap();

        let third = lines.next().unwrap();
        let (prize_x, prize_y) = third.split_once(", ").unwrap();
        let prize_x = prize_x
            .strip_prefix("Prize: X=")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let prize_y = prize_y.strip_prefix("Y=").unwrap().parse::<i64>().unwrap();

        Game {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        }
    })
}

pub fn solve_part_one<'a>(input: impl Iterator<Item = Game>) -> String {
    let mut sum = 0;
    for game in input {
        let divisor = game.a_x * game.b_y - game.a_y * game.b_x;

        let a_dividend = game.prize_x * game.b_y - game.prize_y * game.b_x;
        let b_dividend = game.prize_y * game.a_x - game.prize_x * game.a_y;

        if a_dividend % divisor == 0 && b_dividend % divisor == 0 {
            let a_presses = a_dividend / divisor;
            let b_presses = b_dividend / divisor;

            let game_sum = 3 * a_presses + b_presses;
            // println!("game sum {game_sum}");
            sum += game_sum;
        }
        // else {
        //     println!(
        //         "No solution for game with prize {} {}",
        //         game.prize_x, game.prize_y
        //     );
        // }
    }

    sum.to_string()
}

const BIG_NUMBER: i64 = 10_000_000_000_000;

pub fn solve_part_two<'a>(input: impl Iterator<Item = Game>) -> String {
    let mut sum = 0;
    for game in input {
        let prize_x = game.prize_x + BIG_NUMBER;
        let prize_y = game.prize_y + BIG_NUMBER;

        let divisor = game.a_x * game.b_y - game.a_y * game.b_x;

        let a_dividend = prize_x * game.b_y - prize_y * game.b_x;
        let b_dividend = prize_y * game.a_x - prize_x * game.a_y;

        if a_dividend % divisor == 0 && b_dividend % divisor == 0 {
            let a_presses = a_dividend / divisor;
            let b_presses = b_dividend / divisor;

            let game_sum = 3 * a_presses + b_presses;
            sum += game_sum;
        }
    }

    sum.to_string()
}
