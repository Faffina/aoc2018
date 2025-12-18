use std::fs::read_to_string;


pub fn part1() {
    let all_claim = Claim::get_all("data/3");
    let mut fabric: Fabric<1000> = Fabric::new();

    for claim in all_claim.iter() {
        fabric.compose(claim);
    }

    println!("number of overlaps = {}", fabric.count_overlap());

    for claim in all_claim.iter() {
        if let Some(id) = fabric.compare(claim) {
            println!("id for non overlaping = {}", id);
            break;
        }
    }
}

pub fn part2() {

}

struct Fabric<const SIZE: usize> {
    gird: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Claim {
    id: usize,
    y: usize,
    x: usize,
    w: usize,
    h: usize,
}

impl<const SIZE: usize> Fabric<SIZE> {
    fn new() -> Self {
        let gird = vec![vec![0usize; SIZE]; SIZE];
        Self { gird }
    }

    fn compare(&self, claim: &Claim) -> Option<usize> {
        for y in claim.y..(claim.y + claim.h){
            for x in claim.x..(claim.x + claim.w){
                if self.gird[y][x] != 1 {
                    return None
                }
            }
        }
        Some(claim.id)
    }

    fn compose(&mut self, claim: &Claim) {
        for y in claim.y..(claim.y + claim.h){
            for x in claim.x..(claim.x + claim.w){
                self.gird[y][x] += 1;
            }
        }
    }

    fn count_overlap(&self) -> usize{
        let mut overlaps = 0;
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.gird[y][x] >= 2 {
                    overlaps += 1;
                }
            }
        }
        overlaps
    } 
}

impl Claim {
    fn new(line: &str) -> Option<Self> {
        let (id, line) = line.split_once('@')?;
        let (x, line) = line.split_once(',')?;
        let (y, line) = line.split_once(':')?;
        let (w, h) = line.split_once('x')?;
        let id = id.trim_start_matches('#').trim().parse().ok()?;
        let y = y.trim().parse().ok()?;
        let x = x.trim().parse().ok()?;
        let w = w.trim().parse().ok()?;
        let h = h.trim().parse().ok()?;
        Some(Self{id, y, x, w, h})
    }

    fn get_all(path: &str) -> Vec<Self> {
        let data: Vec<Claim> = read_to_string(path)
            .unwrap()
            .lines()
            .filter_map(Claim::new)
            .collect();
        assert!(data.len() == 1301, "len is not 1300: {}", data.len());
        data
    }
}
