use std::collections::{BTreeSet, HashSet};
use std::time::Instant;
use itertools::Itertools;

type Data = Vec<((i32, i32), (i32, i32))>;

pub fn run(data: Data) -> usize {
    const ROW : i32 = 2000000;
    //let (min, max) = data.iter().map(|x| x.1.0).minmax().into_option().unwrap();
    let mut map = HashSet::new();
    let mut beacons = HashSet::new();

    for (sensor, beacon) in data {
        if beacon.1 == ROW {
            beacons.insert(beacon.0);
        }
        let distance = sensor.0.abs_diff(beacon.0)+sensor.1.abs_diff(beacon.1);
        let allowed_distance  = distance.checked_sub(sensor.1.abs_diff(ROW));
        if allowed_distance.is_none() {
            continue;
        }
        let allowed_distance = allowed_distance.unwrap();
        for i in sensor.0-allowed_distance as i32..=sensor.0+allowed_distance as i32 {
            map.insert(i);
        }
    }
    map.difference(&beacons).count()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> u64 {
    const MAX_COORD : i32 = 4000000;
    'y_coordinates: for i in 0..=MAX_COORD {
        let mut ranges = Vec::new();
        for (sensor, beacon) in &data {
            if beacon.1 == i {
                ranges.push((beacon.1, beacon.1));
            }

            let distance = sensor.0.abs_diff(beacon.0)+sensor.1.abs_diff(beacon.1);
            let allowed_distance  = distance.checked_sub(sensor.1.abs_diff(i));
            if allowed_distance.is_none() {
                continue;
            }
            let allowed_distance = allowed_distance.unwrap();
            ranges.push((sensor.0.saturating_sub(allowed_distance as i32), sensor.0+allowed_distance as i32));
        }
        ranges.sort();
        let mut position = 0;
        for i in ranges {
            if i.0 <= position && position <= i.1 {
                position = i.1 + 1;
                if position > MAX_COORD {
                    continue 'y_coordinates;
                }
            }
        }
        return position as u64*4000000+i as u64;
    }
    0
}

type ConvertData<'a> = Vec<&'a str>;
// type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| x.split(',').map(|x| {
        let (l, r) = x.split_once(' ').unwrap();
        (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
    }).collect_tuple().unwrap()).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
    // data.iter().map(|x| x.as_bytes()).collect()
}