use std::cmp::Reverse;
use std::time::Instant;

use gcd::Gcd;
use priority_queue::PriorityQueue;

type Data = Vec<Vec<Vec<bool>>>;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Blizzard {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

impl Blizzard {
    pub fn position_at(&self, width: usize, height: usize, step: usize) -> (usize, usize) {
        match self.direction {
            Direction::Up => {
                let new_y = self.y as isize - step as isize;
                (self.x, new_y.rem_euclid(height as isize) as usize)
            }
            Direction::Down => {
                (self.x, (self.y + step) % height)
            }
            Direction::Left => {
                let new_x = self.x as isize - step as isize;
                (new_x.rem_euclid(width as isize) as usize, self.y)
            }
            Direction::Right => {
                ((self.x + step) % width, self.y)
            }
        }
    }

    pub fn will_intersect(&self, x: usize, y: usize) -> bool {
        match self.direction {
            Direction::Up | Direction::Down => self.x.abs_diff(x) <= 1,
            Direction::Left | Direction::Right => self.y.abs_diff(y) <= 1,
        }
    }
}

pub fn run(data: Data) -> usize {
    let width = data[0][0].len();
    let height = data[0].len();

    let mut priority_queue = PriorityQueue::new();

    for i in 0..data.len() {
        if !data[i][0][0] {
            priority_queue.push((0, 0, i), Reverse(width - 1 + height - 1 + i));
        }
    }
    while let Some(((x, y, time), distance)) = priority_queue.pop() {
        if x == width - 1 && y == height - 1 {
            return time + 1;
        }

        let new_time = (time + 1) % data.len();

        if !data[new_time][y][x] {
            priority_queue.push((x, y, time + 1), Reverse(distance.0 + 1));
        }

        if y < height - 1 && !data[new_time][y + 1][x] {
            priority_queue.push((x, y + 1, time + 1), Reverse(distance.0));
        }

        if y > 0 && !data[new_time][y - 1][x] {
            priority_queue.push((x, y - 1, time + 1), Reverse(distance.0 + 2));
        }

        if x < width - 1 && !data[new_time][y][x + 1] {
            priority_queue.push((x + 1, y, time + 1), Reverse(distance.0));
        }

        if x > 0 && !data[new_time][y][x - 1] {
            priority_queue.push((x - 1, y, time + 1), Reverse(distance.0 + 2));
        }
    }
    panic!("No solution found");
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let width = data[0][0].len();
    let height = data[0].len();
    let total_time = data.len();

    let mut priority_queue = PriorityQueue::new();

    for i in 0..data.len() {
        if !data[i][0][0] {
            priority_queue.push((0, 0, i), Reverse(width - 1 + height - 1 + i));
        }
    }

    let mut valid = false;
    let mut first_time_at_end = 0;
    while let Some(((x, y, time), distance)) = priority_queue.pop() {
        if x == width - 1 && y == height - 1 {
            first_time_at_end = time + 1;
            valid = true;
            break;
        }

        let new_time = (time + 1) % total_time;

        if !data[new_time][y][x] {
            priority_queue.push((x, y, time + 1), Reverse(distance.0 + 1));
        }

        if y < height - 1 && !data[new_time][y + 1][x] {
            priority_queue.push((x, y + 1, time + 1), Reverse(distance.0));
        }

        if y > 0 && !data[new_time][y - 1][x] {
            priority_queue.push((x, y - 1, time + 1), Reverse(distance.0 + 2));
        }

        if x < width - 1 && !data[new_time][y][x + 1] {
            priority_queue.push((x + 1, y, time + 1), Reverse(distance.0));
        }

        if x > 0 && !data[new_time][y][x - 1] {
            priority_queue.push((x - 1, y, time + 1), Reverse(distance.0 + 2));
        }
    }
    if !valid {
        panic!("No solution found");
    }

    let mut priority_queue = PriorityQueue::new();
    for i in 0..data.len() {
        if !data[(i + first_time_at_end + 1) % total_time][height - 1][width - 1] {
            priority_queue.push((width - 1, height - 1, i + first_time_at_end + 1), Reverse(width - 1 + height - 1 + i));
        }
    }

    let mut valid = false;
    let mut second_time_at_end = 0;
    while let Some(((x, y, time), distance)) = priority_queue.pop() {
        if x == 0 && y == 0 {
            second_time_at_end = time + 1;
            valid = true;
            break;
        }

        let new_time = (time + 1) % total_time;

        if !data[new_time][y][x] {
            priority_queue.push((x, y, time + 1), Reverse(distance.0 + 1));
        }

        if y < height - 1 && !data[new_time][y + 1][x] {
            priority_queue.push((x, y + 1, time + 1), Reverse(distance.0 + 2));
        }

        if y > 0 && !data[new_time][y - 1][x] {
            priority_queue.push((x, y - 1, time + 1), Reverse(distance.0));
        }

        if x < width - 1 && !data[new_time][y][x + 1] {
            priority_queue.push((x + 1, y, time + 1), Reverse(distance.0 + 2));
        }

        if x > 0 && !data[new_time][y][x - 1] {
            priority_queue.push((x - 1, y, time + 1), Reverse(distance.0));
        }
    }
    if !valid {
        panic!("No solution found");
    }

    let mut priority_queue = PriorityQueue::new();
    for i in 0..data.len() {
        if !data[(i + second_time_at_end + 1) % total_time][0][0] {
            priority_queue.push((0, 0, i + second_time_at_end + 1), Reverse(width - 1 + height - 1 + i));
        }
    }

    while let Some(((x, y, time), distance)) = priority_queue.pop() {
        if x == width - 1 && y == height - 1 {
            return time + 1;
        }

        let new_time = (time + 1) % total_time;

        if !data[new_time][y][x] {
            priority_queue.push((x, y, time + 1), Reverse(distance.0 + 1));
        }

        if y < height - 1 && !data[new_time][y + 1][x] {
            priority_queue.push((x, y + 1, time + 1), Reverse(distance.0));
        }

        if y > 0 && !data[new_time][y - 1][x] {
            priority_queue.push((x, y - 1, time + 1), Reverse(distance.0 + 2));
        }

        if x < width - 1 && !data[new_time][y][x + 1] {
            priority_queue.push((x + 1, y, time + 1), Reverse(distance.0));
        }

        if x > 0 && !data[new_time][y][x - 1] {
            priority_queue.push((x - 1, y, time + 1), Reverse(distance.0 + 2));
        }
    }
    panic!("No solution found");
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    let maps = &data[1..data.len() - 1];
    let mut blizzards = Vec::new();
    for (y, map) in maps.iter().enumerate() {
        for (x, c) in map[1..map.len() - 1].iter().enumerate() {
            if *c == b'^' {
                blizzards.push(Blizzard {
                    x,
                    y,
                    direction: Direction::Up,
                });
            } else if *c == b'>' {
                blizzards.push(Blizzard {
                    x,
                    y,
                    direction: Direction::Right,
                });
            } else if *c == b'v' {
                blizzards.push(Blizzard {
                    x,
                    y,
                    direction: Direction::Down,
                });
            } else if *c == b'<' {
                blizzards.push(Blizzard {
                    x,
                    y,
                    direction: Direction::Left,
                });
            }
        }
    }

    // Calculate least common multiple of width and height
    let width = maps[0].len() - 2;
    let height = maps.len();
    let lcm = width * height / (width.gcd(height));
    let mut occupancy_map = vec![vec![vec![false; width]; height]; lcm];
    for i in 0..lcm {
        for j in &blizzards {
            let (x, y) = j.position_at(width, height, i);
            occupancy_map[i][y][x] = true;
        }
    }

    occupancy_map
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.into_iter().map(|s| s.as_bytes()).collect()
}