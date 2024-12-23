use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub fn parse<'a>(input: &'a str) -> Vec<(&'a str, &'a str)> {
    input.lines().map(|l| l.split_once("-").unwrap()).collect()
}

#[derive(Debug, Eq, Ord)]
struct Group<'a>(&'a str, &'a str, &'a str);

impl<'a> PartialEq for Group<'a> {
    fn eq(&self, rhs: &Self) -> bool {
        (self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2)
            || (self.0 == rhs.0 && self.1 == rhs.2 && self.2 == rhs.1)
            || (self.0 == rhs.1 && self.1 == rhs.2 && self.2 == rhs.0)
            || (self.0 == rhs.1 && self.1 == rhs.0 && self.2 == rhs.2)
            || (self.0 == rhs.2 && self.1 == rhs.0 && self.2 == rhs.1)
            || (self.0 == rhs.2 && self.1 == rhs.1 && self.2 == rhs.0)
    }
}

impl<'a> Hash for Group<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut v = vec![&self.0, &self.1, &self.2];
        v.sort();
        v[0].hash(state);
        v[1].hash(state);
        v[2].hash(state);
    }
}

impl<'a> PartialOrd for Group<'a> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(if self.0 < rhs.0 {
            Ordering::Less
        } else if self.0 > rhs.0 {
            Ordering::Greater
        } else {
            if self.1 < rhs.1 {
                Ordering::Less
            } else if self.1 > rhs.1 {
                Ordering::Greater
            } else {
                if self.2 < rhs.2 {
                    Ordering::Less
                } else if self.2 > rhs.2 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        })
    }
}

pub fn solve_part_one<'a>(input: Vec<(&str, &str)>) -> String {
    let mut connected: HashMap<&str, Vec<&str>> = HashMap::new();

    for (a, b) in input.into_iter() {
        if let Some(s) = connected.get_mut(a) {
            s.push(b);
        } else {
            connected.insert(a, vec![b]);
        }

        if let Some(s) = connected.get_mut(b) {
            s.push(a);
        } else {
            connected.insert(b, vec![a]);
        }
    }

    let mut groups = HashSet::new();

    for (k, ks) in connected.iter().filter(|(k, _)| k.starts_with('t')) {
        if ks.len() < 2 {
            continue;
        }

        for t in ks.iter() {
            for y in connected.get(t).unwrap() {
                if ks.contains(y) {
                    groups.insert(Group(k, t, y));
                }
            }
        }
    }

    groups.len().to_string()
}

pub fn solve_part_two<'a>(input: Vec<(&str, &str)>) -> String {
    let mut connected: HashMap<&str, Vec<&str>> = HashMap::new();

    for (a, b) in input.into_iter() {
        if let Some(s) = connected.get_mut(a) {
            s.push(b);
        } else {
            connected.insert(a, vec![b]);
        }

        if let Some(s) = connected.get_mut(b) {
            s.push(a);
        } else {
            connected.insert(b, vec![a]);
        }
    }

    let mut visited = HashSet::new();
    let mut graph = HashSet::new();
    let mut best = HashSet::new();

    for (k, ks) in connected.iter() {
        if visited.contains(k) {
            continue;
        }

        graph.clear();
        graph.insert(k);

        for t in ks.iter() {
            let ts = connected.get(t).unwrap();
            if graph.iter().all(|y| ts.contains(y)) {
                visited.insert(t);
                graph.insert(t);
            }
        }

        if graph.len() > best.len() {
            best = graph.clone();
        }
    }

    let mut best = best.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    best.sort_unstable();

    best.join(",")
}
