use std::{fs::read_to_string, io::Write, ops::{Add, Sub}};

pub fn part1() {
    let mut points = parse();
    let (mut top, mut size) = find_bounding_box(&mut points);
    let mut time = 0;

    while size.y > 9 {
        for p in points.iter_mut() {
            p.apply();
        }
        (top, size) = find_bounding_box(&mut points);
        time += 1;
    } 
    println!("\rtime saved: {time}");

    let mut draw = vec![vec!['.';size.x as usize + 1];size.y as usize + 1];

    for p in points.iter() {
        let rp = p.position - top;
        draw[rp.y as usize][rp.x as usize] = '#';
    }

    let draw: Vec<String> = draw.iter().map(|x| x.iter().collect()).collect();
    for l in draw {
        println!("{l}");
    }
}

pub fn part2() {

}

fn find_bounding_box(points: &mut Vec<Point>) -> (Vec2, Vec2) {
    let mut min = Vec2 { x: i64::MAX, y: i64::MAX };
    let mut max = Vec2 { x: i64::MIN, y: i64::MIN };

    for p in points {
        min.x = min.x.min(p.position.x);
        max.x = max.x.max(p.position.x);

        min.y = min.y.min(p.position.y);
        max.y = max.y.max(p.position.y);
    }

    let size = max - min;
    (min, size)
}

fn parse() -> Vec<Point> {
    let data:Vec<Point> = read_to_string("data/10")
        .unwrap()
        .lines()
        .filter_map(Point::new)
        .collect();
    assert!(data.len() == 359);
    data
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    position: Vec2,
    velocity: Vec2,
}

impl Point {
    fn new(line: &str) -> Option<Self>{ 
        let (_, line) = line.split_once('<')?;
        let (px, line) = line.split_once(',')?;
        let (py, line) = line.split_once('>')?;
        let (_, line) = line.split_once('<')?;
        let (vx, vy) = line.split_once(',')?;
        let px = px.trim().parse::<i64>().ok()?;
        let py = py.trim().parse::<i64>().ok()?;
        let vx = vx.trim().parse::<i64>().ok()?;
        let vy = vy.trim_end_matches('>').trim().parse::<i64>().ok()?;
        Some(Self { position: Vec2 { x: px, y: py }, velocity: Vec2 { x: vx, y: vy } })
    }

    fn apply(&mut self) {
        self.position = self.position + self.velocity;
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}
