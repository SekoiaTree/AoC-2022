use std::collections::{HashMap, HashSet};
use std::time::Instant;
use itertools::Itertools;
use aoc_util::IPoint;

type Data = HashSet<IPoint>;

fn north(occupation_map: &HashMap<IPoint, Option<IPoint>>, query: IPoint) -> bool {
    if !occupation_map.contains_key(&query.north()) &&
        !occupation_map.contains_key(&query.north().west()) &&
        !occupation_map.contains_key(&query.north().east()) {
        true
    } else {
        false
    }
}

fn south(occupation_map: &HashMap<IPoint, Option<IPoint>>, query: IPoint) -> bool {
    if !occupation_map.contains_key(&query.south()) &&
        !occupation_map.contains_key(&query.south().west()) &&
        !occupation_map.contains_key(&query.south().east()) {
        true
    } else {
        false
    }
}

fn west(occupation_map: &HashMap<IPoint, Option<IPoint>>, query: IPoint) -> bool {
    if !occupation_map.contains_key(&query.west()) &&
        !occupation_map.contains_key(&query.west().north()) &&
        !occupation_map.contains_key(&query.west().south()) {
        true
    } else {
        false
    }
}

fn east(occupation_map: &HashMap<IPoint, Option<IPoint>>, query: IPoint) -> bool {
    if !occupation_map.contains_key(&query.east()) &&
        !occupation_map.contains_key(&query.east().north()) &&
        !occupation_map.contains_key(&query.east().south()) {
        true
    } else {
        false
    }
}

fn nearby(occupation_map: &HashMap<IPoint, Option<IPoint>>, query: IPoint) -> bool {
    query.neighbours_8().iter().any(|p| occupation_map.contains_key(p))
}

const CHECKS : [fn(&HashMap<IPoint, Option<IPoint>>, IPoint) -> bool; 4] = [north, south, west, east];
const DIRECTIONS : [IPoint; 4] = [IPoint::NORTH, IPoint::SOUTH, IPoint::WEST, IPoint::EAST];

pub fn run(data: Data) -> i32 {
    let mut move_directions = HashMap::new();
    for i in data {
        move_directions.insert(i, None);
    }
    let mut pointer = 0;

    for _ in 0..10 {
        let mut new_move_directions = move_directions.clone();

        let mut proposed_locations : HashMap<IPoint, usize> = HashMap::new();
        for (p, direction) in move_directions.iter_mut() {
            *direction = None;
            if !nearby(&new_move_directions, *p) {
                continue;
            }
            for i in 0..CHECKS.len() {
                let check = CHECKS[(i+pointer) % CHECKS.len()];
                if check(&new_move_directions, *p) {
                    proposed_locations.insert(*p + DIRECTIONS[(i+pointer) % DIRECTIONS.len()],
                                              proposed_locations.get(&(*p + DIRECTIONS[(i+pointer) % DIRECTIONS.len()])).unwrap_or(&0) + 1);
                    *direction = Some(*p + DIRECTIONS[(i+pointer) % DIRECTIONS.len()]);
                    break;
                }
            }
        }

        // dbg!(&proposed_locations, &move_directions);
        for (p, direction) in move_directions.iter() {
            if let Some(d) = direction {
                if *proposed_locations.get(d).unwrap_or(&0) == 1 {
                    new_move_directions.remove(p);
                    new_move_directions.insert(*d, None);
                } else {
                    new_move_directions.insert(*p, None);
                }
            } else {
                new_move_directions.insert(*p, None);
            }
        }
        pointer += 1;
        move_directions = new_move_directions;
    }

    let (min_x, max_x) = move_directions.keys().map(|p| p.x()).minmax().into_option().unwrap();
    let (min_y, max_y) = move_directions.keys().map(|p| p.y()).minmax().into_option().unwrap();

    // for j in -5..10 {
    //     for i in -5..10 {
    //         let p = IPoint::new(i, j);
    //         if move_directions.contains_key(&p) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    (max_x - min_x + 1) * (max_y - min_y + 1) - move_directions.len() as i32
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let mut move_directions = HashMap::new();
    for i in data {
        move_directions.insert(i, None);
    }
    let mut pointer = 0;

    for v in 1.. {
        let mut new_move_directions = move_directions.clone();

        let mut proposed_locations : HashMap<IPoint, usize> = HashMap::new();
        for (p, direction) in move_directions.iter_mut() {
            *direction = None;
            if !nearby(&new_move_directions, *p) {
                continue;
            }
            for i in 0..CHECKS.len() {
                let check = CHECKS[(i+pointer) % CHECKS.len()];
                if check(&new_move_directions, *p) {
                    proposed_locations.insert(*p + DIRECTIONS[(i+pointer) % DIRECTIONS.len()],
                                              proposed_locations.get(&(*p + DIRECTIONS[(i+pointer) % DIRECTIONS.len()])).unwrap_or(&0) + 1);
                    *direction = Some(*p + DIRECTIONS[(i+pointer) % DIRECTIONS.len()]);
                    break;
                }
            }
        }

        // dbg!(&proposed_locations, &move_directions);
        for (p, direction) in move_directions.iter() {
            if let Some(d) = direction {
                if *proposed_locations.get(d).unwrap_or(&0) == 1 {
                    new_move_directions.remove(p);
                    new_move_directions.insert(*d, None);
                } else {
                    new_move_directions.insert(*p, None);
                }
            } else {
                new_move_directions.insert(*p, None);
            }
        }
        pointer += 1;
        if move_directions == new_move_directions {
            return v;
        }
        move_directions = new_move_directions;
    }
    unreachable!()
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    let mut elves = HashSet::new();
    for (i, v) in data.into_iter().enumerate() {
        for (j, w) in v.into_iter().enumerate() {
            if *w == b'#' {
                elves.insert(IPoint::new(j as i32, i as i32));
            }
        }
    }
    elves
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.into_iter().map(|x| x.as_bytes()).collect()
}