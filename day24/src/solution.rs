use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Wire {
    Val(bool),
    Or(String, String),
    And(String, String),
    Xor(String, String),
}

impl Wire {
    pub fn get_inputs(&self) -> Option<(&String, &String)> {
        Some(match self {
            Wire::Val(_) => return None,
            Wire::Or(a, b) => (a, b),
            Wire::And(a, b) => (a, b),
            Wire::Xor(a, b) => (a, b),
        })
    }

    pub fn depends_on(&self, wire: &String) -> bool {
        if let Some((a, b)) = self.get_inputs() {
            a == wire || b == wire
        } else {
            false
        }
    }

    pub fn has_input_of_type(&self, c: char) -> bool {
        if let Some((a, b)) = self.get_inputs() {
            a.starts_with(c) || b.starts_with(c)
        } else {
            false
        }
    }
}

pub fn parse<'a>(input: &str) -> HashMap<String, Wire> {
    let (inits, conns) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();

    for wire in inits.lines() {
        let (w, v) = wire.split_once(": ").unwrap();
        let v = match v {
            "1" => true,
            "0" => false,
            _ => panic!("Invalid value {v}"),
        };
        wires.insert(w.to_string(), Wire::Val(v));
    }

    for conn in conns.lines() {
        let (inps, out) = conn.split_once(" -> ").unwrap();

        if let Some((a, b)) = inps.split_once(" AND ") {
            wires.insert(out.to_string(), Wire::And(a.into(), b.into()));
        } else if let Some((a, b)) = inps.split_once(" OR ") {
            wires.insert(out.to_string(), Wire::Or(a.into(), b.into()));
        } else {
            let (a, b) = inps.split_once(" XOR ").unwrap();
            wires.insert(out.to_string(), Wire::Xor(a.into(), b.into()));
        }
    }

    wires
}

fn solve_for(
    map: &mut HashMap<String, bool>,
    wires: &HashMap<String, Wire>,
    wire: &String,
) -> bool {
    if let Some(v) = map.get(wire) {
        return *v;
    }

    match wires.get(wire).expect("Wire not in map?") {
        Wire::And(a, b) => {
            let a = solve_for(map, wires, a);
            let b = solve_for(map, wires, b);
            let c = a && b;
            map.insert(wire.clone(), c);
            return c;
        }
        Wire::Or(a, b) => {
            let a = solve_for(map, wires, a);
            let b = solve_for(map, wires, b);
            let c = a || b;
            map.insert(wire.clone(), c);
            return c;
        }
        Wire::Xor(a, b) => {
            let a = if solve_for(map, wires, a) { 1 } else { 0 };
            let b = if solve_for(map, wires, b) { 1 } else { 0 };
            let c = if (a ^ b) == 1 { true } else { false };
            map.insert(wire.clone(), c);
            return c;
        }
        w => panic!("This should not happen! {w:?}"),
    }
}

fn solve(input: &HashMap<String, Wire>) -> u128 {
    let zs = input
        .keys()
        .filter(|k| k.starts_with("z"))
        .map(|p| p.to_string())
        .collect::<Vec<_>>();

    let mut vals = HashMap::new();
    for (k, v) in input.iter() {
        if let &Wire::Val(v) = v {
            vals.insert(k.into(), v);
        }
    }

    let mut number = 0;
    for z in zs.iter() {
        let bs = solve_for(&mut vals, &input, z);

        if !bs {
            continue;
        }

        let steps = z.strip_prefix("z").unwrap().parse::<u128>().unwrap();
        let c = 1u128 << steps;
        number |= c;
    }

    number
}

pub fn solve_part_one<'a>(input: HashMap<String, Wire>) -> String {
    solve(&input).to_string()
}

pub fn solve_part_two<'a>(input: HashMap<String, Wire>) -> String {
    let n_input_bits = input
        .iter()
        .filter(|(_, op)| matches!(op, Wire::Val(_)))
        .count()
        / 2;

    let mut incorrect: HashSet<String> = HashSet::new();

    let full_adder_gate_0s = input
        .iter()
        .filter(|(_, op)| op.has_input_of_type('x'))
        .filter_map(|(c, op)| match op {
            Wire::Xor(a, b) => Some((a, b, c)),
            _ => None,
        })
        .collect::<Vec<_>>();

    for (a, b, c) in full_adder_gate_0s.iter() {
        let is_first = a.as_str() == "x00" || b.as_str() == "x00";
        if is_first {
            if c.as_str() != "z00" {
                incorrect.insert(c.to_string());
            }
            continue;
        } else if c.as_str() == "z00" {
            incorrect.insert(c.to_string());
        }

        if c.starts_with('z') {
            incorrect.insert(c.to_string());
        }
    }

    let full_adder_gate_3s = input
        .iter()
        .filter(|(_, op)| match op {
            Wire::Xor(_, _) => true,
            _ => false,
        })
        .filter(|(_, op)| !op.has_input_of_type('x'))
        .collect::<Vec<_>>();
    for (c, _) in full_adder_gate_3s.iter() {
        if !c.starts_with('z') {
            incorrect.insert(c.to_string());
        }
    }

    let outputs = input
        .iter()
        .filter(|(c, _)| c.starts_with('z'))
        .collect::<Vec<_>>();
    let last = format!("z{n_input_bits:02}");
    for (c, w) in outputs.iter() {
        if c == &&last {
            if !matches!(w, Wire::Or(_, _)) {
                incorrect.insert(c.to_string());
            }
            continue;
        }

        if !matches!(w, Wire::Xor(_, _)) {
            incorrect.insert(c.to_string());
        }
    }

    let mut to_check = vec![];
    for (a, b, c) in full_adder_gate_0s.iter() {
        if incorrect.contains(*c) {
            continue;
        }

        if c.as_str() == "z00" {
            continue;
        }

        if full_adder_gate_3s
            .iter()
            .find(|(_, w)| w.depends_on(c))
            .is_none()
        {
            to_check.push((a, b, c));
            incorrect.insert(c.to_string());
        }
    }

    for (a, _, _) in to_check.iter() {
        let expected = format!("z{}", a.chars().skip(1).collect::<String>());

        let m = full_adder_gate_3s
            .iter()
            .filter(|(c, _)| c == &&expected)
            .collect::<Vec<_>>();

        if m.len() != 1 {
            panic!("Invalid matches {}, expected exactly 1", m.len());
        }

        let (_, mw) = m[0];

        let ors = input
            .iter()
            .filter(|(_, op)| matches!(op, Wire::Or(_, _)))
            .filter(|(o, _)| mw.depends_on(o))
            .collect::<Vec<_>>();

        if ors.len() != 1 {
            panic!("Invalid or matches {}, expected exactly 1", ors.len());
        }

        let (ors_a, ors_b) = mw.get_inputs().unwrap();
        if ors_a != ors[0].0 {
            incorrect.insert(ors_a.to_string());
        } else if ors_b != ors[0].0 {
            incorrect.insert(ors_b.to_string());
        } else {
            panic!("Expected one of the or outputs to match?");
        }
    }

    // Validate, convert to output.

    if incorrect.len() != 8 {
        panic!(
            "Unable to solve, got {} bad connections, expected 8",
            incorrect.len()
        )
    }

    let mut flags = incorrect.into_iter().collect::<Vec<_>>();
    flags.sort();
    flags.join(",")
}
