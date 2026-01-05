use std::fs::read_to_string;

pub fn part1() {
    let (map, mut carts) = parse("data/13");
    let init_len = carts.len();

    'main: loop {

        for i in 0..carts.len() {
            if carts[i].to_remove {
                continue;
            }

            carts[i].step(&map);

            for j in 0..carts.len() {
                if i != j
                    && !carts[j].to_remove
                    && carts[i].x == carts[j].x
                    && carts[i].y == carts[j].y
                {
                    carts[i].to_remove = true;
                    carts[j].to_remove = true;
                }
            }
        }

        let mut first: Option<Cart> = None;
        for c in carts.iter() {
            if c.to_remove {
                first = Some(*c);
                break;
            }
        }
        if let Some(first) = first {
            println!("found last: {},{}", first.x, first.y);
            break 'main;
        }
    }
}

pub fn part2() {
    let (map, mut carts) = parse("data/13");
    'main: loop {
        carts.sort_by_key(|c| (c.y, c.x));

        for i in 0..carts.len() {
            if carts[i].to_remove {
                continue;
            }

            carts[i].step(&map);

            for j in 0..carts.len() {
                if i != j
                    && !carts[j].to_remove
                    && carts[i].x == carts[j].x
                    && carts[i].y == carts[j].y
                {
                    carts[i].to_remove = true;
                    carts[j].to_remove = true;
                }
            }
        }

        carts.retain(|c| !c.to_remove);

        if carts.len() == 1 {
            let last = carts.last().unwrap();
            println!("found last: {},{}", last.x, last.y);
            break 'main;
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Cart {
    x: usize,
    y: usize,
    to_remove: bool,
    facing: Dir,
    turn_index: usize,
}
impl Dir {
    fn turn_cc(&self) -> Self {
        match *self {
            Dir::U => Dir::L,
            Dir::R => Dir::U,
            Dir::D => Dir::R,
            Dir::L => Dir::D,
        }
    }

    fn turn_c(&self) -> Self {
        match *self {
            Dir::U => Dir::R,
            Dir::R => Dir::D,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
        }
    }

    fn new_form_char(&self, c: char) -> Self {
        match c {
            '-' => {
                if *self == Dir::L || *self == Dir::R {
                    *self
                } else {
                    unreachable!("can olny go orizantal");
                }
            }
            '|' => {
                if *self == Dir::U || *self == Dir::D {
                    *self
                } else {
                    unreachable!("can only go vertical");
                }
            }
            '\\' => match *self {
                Dir::U => Dir::L,
                Dir::L => Dir::U,
                Dir::D => Dir::R,
                Dir::R => Dir::D,
            },
            '/' => match *self {
                Dir::U => Dir::R,
                Dir::L => Dir::D,
                Dir::D => Dir::L,
                Dir::R => Dir::U,
            },
            '+' => *self,
            _ => unreachable!("outside the path -{c}-"),
        }
    }
}
impl Cart {
    fn step(&mut self, map: &Vec<Vec<char>>) {
        self.cart_move();

        let symbol = map[self.y][self.x];
        if symbol == '+' {
            self.facing = match self.turn_index {
                0 => self.facing.turn_cc(),
                1 => self.facing,
                2 => self.facing.turn_c(),
                _ => unreachable!(),
            };
            self.turn_index += 1;
            self.turn_index %= 3;
        }

        self.facing = self.facing.new_form_char(symbol);
    }

    fn cart_move(&mut self) {
        match self.facing {
            Dir::U => self.y -= 1,
            Dir::D => self.y += 1,
            Dir::L => self.x -= 1,
            Dir::R => self.x += 1,
        }
    }
}

fn parse(path: &str) -> (Vec<Vec<char>>, Vec<Cart>) {
    let mut data: Vec<Vec<char>> = read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    let mut carts: Vec<Cart> = Vec::new();

    for (y, line) in data.iter_mut().enumerate() {
        for (x, c) in line.iter_mut().enumerate() {
            match c {
                '^' => {
                    carts.push(Cart {
                        x,
                        y,
                        to_remove: false,
                        facing: Dir::U,
                        turn_index: 0,
                    });
                    *c = '|';
                }
                'v' => {
                    carts.push(Cart {
                        x,
                        y,
                        to_remove: false,
                        facing: Dir::D,
                        turn_index: 0,
                    });
                    *c = '|';
                }
                '>' => {
                    carts.push(Cart {
                        x,
                        y,
                        to_remove: false,
                        facing: Dir::R,
                        turn_index: 0,
                    });
                    *c = '-';
                }
                '<' => {
                    carts.push(Cart {
                        x,
                        y,
                        to_remove: false,
                        facing: Dir::L,
                        turn_index: 0,
                    });
                    *c = '-';
                }
                _ => (),
            }
        }
    }
    (data, carts)
}
