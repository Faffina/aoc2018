use std::collections::VecDeque;
use std::io::Write;


pub fn part1() {
    const NUMBER_PLAYER: usize = 419;
    const MAX_MARBLE: usize = 71052;
    
    let mut marble_circle = MarbleCircle::new();
    let mut player = [0usize; NUMBER_PLAYER];
    let mut current_plaing = 0;
    
    marble_circle.data.push_front(0);

    for marble in 1..=MAX_MARBLE {
        player[current_plaing] += marble_circle.insert(marble);
        current_plaing = (current_plaing + 1) % NUMBER_PLAYER;
    }

    let winnig_score = player.iter().max().unwrap();
    println!("winnig score: {winnig_score}");
}

pub fn part2() {
    const NUMBER_PLAYER: usize = 419;
    const MAX_MARBLE: usize = 71052 * 100;
    const MARBLE_100NTH: usize = MAX_MARBLE / 100;
    
    let mut marble_circle = MarbleCircle::new();
    let mut player = [0usize; NUMBER_PLAYER];
    let mut current_plaing = 0;
    
    marble_circle.data.reserve(MAX_MARBLE);
    marble_circle.data.push_front(0);

    for marble in 1..=MAX_MARBLE {
        if marble % MARBLE_100NTH == 0 {
            print!("\r{}%", marble * 100 / MAX_MARBLE);
            std::io::stdout().flush().unwrap();
        }
        player[current_plaing] += marble_circle.insert(marble);
        current_plaing = (current_plaing + 1) % NUMBER_PLAYER;
    }

    let winnig_score = player.iter().max().unwrap();
    println!("\rwinnig score: {winnig_score}");
}

struct MarbleCircle {
    data: VecDeque<usize>,
    position: usize,
}

impl MarbleCircle {
    fn new() -> Self {
        Self { data: VecDeque::new(), position: 0 }
    }

    fn insert(&mut self, marble: usize) -> usize {
        if marble % 23 == 0 {
            self.data.rotate_right(7);
            self.data.pop_front().unwrap() + marble
        } else {
            if self.data.len() >= 2 {
                self.data.rotate_left(2);
            }
            self.data.push_front(marble);
            0
        }
    }
}
