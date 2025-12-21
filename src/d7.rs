use std::{collections::HashSet, fs::read_to_string};


pub fn part1() {
    let mut steps = parse();

    while steps.len() != 1 {
        let mut aviable = find_aviable(&steps);
        aviable.sort();
        let new_letter = *aviable.first().unwrap();
        print!("{new_letter}");
        remove_letter(&mut steps, new_letter);
    }
    let (first, then) = steps.first().unwrap();
    println!("{first}{then}");
}


pub fn part2() {
    let mut jobs = Job::new();
    let mut time: usize = 0;
    let mut working: usize = 0;
    jobs.sort_by_key(|x| x.id);

    while !jobs.is_empty() {
        if working < 5 {
            for i in 0..jobs.len() {
                if jobs[i].is_aviable(&jobs) {
                    jobs[i].start();
                    working += 1;
                }
            }
        }
        
        let mut i = 0;
        'outer:
        while i < jobs.len() {
            if jobs[i].in_progress {
                if let Some(c) = jobs[i].working() {
                    working -= 1;
                    jobs.remove(i);
                    continue 'outer;
                }
            }
            i += 1;
        }
        time += 1;
    }
    println!("{time}");
}

#[derive(Debug)]
struct Job {
    id: char,
    time: usize,
    in_progress: bool,
    dep: Vec<char>,
}

impl Job {
    fn new() -> Vec<Job> {
        let list = parse();
        let ids: HashSet<char> = list.iter().map(|x| [x.0, x.1]).flatten().collect();
        let mut jobs: Vec<Job> = Vec::new();
        for id in ids.iter() {
            let mut dep = Vec::new();
            for (fist, then) in list.iter() {
                if *then == *id {
                    dep.push(*fist);
                }
            }
            jobs.push(Job { id: *id, time: 0, in_progress: false, dep });
        }
        jobs
    }

    fn start(&mut self) {
        self.in_progress = true;
        self.time = 60 + (self.id as u8 - b'A') as usize;
    }

    fn working(&mut self) -> Option<char> {
        if self.time == 0 {
            return Some(self.id);
        }
        self.time -= 1;
        None
    }

    fn is_aviable(&self, jobs: &Vec<Job>) -> bool {
        if self.in_progress {
            return false;
        }

        for d in self.dep.iter() {
            for j in jobs {
                if j.id == *d {
                    return false;
                }
            }
        }

        true
    }
}

fn remove_letter(list: &mut Vec<(char, char)>, to_remove: char) {
    let mut i = 0;
    while i < list.len() {
        if list[i].0 == to_remove {
            list.swap_remove(i);
            continue;
        }
        i += 1;
    }
}

fn find_aviable(steps: &Vec<(char, char)>) -> Vec<char> {
    let mut list:Vec<char> = steps.iter().map(|x| x.0).collect();

    let mut i = 0;
    'outer: 
    while i < list.len() {
        for (_, then) in steps.iter() {
            if list[i] == *then {
                list.swap_remove(i);
                continue 'outer;
            }
        } 
        i += 1;
    }

    list
}

fn parse() -> Vec<(char, char)> {
    let data: Vec<(char, char)> = read_to_string("data/7")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.nth(1)?;
            let then = parts.nth(5)?;
            let first = first.chars().nth(0)?;
            let then = then.chars().nth(0)?;
            Some((first, then))
        })
        .collect();
    assert!(data.len() == 101);
    data
}
