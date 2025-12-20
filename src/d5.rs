use std::fs::read_to_string;

pub fn part1() {
    let mut polymer = parse();
    while reduce(&mut polymer) {}
    let len = polymer.len();
    println!("the final lenght: {}", len);
}

pub fn part2() {
    let mut polymer = parse();
    let mut shotes_polymer = polymer.len();

    for filter_mol in 'a'..='z' {
        let test_mol = filter_mol as u8 - b'a';
        let mut test_polymer: Vec<_> = polymer
            .iter()
            .copied()
            .filter(|x| x.0 != test_mol)
            .collect();
        while reduce(&mut test_polymer) {}
        let len = test_polymer.len();

        if shotes_polymer > len {
            shotes_polymer = len;
        }
    }

    println!("the shotest is {}", shotes_polymer);
}

fn reduce(polymer: &mut Vec<(u8, bool)>) -> bool {
    let mut reactiond_happen = false;

    let mut i: usize = 0;
    while (i + 1) < polymer.len() {
        let mol1 = polymer[i];
        let mol2 = polymer[i + 1];
        if mol1.0 == mol2.0 && mol1.1 != mol2.1 {
            polymer.drain(i..=(i + 1));
            reactiond_happen = true;
        } else {
            i += 1;
        }
    }

    reactiond_happen
}

fn parse() -> Vec<(u8, bool)> {
    read_to_string("data/5")
        .unwrap()
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                let sin = c.is_uppercase();
                let c = c.to_ascii_lowercase();
                let id = c as u8 - b'a';
                Some((id, sin))
            } else {
                None
            }
        })
        .collect()
}
