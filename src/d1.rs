use std::{collections::HashSet, fs::read_to_string};

pub fn part1() {
    let frequency: i64 = parse().iter().sum();
    println!("frequency = {}", frequency);
}

pub fn part2() {
    let mut frequency: i64 = 0;
    let mut set: HashSet<i64> = HashSet::new();

    for df in parse().iter().cycle() {
        if !set.insert(frequency) {
            println!("first frequency = {}", frequency );
            break;
        }
        frequency += df;
    }
}

fn parse() -> Vec<i64> {
    read_to_string("data/1")
        .unwrap()
        .lines()
        .filter_map(|line| Some(line.parse().ok()?))
        .collect()
}
