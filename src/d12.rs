use std::collections::{HashSet};
use std::{fs::read_to_string, mem::swap};

pub fn part1() {
    let mut tunel = Tunel::new("data/12");

    for _ in 0..20 {
        tunel.step();
    }

    println!("part 1: {}", tunel.count());
}

pub fn part2() {
    let mut tunel = Tunel::new("data/12");

    let mut last_count = tunel.count();
    let mut last_speed = 0;
    let target = 50000000000i64;
    let mut n_turns_stable = 0;
    for i in 1..target {
        tunel.step();
        let count = tunel.count();
        let speed = count - last_count;
        let acc = speed - last_speed;
        let estimate = count + speed*(target - i);

        print!("\ri:{i}, c:{count}, s:{speed}, a:{acc}, ris:{estimate}                         ");

        if n_turns_stable == 2000 {
            break;
        }
        if acc == 0 {
            n_turns_stable += 1;
        } else {
            n_turns_stable = 0;
        }

        last_count = count;
        last_speed = speed;
    }
    println!("")
}

#[derive(Debug)]
struct Tunel {
    old_plants: HashSet<i64>,
    new_plants: HashSet<i64>,
    rules: HashSet<u8>,
}

impl Tunel {
    fn step(&mut self) {
        let min = self.old_plants.iter().min().unwrap() - 2;
        let max = self.old_plants.iter().max().unwrap() + 2;

        for i in min..=max {
            'rule:
            for r in self.rules.iter() {
                for j in 0..5 {
                    let rule_alive = (r >> j) & 1 == 1;
                    if rule_alive != self.old_plants.contains(&(i - 2 + j)) {
                        continue 'rule;
                    }
                }
                self.new_plants.insert(i);
                break 'rule;
            }
        }

        swap(&mut self.new_plants, &mut self.old_plants);
        self.new_plants.clear();
    }

    fn count(&self) -> i64 {
        self.old_plants.iter().sum()
    }

    fn new(path: &str) -> Self {
        let input_data = read_to_string(path).unwrap();
        let mut new_tunel = Tunel {
            old_plants: HashSet::new(),
            new_plants: HashSet::new(),
            rules: HashSet::new(),
        };

        for line in input_data.lines() {
            if line.is_empty() {
                continue;
            }
            let line: Vec<_> = line.split_whitespace().collect();
            match line.as_slice() {
                ["initial", "state:", state] => {
                    let mut i = 0;
                    for c in state.chars() {
                        match c {
                            '#' => {new_tunel.old_plants.insert(i);},
                            '.' => (),
                            _ => unreachable!("shuld not be any char other than # ."), 
                        }
                        i += 1;
                    }
                },

                [pattern, "=>", "#"] => {
                    let mut p = 0;
                    for c in pattern.chars().rev() {
                        p <<= 1;
                        match c {
                            '#' => p += 1,
                            '.' => (),
                            _ => unreachable!("shuld not be any char other than # ."), 
                        }
                    }

                    new_tunel.rules.insert(p);
                },

                [_, "=>", "."] => (),
                _ => println!("could not parse: {line:?}")
            }
        }
        new_tunel
    }
}
