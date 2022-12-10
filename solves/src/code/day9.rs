use std::collections::HashSet;
use std::time::Instant;
use itertools::max;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Right,
    Up,
    Down,
    Left,
}

impl Direction {
    pub fn signum(&self) -> i32 {
        match self {
            Direction::Right | Direction::Up => 1,
            Direction::Down | Direction::Left => -1,
        }
    }

    pub fn x(&self) -> i32 {
        match self {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0
        }
    }

    pub fn y(&self) -> i32 {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0
        }
    }

    pub fn char(&self) -> char {
        match self {
            Direction::Right => 'R',
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L'
        }
    }
}

type Data = Vec<(Direction, u32)>;

fn signum_offset(x: i32, n: i32) -> i32 {
    if x.abs() <= n {
        0
    } else {
        x.signum()
    }
}

pub fn run(data: Data) -> usize {
    let mut positions = HashSet::new();
    let mut head = (0i32, 0i32);
    let mut tail = (0i32, 0i32);

    positions.insert(tail);
    for (i, count) in data {
        let prev_tail = tail;
        head.0 += i.x() * (count as i32);
        head.1 += i.y() * (count as i32);

        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
            let diff_x = head.0 - tail.0;
            let diff_y = head.1 - tail.1;
            let min_abs = if diff_x.abs() == diff_y.abs() {
                diff_x.abs() - 1
            } else if diff_x.abs() < diff_y.abs() {
                diff_x.abs()
            } else {
                diff_y.abs()
            };
            tail.0 += (head.0 - tail.0).signum() * min_abs;
            tail.1 += (head.1 - tail.1).signum() * min_abs;

            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 { // One of the two is 0 now
                tail.0 += (head.0 - tail.0) - (head.0 - tail.0).signum();
                tail.1 += (head.1 - tail.1) - (head.1 - tail.1).signum();
            }
        }

        let mut start_x = prev_tail.0;
        let mut end_x = tail.0;
        start_x += (end_x-start_x).signum();
        if start_x > end_x {
            std::mem::swap(&mut start_x, &mut end_x);
        }
        let range_x = start_x..=end_x;

        let mut start_y = prev_tail.1;
        let mut end_y = tail.1;
        start_y += (end_y-start_y).signum();
        if start_y > end_y {
            std::mem::swap(&mut start_y, &mut end_y);
        }
        let range_y = start_y..=end_y;

        let x_size = range_x.size_hint().0;
        let y_size = range_y.size_hint().0;
        let max_size = x_size.max(y_size);
        let range_x = range_x.chain(std::iter::repeat(tail.0).take(max_size-x_size));
        let range_y = range_y.chain(std::iter::repeat(tail.1).take(max_size-y_size));

        for (x, y) in range_x.zip(range_y) {
            positions.insert((x, y));
        }
    }

    positions.len()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let mut positions = HashSet::new();
    let mut rope = [(0i32, 0i32); 10];
    positions.insert(rope[9]);

    for (i, count) in data {
        for _ in 0..count {
            rope[0].0 += i.x();
            rope[0].1 += i.y();

            for j in 1..10 {
                if (rope[j-1].0 - rope[j].0).abs() > 1 || (rope[j-1].1 - rope[j].1).abs() > 1 {
                    rope[j].0 += (rope[j-1].0 - rope[j].0).signum();
                    rope[j].1 += (rope[j-1].1 - rope[j].1).signum();
                }
            }
            positions.insert(rope[9]);

        }
    }
    positions.len()
}

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| {
        let (left, right) = x.split_once(' ').unwrap();
        let right = right.parse::<u32>().unwrap();
        (match left {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => panic!("Invalid direction")
        }, right)
    }).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
}