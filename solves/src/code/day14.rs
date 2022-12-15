use std::time::Instant;
use itertools::{Itertools, max};

type Data = Vec<Vec<(usize, usize)>>;


pub fn run(data: Data) -> usize {
    fn display_buffer(buffer: &Vec<Vec<bool>>) {
        for i in buffer.iter().enumerate() {
            print!("{} ", i.0);
            for j in i.1 {
                print!("{}", if *j { aoc_util::EMPTY_CHAR } else { aoc_util::BLOCK_CHAR });
            }
            println!();
        }
    }
    let (leftmost, rightmost) = data.iter().flatten().map(|x| x.0).minmax().into_option().unwrap();
    let max_y = data.iter().flatten().map(|x| x.1).max().unwrap();

    let mut buffer = vec![vec![true; rightmost-leftmost+1]; max_y+1];
    for i in data {
        for j in i.windows(2) {
            let from = j[0];
            let to = j[1];
            if from.0 == to.0 {
                let (from_pos, to_pos) = if from.1 <= to.1 { (from.1, to.1) } else { (to.1, from.1) };
                for x in from_pos..=to_pos {
                    buffer[x][from.0-leftmost] = false;
                }
            } else {
                let (from_pos, to_pos) = if from.0 <= to.0 { (from.0, to.0) } else { (to.0, from.0) };
                for x in from_pos..=to_pos {
                    buffer[from.1][x-leftmost] = false;
                }
            }
        }
    }

    //display_buffer(&buffer);

    let mut counter = 0;
    'rest: loop {
        let mut moving_sand = (500-leftmost, 0);
        while moving_sand.1 < max_y {
            if buffer[moving_sand.1+1][moving_sand.0] {
                moving_sand.1 += 1;
            } else if moving_sand.0 > 0 && buffer[moving_sand.1+1][moving_sand.0-1] {
                moving_sand.0 -= 1;
                moving_sand.1 += 1;
            } else if moving_sand.0 < rightmost-leftmost+1 && buffer[moving_sand.1+1][moving_sand.0+1] {
                moving_sand.0 += 1;
                moving_sand.1 += 1;
            } else if moving_sand.0 > 0 && moving_sand.0 < rightmost-leftmost+1 { // If any of these aren't true, we could slip off.
                buffer[moving_sand.1][moving_sand.0] = false;
                counter += 1;
                //display_buffer(&buffer);
                //println!();
                continue 'rest;
            } else {
                break;
            }
        }
        return counter;
    }
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    fn display_buffer(buffer: &Vec<Vec<bool>>) {
        for i in buffer.iter().enumerate() {
            print!("{:0>2} ", i.0);
            for j in i.1 {
                print!("{}", if *j{ aoc_util::EMPTY_CHAR } else { aoc_util::BLOCK_CHAR });
            }
            println!();
        }
    }
    let (leftmost, rightmost) = data.iter().flatten().map(|x| x.0).minmax().into_option().unwrap();
    let max_y = data.iter().flatten().map(|x| x.1).max().unwrap();
    let (leftmost, max_x) = if max_y*2+2 >= rightmost-leftmost+1 {
        (499-max_y, max_y*2+3)
    } else {
        (leftmost, rightmost-leftmost+1)
    };

    let mut buffer = vec![vec![true; max_x]; max_y+1];
    buffer.push(vec![true; max_x]);
    buffer.push(vec![false; max_x]);
    for i in data {
        for j in i.windows(2) {
            let from = j[0];
            let to = j[1];
            if from.0 == to.0 {
                let (from_pos, to_pos) = if from.1 <= to.1 { (from.1, to.1) } else { (to.1, from.1) };
                for x in from_pos..=to_pos {
                    buffer[x][from.0-leftmost] = false;
                }
            } else {
                let (from_pos, to_pos) = if from.0 <= to.0 { (from.0, to.0) } else { (to.0, from.0) };
                for x in from_pos..=to_pos {
                    buffer[from.1][x-leftmost] = false;
                }
            }
        }
    }

    //display_buffer(&buffer);

    let mut counter = 0;
    'rest: loop {
        let mut moving_sand = (500-leftmost, 0);
        if buffer[moving_sand.1+1][moving_sand.0-1..=moving_sand.0+1].iter().all(|x| !*x) {
            return counter+1;
        }
        while moving_sand.1 < max_y+2 {
            if buffer[moving_sand.1+1][moving_sand.0] {
                moving_sand.1 += 1;
            } else if moving_sand.0 > 0 && buffer[moving_sand.1+1][moving_sand.0-1] {
                moving_sand.0 -= 1;
                moving_sand.1 += 1;
            } else if moving_sand.0 < max_x-1 && buffer[moving_sand.1+1][moving_sand.0+1]  {
                moving_sand.0 += 1;
                moving_sand.1 += 1;
            } else {
                buffer[moving_sand.1][moving_sand.0] = false;
                counter += 1;
                //display_buffer(&buffer);
                //println!();
                continue 'rest;
            }
        }
    }
}

type ConvertData<'a> = Vec<&'a str>;
// type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| x.split(" -> ").map(|y| {
        let (left, right) = y.split_once(',').unwrap();
        (left.parse().unwrap(), right.parse().unwrap())
    }).collect()).collect()
}

pub fn free_convert<'a>(data: Vec<&'a str>) -> ConvertData<'a> {
    data
    // data.iter().map(|x| x.as_bytes()).collect()
}