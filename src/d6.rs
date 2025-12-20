use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::{Add, Sub},
};

pub fn part1() {
    let mut points = parse();
    let (top, bottom) = found_corners(&mut points);
    let size = bottom - top;
    let mut areas = vec![vec![Some(0usize); size.x]; size.y];

    for y in 0..size.y {
        for x in 0..size.x {
            let current: Point = Point { x, y, id: 0 } + top;
            let mut distance = current.distance(points.first().unwrap());
            let mut id: Option<usize> = Some(points.first().unwrap().id);

            for p in &points[1..] {
                let current_distance = current.distance(p);
                if current_distance == distance {
                    id = None;
                } else if current_distance < distance {
                    distance = current_distance;
                    id = Some(p.id);
                }
            }

            areas[y][x] = id;
        }
    }


    let mut to_remove: HashSet<usize> = HashSet::new();

    for x in 0..size.x {
        if let Some(id) = areas[0][x] {
            to_remove.insert(id);
        }
        if let Some(id) = areas[size.y - 1][x] {
            to_remove.insert(id);
        }
    }

    for y in 0..size.y {
        if let Some(id) = areas[y][0] {
            to_remove.insert(id);
        }
        if let Some(id) = areas[y][size.x - 1] {
            to_remove.insert(id);
        }
    }

    for y in 0..size.y {
        for x in 0..size.x {
            if let Some(id) = areas[y][x] {
                if to_remove.contains(&id) {
                    areas[y][x] = None;
                }
            }
        }
    }


    let mut non_inf_areas: HashMap<usize, usize> = HashMap::new();

    for y in 0..size.y {
        for x in 0..size.x {
            if let Some(id) = areas[y][x] {
                *non_inf_areas.entry(id).or_insert(0) += 1;
            } 
        }
    }

    let mut final_areas: Vec<usize> = non_inf_areas.into_iter().map(|x| x.1).collect();
    final_areas.sort();
    let last = final_areas.last().unwrap();

    println!("part1 {last}");
}

pub fn part2() {
    let mut points = parse();
    let (top, bottom) = found_corners(&mut points);
    let size = bottom - top;
    let mut areas = vec![vec![false; size.x]; size.y];

    for y in 0..size.y {
        for x in 0..size.x {
            let current: Point = Point { x, y, id: 0 } + top;
            let mut sum = 0;
            for p in points.iter() {
                sum += current.distance(p);
            }
            areas[y][x] = sum < 10000;
        }
    }

    let area = areas.iter().flatten().filter(|x| **x).count();
    println!("fount the area: {}", area);
}

fn found_corners(list: &mut Vec<Point>) -> (Point, Point) {
    list.sort_by_key(|x| x.x);
    let minx = list.first().unwrap().x;
    let maxx = list.last().unwrap().x; 

    list.sort_by_key(|y| y.y);
    let miny = list.first().unwrap().y;
    let maxy = list.last().unwrap().y; 
    
    (Point{x:minx, y: miny, id: 0}, Point{x:maxx, y: maxy, id: 0})
}

fn parse() -> Vec<Point> {
    let data: Vec<_> = read_to_string("data/6")
        .unwrap()
        .lines()
        .enumerate()
        .filter_map(|(id, v)| {
            let (x, y) = v.split_once(", ")?;
            let x = x.parse().ok()?;
            let y = y.parse().ok()?;
            Some(Point { x, y, id })
        })
        .collect();
    assert!(data.len() == 50);
    data
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
    id: usize,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y, id: 0 }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self { x, y, id: 0 }
    }
}

impl Point {
    fn distance(&self, other: &Self) -> usize {
        (((self.x as i64) - (other.x as i64)).abs() + ((self.y as i64) - (other.y as i64)).abs())
            as usize
    }
}
