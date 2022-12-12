use std::cmp::Reverse;
use std::collections::HashSet;
use std::time::Instant;
use priority_queue::PriorityQueue;
use aoc_util::parse_in_grid;

type Data = (Vec<Vec<u32>>, (usize, usize), (usize, usize));

pub fn run(data: Data) -> u32 {
    let (map, start, end) = data;
    let mut queue = PriorityQueue::new();
    let mut visited_set = HashSet::new();
    queue.push(start, Reverse(0));
    while queue.len() > 0 {
        let ((i, j), len) = queue.pop().unwrap();
        if !visited_set.insert((i, j)) {
            continue;
        }

        if (i, j) == end {
            return len.0;
        }

        if i > 0 && map[i-1][j] <= map[i][j]+1 {
            queue.push((i-1, j), Reverse(len.0+1));
        }

        if i < map.len()-1 && map[i+1][j] <= map[i][j]+1 {
            queue.push((i+1, j), Reverse(len.0+1));
        }

        if j > 0 && map[i][j-1] <= map[i][j]+1 {
            queue.push((i, j-1), Reverse(len.0+1));
        }

        if j < map[i].len()-1 && map[i][j+1] <= map[i][j]+1 {
            queue.push((i, j+1), Reverse(len.0+1));
        }
    }
    panic!("No solution found!")
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let (map, _start, end) = data;
    let mut queue = PriorityQueue::new();
    let mut visited_set = HashSet::new();
    queue.push(end, Reverse(0));
    while queue.len() > 0 {
        let ((i, j), len) = queue.pop().unwrap();
        if !visited_set.insert((i, j)) {
            continue;
        }

        if map[i][j] == 0 {
            return len.0;
        }

        if i > 0 && map[i][j] <= map[i-1][j]+1 {
            queue.push((i-1, j), Reverse(len.0+1));
        }

        if i < map.len()-1 && map[i][j] <= map[i+1][j]+1 {
            queue.push((i+1, j), Reverse(len.0+1));
        }

        if j > 0 && map[i][j] <= map[i][j-1]+1 {
            queue.push((i, j-1), Reverse(len.0+1));
        }

        if j < map[i].len()-1 && map[i][j] <= map[i][j+1]+1 {
            queue.push((i, j+1), Reverse(len.0+1));
        }
    }
    panic!("No solution found!")
}

// type ConvertData<'a> = Vec<&'a str>;
type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    let mut start = (0, 0);
    let mut end = (0, 0);
    (data.iter().enumerate().map(|(i, x)| x.iter().enumerate().map(|(j, v)| {
        if *v == b'S' {
            start = (i, j);
            0
        } else if *v == b'E' {
            end = (i, j);
            25
        } else {
            (*v-b'a') as u32
        }
    }).collect()).collect(), start, end)
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.iter().map(|x| x.as_bytes()).collect()
}