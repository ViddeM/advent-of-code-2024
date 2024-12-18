#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    /// Sets A to result of division between A and 2^COMBO_OP
    Adv,
    /// Bitwise XOR of B and LITERAL_OP
    Bxl,
    /// Sets B to COMBO_OP % 8
    Bst,
    /// If A == 0 then nothing else sets IP to LITERAL_OP. DO NOT INCREASE IP AFTERWARDS!
    Jnz,
    /// Sets B to the result of bitwise XOR between B and C. Ignores OP
    Bxc,
    /// Output COMBO_OP % 8
    Out,
    /// Same as ADV but store result in B
    Bdv,
    /// Same as Bdv but for C.
    Cdv,
}

pub struct Input {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<(Instruction, i64)>,
    raw_program: String,
}

pub fn parse<'a>(input: &str) -> Input {
    let (regs, program) = input.split_once("\n\n").unwrap();
    let mut regs = regs.lines();

    let reg_a = regs
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let reg_b = regs
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let reg_c = regs
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let raw_program = program
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .to_string();
    let program = raw_program
        .split(",")
        .array_chunks::<2>()
        .map(|[a, b]| {
            let ins = match a.chars().next().unwrap() {
                '0' => Instruction::Adv,
                '1' => Instruction::Bxl,
                '2' => Instruction::Bst,
                '3' => Instruction::Jnz,
                '4' => Instruction::Bxc,
                '5' => Instruction::Out,
                '6' => Instruction::Bdv,
                '7' => Instruction::Cdv,
                c => panic!("Invalid OP_CODE {c}"),
            };
            let num = b.parse::<i64>().unwrap();
            (ins, num)
        })
        .collect::<Vec<_>>();

    Input {
        reg_a,
        reg_b,
        reg_c,
        program,
        raw_program,
    }
}

fn combo(num: &i64, a: &i64, b: &i64, c: &i64) -> i64 {
    match num {
        0_i64..=3_i64 => *num,
        4 => *a,
        5 => *b,
        6 => *c,
        7 => panic!("Reserved combo num used!"),
        n => panic!("Invalid combo num '{n}' received"),
    }
}

fn run_program(
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: &Vec<(Instruction, i64)>,
) -> Vec<String> {
    let mut reg_a = reg_a;
    let mut reg_b = reg_b;
    let mut reg_c = reg_c;

    let mut ip = 0;
    let mut outputs = vec![];

    while ip < (program.len() * 2) {
        if ip % 2 != 0 {
            panic!("Instruction pointer uneven!");
        }
        let i = ip / 2;
        let (ins, num) = program.get(i).as_ref().expect("Getting instruction/num");
        // println!("IP {ip} INS {ins:?} NUM {num} REGS {reg_a} :: {reg_b} :: {reg_c} ");

        match ins {
            Instruction::Adv => {
                let n = combo(num, &reg_a, &reg_b, &reg_c) as u32;
                let denom = 2i64.pow(n);
                reg_a = reg_a.div_floor(denom);
            }
            Instruction::Bxl => {
                reg_b = reg_b ^ num;
            }
            Instruction::Bst => {
                let n = combo(num, &reg_a, &reg_b, &reg_c);
                reg_b = n % 8;
            }
            Instruction::Jnz => {
                if reg_a != 0 {
                    ip = *num as usize;
                }
            }
            Instruction::Bxc => {
                reg_b = reg_b ^ reg_c;
            }
            Instruction::Out => {
                let n = combo(num, &reg_a, &reg_b, &reg_c);
                let v = n % 8;
                outputs.push(v.to_string());
            }
            Instruction::Bdv => {
                let n = combo(num, &reg_a, &reg_b, &reg_c) as u32;
                let denom = 2i64.pow(n);
                reg_b = reg_a.div_floor(denom);
            }
            Instruction::Cdv => {
                let n = combo(num, &reg_a, &reg_b, &reg_c) as u32;
                let denom = 2i64.pow(n);
                reg_c = reg_a.div_floor(denom);
            }
        }

        if !(ins == &Instruction::Jnz && reg_a != 0) {
            ip += 2;
        }
    }

    outputs
}

pub fn solve_part_one<'a>(input: Input) -> String {
    run_program(input.reg_a, input.reg_b, input.reg_c, &input.program).join(",")
}

pub fn solve_part_two<'a>(input: Input) -> String {
    let raw_program_nums = input
        .raw_program
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let mut facts = vec![0; raw_program_nums.len()];

    let ans = loop {
        let mut ia = 0;
        for (i, f) in facts.iter().enumerate() {
            ia += 8i64.pow(i as u32) * f;
        }

        let output = run_program(ia, 0, 0, &input.program);

        if output == raw_program_nums {
            break ia;
        }

        for i in (0..raw_program_nums.len()).rev() {
            if output.len() < i || output[i] != raw_program_nums[i] {
                facts[i] += 1;
                break;
            }
        }
    };

    ans.to_string()
}
