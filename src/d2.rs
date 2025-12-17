use std::{fs::read_to_string};

pub fn part1() {
    let check_sum = read_to_string("data/2")
        .unwrap()
        .lines()
        .map(letter_frequency)
        .fold([0usize, 0usize], |acc, x| [acc[0] + x.0, acc[1] + x.1]);
    let check_sum = check_sum[0] * check_sum[1];
    println!("check_sum = {}", check_sum);
}

pub fn part2() {
    let data = read_to_string("data/2").unwrap();
    let ids: Vec<&str> = data
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();
    'outer:
    for a in ids.iter() {
        for b in ids.iter() {
            if letter_diffences(a, b) == 1 {
                println!("correct ids: ");
                println!("{a}");
                println!("{b}");
                break 'outer;
            }
        }
    }
}

fn letter_diffences(a: &str, b: &str) -> usize {
    let mut n = 0;
    for (c1, c2) in a.chars().zip(b.chars()) {
        if c1 != c2 {
            n += 1;
        } 
    }
    n
}

fn letter_frequency(line: &str) -> (usize, usize) {
    let mut n_of_2 = 0;
    let mut n_of_3 = 0;
    let mut letter_array = [0usize; 26];

    for i in line.trim().bytes().map(|x| x - b'a') {
        if i >= 0 && i < 26 {
            letter_array[i as usize] += 1;
        }
    }

    for i in letter_array.iter() {
        match i {
            2 => n_of_2 = 1,
            3 => n_of_3 = 1,
            _ => (),
        }
    }

    (n_of_2, n_of_3)
}

