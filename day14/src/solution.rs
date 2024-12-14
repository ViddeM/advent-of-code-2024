use std::collections::HashSet;

pub struct Robot {
    pos_x: usize,
    pos_y: usize,
    vel_x: i64,
    vel_y: i64,
}

pub fn parse<'a>(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|l| {
            let (pos, vel) = l.split_once(" ").unwrap();
            let (pos_x, pos_y) = pos.split_once(",").unwrap();
            let pos_x = pos_x.strip_prefix("p=").unwrap();
            let pos_x = pos_x.parse::<usize>().unwrap();
            let pos_y = pos_y.parse::<usize>().unwrap();

            let (vel_x, vel_y) = vel.split_once(",").unwrap();
            let vel_x = vel_x.strip_prefix("v=").unwrap();
            let vel_x = vel_x.parse::<i64>().unwrap();
            let vel_y = vel_y.parse::<i64>().unwrap();

            Robot {
                pos_x,
                pos_y,
                vel_x,
                vel_y,
            }
        })
        .collect()
}

const NUM_ROUNDS: i64 = 100;
// const ROOM_WIDTH: i64 = 101;
// const ROOM_HEIGHT: i64 = 103;

const ROOM_WIDTH: i64 = 101;
const ROOM_HEIGHT: i64 = 103;

pub fn solve_part_one<'a>(input: Vec<Robot>) -> String {
    let mut quad_1_count = 0;
    let mut quad_2_count = 0;
    let mut quad_3_count = 0;
    let mut quad_4_count = 0;

    let mid_x = (ROOM_WIDTH - 1) / 2;
    let mid_y = (ROOM_HEIGHT - 1) / 2;

    for robot in input.into_iter() {
        let change_x = robot.vel_x * NUM_ROUNDS;
        let change_y = robot.vel_y * NUM_ROUNDS;

        let new_x = (robot.pos_x as i64 + change_x) % ROOM_WIDTH;
        let new_y = (robot.pos_y as i64 + change_y) % ROOM_HEIGHT;

        let new_x = if new_x < 0 { new_x + ROOM_WIDTH } else { new_x };
        let new_y = if new_y < 0 {
            new_y + ROOM_HEIGHT
        } else {
            new_y
        };

        // println!(
        //     "Original: {} {}  change: {change_x} {change_y}  new: {new_x} {new_y}",
        //     robot.pos_x, robot.pos_y
        // );

        if new_x < mid_x && new_y < mid_y {
            quad_1_count += 1;
        }
        if new_x < mid_x && new_y > mid_y {
            quad_3_count += 1;
        }
        if new_x > mid_x && new_y < mid_y {
            quad_2_count += 1;
        }
        if new_x > mid_x && new_y > mid_y {
            quad_4_count += 1;
        }
    }

    (quad_1_count * quad_2_count * quad_3_count * quad_4_count).to_string()
}

fn print_tree(robot_positions: &HashSet<(usize, usize)>) {
    for y in 0..ROOM_HEIGHT as usize {
        for x in 0..ROOM_WIDTH as usize {
            if robot_positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

pub fn solve_part_two<'a>(input: Vec<Robot>) -> String {
    let mut robots = input;

    let mut second = 0;
    loop {
        second += 1;

        let mut robot_positions = HashSet::new();
        for robot in robots.iter_mut() {
            let new_x = ((robot.pos_x as i64) + robot.vel_x) % ROOM_WIDTH;
            robot.pos_x = if new_x < 0 { ROOM_WIDTH + new_x } else { new_x } as usize;

            let new_y = ((robot.pos_y as i64) + robot.vel_y) % ROOM_HEIGHT;
            robot.pos_y = if new_y < 0 {
                ROOM_HEIGHT + new_y
            } else {
                new_y
            } as usize;

            robot_positions.insert((robot.pos_x, robot.pos_y));
        }

        if robot_positions.len() == robots.len() {
            print_tree(&robot_positions);
            break;
        }
    }

    second.to_string()
}
