use std::{collections::VecDeque, fs::read_to_string};


pub fn part1() {
    let mut data = parse();
    let head = Node::new(&mut data);
    let check_sum = head.sum();
    println!("{check_sum}")
}

pub fn part2() {
    let mut data = parse();
    let mut head = Node::new(&mut data);
    let check_sum = head.value();
    println!("{check_sum}")
}

fn parse() -> VecDeque<usize> {
    read_to_string("data/8")
        .unwrap()
        .split_whitespace()
        .filter_map(|x| Some(x.parse().ok()?))
        .collect()
}

struct Node {
    child: Vec<Box<Node>>,
    metadata: Vec<usize>,
    value: Option<usize>,
}

impl Node {
    fn new(data: &mut VecDeque<usize>) -> Self {
        let n_child = data.pop_front().unwrap();
        let n_metadata = data.pop_front().unwrap();
        let mut child = Vec::new();
        let mut metadata = Vec::new();
        for _ in 0..n_child {
            child.push(Box::new(Self::new(data)));
        }
        for _ in 0..n_metadata {
            metadata.push(data.pop_front().unwrap());
        }
        Self { child, metadata, value: None}
    }

    fn sum(&self) -> usize {
        let mut sum: usize = 0;
        sum += self.metadata.iter().sum::<usize>();
        sum += self.child.iter().map(|x| x.sum()).sum::<usize>();
        sum
    }

    fn value(&mut self) -> usize {
        if let Some(value) = self.value {
            return value;
        }

        if self.child.is_empty() {
            let value = self.metadata.iter().sum::<usize>();
            self.value = Some(value);
            return value;
        }

        let mut value = 0;
        for index in self.metadata.iter() {
            if *index == 0 {
                continue;
            }

            let index = *index - 1;
            if index >= self.child.len() {
                continue;
            }
            
            value += self.child[index].value();
        }
        self.value = Some(value);
        value
    }
}
