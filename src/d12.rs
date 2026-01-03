use std::io::Write;
use std::{fs::read_to_string, mem::swap};

pub fn part1() {
    let (patterns, mut pots) = parse::<{ 200 + 20 * 4 }>("data/12");
    for _ in 0..20 {
        pots.step(&patterns)
    }
    println!("plants alive: {}", pots.count());
}

pub fn part2() {
    let (patterns, mut pots) = parse::<{ 5000 }>("data/12");
    let mut pop = 0;
    let mut speed = 0;
    let max_tunrs = 50000000000usize;
    let mut n_acc_zero = 0;
    for current_turn in 0..max_tunrs {
        pots.step(&patterns);
        let new_pop = pots.count() as i64;
        let new_speed = new_pop - pop;
        let acc = new_speed - speed;
        if acc == 0 {
            n_acc_zero += 1;
        } else {
            n_acc_zero = 0;
        }
        let estimate = new_pop + new_speed * (max_tunrs - current_turn - 1) as i64;
        print!("\rt:{current_turn} p:{pop} s:{new_speed} a:{acc} e:{estimate}          ");
        std::io::stdout().flush().unwrap();
        pop = new_pop;
        speed = new_speed; 
        if n_acc_zero == 1000 {
            break;
        }
    }
    println!("");
}

fn parse<const SIZE: usize>(path: &str) -> (Vec<(u8, bool)>, Pots<SIZE>) {
    let _data = read_to_string(path).unwrap();
    let lines: Vec<_> = _data
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .collect();
    let mut initial_state: &str = "";
    let mut ris = Vec::new();
    for l in lines {
        match l.as_slice() {
            ["initial", "state:", state] => initial_state = state,
            [pattern, "=>", plant_state] => {
                let mut p = 0u8;
                let b;

                for c in pattern.chars() {
                    p <<= 1;
                    if c == '#' {
                        p |= 1;
                    }
                }

                match *plant_state {
                    "#" => b = true,
                    "." => b = false,
                    _ => unreachable!("palent steate should be # . {plant_state}"),
                }
                if b {
                    ris.push((p, b));
                }
            }
            [] => (),
            _ => println!("cant parse: {l:?}"),
        }
    }
    if initial_state == "" {
        unreachable!("initial string should not be empty");
    }
    (ris, Pots::new(initial_state))
}

struct Pots<const SIZE: usize> {
    data: Vec<usize>,
    buffer: Vec<usize>,
}

impl<const SIZE: usize> Pots<SIZE> {
    fn new(line: &str) -> Pots<SIZE> {
        let mut pos = SIZE / 2;
        let data = vec![0usize; SIZE / 64 + 100];
        let buffer = vec![0usize; SIZE / 64 + 100];
        let mut pot = Pots { data, buffer };

        for c in line.chars() {
            match c {
                '#' => pot.set(pos),
                '.' => pot.clear(pos),
                _ => unreachable!("the string should only be # ."),
            }
            pos += 1;
        }
        swap(&mut pot.data, &mut pot.buffer);
        pot
    }

    fn step(&mut self, pattern: &Vec<(u8, bool)>) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = 0;
        }
        for i in 2..(SIZE - 2) {
            for (p, b) in pattern.iter() {
                if self.set_if(i, *p, *b) {
                    break;
                }
            }
        }
        swap(&mut self.data, &mut self.buffer);
    }

    fn set_if(&mut self, mut index: usize, pattern: u8, pant_next: bool) -> bool {
        assert!(index >= 2);
        index -= 2;
        let mut pattern_internal = 0;
        for di in 0..5 {
            pattern_internal <<= 1;
            pattern_internal += self.get(index + di);
        }
        index += 2;
        if pattern_internal == pattern {
            if pant_next {
                self.set(index);
            }
            return true;
        }
        false
    }

    fn get(&self, index: usize) -> u8 {
        let word = index / 64;
        let bit = index % 64;
        ((self.data[word] >> bit) & 1) as u8
    }

    fn set(&mut self, index: usize) {
        let word = index / 64;
        let bit = index % 64;
        self.buffer[word] |= 1 << bit;
    }

    fn clear(&mut self, index: usize) {
        let word = index / 64;
        let bit = index % 64;
        self.buffer[word] &= !(1 << bit);
    }

    fn count(&self) -> i64 {
        let mut sum = 0;
        for i in 0..SIZE {
            if self.get(i) > 0 {
                sum += i as i64 - SIZE as i64 / 2;
            }
        }
        sum
    }
}
