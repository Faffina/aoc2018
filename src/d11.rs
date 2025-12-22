
pub fn part1() {
    const GRID_SIZE: usize = 300;
    const SERIAL: i64 = 9424;
    let mut grid = [[0i64; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let id = (x + 1) as i64 + 10;
            let mut power = id * (y + 1) as i64 + SERIAL;
            power *= id;
            power /= 100;
            power %= 10;
            power -= 5;
            grid[y][x] = power;
        }
    }
    let mut max = 0;
    let mut risx = 0;
    let mut risy = 0;
    for y in 0..(GRID_SIZE - 3) {
        for x in 0..(GRID_SIZE - 3) { 
            let mut total_power = 0;
            for dy in 0..3 {
                for dx in 0..3 {
                    total_power += grid[y + dy][x + dx];
                }
            }
            if total_power > max {
                max = total_power;
                risx = x + 1;
                risy = y + 1;
            }
        }
    }

    println!("fount best power at: {risx},{risy}");
}

pub fn part2() {
    const GRID_SIZE: usize = 300;
    const SERIAL: i64 = 9424;
    let mut grid = [[0i64; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let id = (x + 1) as i64 + 10;
            let mut power = id * (y + 1) as i64 + SERIAL;
            power *= id;
            power /= 100;
            power %= 10;
            power -= 5;
            grid[y][x] = power;
        }
    }
    let mut max = 0;
    let mut risx = 0;
    let mut risy = 0;
    let mut size = 0;
    for s in 1..GRID_SIZE {
        for y in 0..(GRID_SIZE - s + 1) {
            for x in 0..(GRID_SIZE - s + 1) { 
                let mut total_power = 0;
                for dy in 0..s {
                    for dx in 0..s {
                        total_power += grid[y + dy][x + dx];
                    }
                }
                if total_power > max {
                    max = total_power;
                    risx = x + 1;
                    risy = y + 1;
                    size = s;
                }
            }
        }
    }
    println!("fount best power at: {risx},{risy},{size}");
}
