use std::collections::{HashSet, VecDeque};
use std::time::Instant;

use itertools::Itertools;

type Data = HashSet<(usize, usize, usize)>;

pub fn run(data: Data) -> usize {
    let mut faces = 0;
    for (x, y, z) in &data {
        if *x == 0 || !data.contains(&(*x - 1, *y, *z)) {
            faces += 1;
        }
        if !data.contains(&(*x + 1, *y, *z)) {
            faces += 1;
        }
        if *y == 0 || !data.contains(&(*x, *y - 1, *z)) {
            faces += 1;
        }
        if !data.contains(&(*x, *y + 1, *z)) {
            faces += 1;
        }
        if *z == 0 || !data.contains(&(*x, *y, *z - 1)) {
            faces += 1;
        }
        if !data.contains(&(*x, *y, *z + 1)) {
            faces += 1;
        }
    }

    faces
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let mut exterior_reachable = HashSet::new();

    let mut queue = VecDeque::new();
    for i in 0..20 {
        for j in 0..20 {
            queue.push_back((i, j, 0));
            queue.push_back((i, j, 19));
            queue.push_back((0, i, j));
            queue.push_back((19, i, j));
            queue.push_back((i, 0, j));
            queue.push_back((i, 19, j));
        }
    }
    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        if exterior_reachable.contains(&(x, y, z)) || data.contains(&(x, y, z)) {
            continue;
        }
        exterior_reachable.insert((x, y, z));

        if x > 0 {
            queue.push_back((x - 1, y, z));
        }

        if x < 19 {
            queue.push_back((x + 1, y, z));
        }

        if y > 0 {
            queue.push_back((x, y - 1, z));
        }

        if y < 19 {
            queue.push_back((x, y + 1, z));
        }

        if z > 0 {
            queue.push_back((x, y, z - 1));
        }

        if z < 19 {
            queue.push_back((x, y, z + 1));
        }
    }

    let mut faces = 0;
    for &(x, y, z) in &data {
        if x == 0 || exterior_reachable.contains(&(x - 1, y, z)) {
            faces += 1;
        }
        if x == 19 || exterior_reachable.contains(&(x + 1, y, z)) {
            faces += 1;
        }
        if y == 0 || exterior_reachable.contains(&(x, y - 1, z)) {
            faces += 1;
        }
        if y == 19 || exterior_reachable.contains(&(x, y + 1, z)) {
            faces += 1;
        }
        if z == 0 || exterior_reachable.contains(&(x, y, z - 1)) {
            faces += 1;
        }
        if z == 19 || exterior_reachable.contains(&(x, y, z + 1)) {
            faces += 1;
        }
    }

    faces
}

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| x.split(',').map(|i| i.parse().unwrap()).collect_tuple().unwrap()).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
}