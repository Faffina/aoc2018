use std::{collections::HashMap, fs::read_to_string, usize};

pub fn part1() {
    let shifts = GuardShift::get_all();
    let mut grups: HashMap<usize, Vec<usize>> = HashMap::new();

    for s in shifts {
        grups.entry(s.id)
            .or_insert(Vec::new())
            .push(s.hours_sleep);
    }

    let mut grups: Vec<_> = grups.into_iter()
        .map(|(k, v)| {
            let v = v.iter().fold([0usize; 60], |mut acc, v| {
                for i in 0..60 {
                    acc[i] += (v >> i) & 1;
                }
                acc
            });
            let total: usize = v.iter().sum();
            let (most, _) = v.iter().enumerate().max_by_key(|(_,v)| **v).unwrap();
            (k, most, total)
        })
        .collect();

    grups.sort_by_key(|(k, m, t)| *t);

    let (k, m, _) = grups.last().unwrap();

    println!("found: {}", k*m)
}

pub fn part2() {
    let shifts = GuardShift::get_all();
    let mut grups: HashMap<usize, Vec<usize>> = HashMap::new();

    for s in shifts {
        grups.entry(s.id)
            .or_insert(Vec::new())
            .push(s.hours_sleep);
    }

    let mut grups: Vec<_> = grups.into_iter()
        .map(|(k, v)| {
            let v = v.iter().fold([0usize; 60], |mut acc, v| {
                for i in 0..60 {
                    acc[i] += (v >> i) & 1;
                }
                acc
            });
            let (most, freq) = v.iter().enumerate().max_by_key(|(_,v)| **v).unwrap();
            (k, most, *freq)
        })
        .collect();

    grups.sort_by_key(|(k, m, t)| *t);

    let (k, m, _) = grups.last().unwrap();

    println!("found: {}", k*m)
}

#[derive(Debug)]
enum Event {
    ShiftStart(usize),
    FallsAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Record {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    event: Event,
}

#[derive(Debug)]
struct GuardShift {
    id: usize,
    hours_sleep: usize,
}

impl GuardShift {
    fn get_all() -> Vec<GuardShift> {
        let mut records = parse("data/4");
        records.sort_by_key(|r| (r.year, r.month, r.day, r.hour, r.minute));
        let mut records = records.iter();
        let mut shifts = Vec::new();

        let first = records.nth(0).unwrap();
        let id = if let Event::ShiftStart(i) = first.event {
            i
        } else {
            panic!("first shuld be start shift");
        };

        let mut current_shift = GuardShift::new(id);
        let mut is_awake = true;
        let mut start_sleep = 0;
        for r in records {
            match r.event {
                Event::FallsAsleep => {
                    assert!(is_awake);
                    assert!(r.hour == 0);
                    start_sleep = r.minute;
                    is_awake = false;
                }
                Event::WakeUp => {
                    assert!(!is_awake);
                    assert!(r.hour == 0);
                    current_shift.set_range(start_sleep, r.minute);
                    is_awake = true;
                }
                Event::ShiftStart(id) => {
                    assert!(is_awake);
                    shifts.push(current_shift);
                    current_shift = GuardShift::new(id);
                }
            }
        }
        shifts
    }

    fn new(id: usize) -> GuardShift {
        Self { id, hours_sleep: 0 }
    }

    fn set_range(&mut self, start: usize, end: usize) {
        assert!(end < 60);
        assert!(start < 60);
        for i in start..end {
            self.set(i);
        }
    }

    fn set(&mut self, index: usize) {
        self.hours_sleep |= 1 << index;
    }

    fn get(&self, index: usize) -> bool {
        (self.hours_sleep & (1 << index)) > 0
    }
}

impl Event {
    fn new(line: &str) -> Option<Self>{
        let mut parts = line.trim().split(' ');
        match parts.nth(0)? {
            "falls" => Some(Self::FallsAsleep),
            "wakes" => Some(Self::WakeUp),
            "Guard" => {
                let id = parts.nth(0)?;
                let id = id.trim_start_matches('#').parse().ok()?;
                Some(Self::ShiftStart(id))
            }
            _ => None
        }
    }
}

impl Record {

    fn new(line: &str) -> Option<Self> {
        let line = line.trim_start_matches('[');
        let (date, line) = line.split_once(' ')?;
        let (year, date) = date.split_once('-')?;
        let (month, day) = date.split_once('-')?; 
        let (time, line) = line.split_once(']')?;
        let (hour, minute) = time.split_once(':')?;
        let event = Event::new(line)?;
        let year = year.trim().parse().ok()?;
        let month = month.trim().parse().ok()?;
        let day = day.trim().parse().ok()?;
        let hour = hour.trim().parse().ok()?;
        let minute = minute.trim().parse().ok()?;
        Some(Self { year, month, day, hour, minute, event })
    }
}

fn parse(path: &str) -> Vec<Record> {
    let r: Vec<Record> = read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(Record::new)
        .collect();
    assert_eq!(r.len(), 1186);
    r
} 
