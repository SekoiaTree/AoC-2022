use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use priority_queue::PriorityQueue;

type Data = Vec<(usize, usize, (usize, usize), (usize, usize))>;

pub fn run(data: Data) -> usize {
    let mut threads: [Option<JoinHandle<usize>>; 8] = [0; 8].map(|_| None);
    let mut next = 0;
    let mut sum = 0;
    loop {
        let mut all_none = true;
        for i in 0..threads.len() {
            match &threads[i] {
                Some(thread) if thread.is_finished() => {
                    let v = std::mem::take(&mut threads[i]).unwrap();
                    sum += v.join().unwrap();
                    all_none = false;
                },
                Some(_) => all_none = false,
                None => if next < data.len() {
                    let (ore, clay, obsidian, geode) = data[next];
                    println!("Running {}", next + 1);
                    threads[i] = Some(thread::spawn(move || {
                        let ret = (next + 1) * max_geodes(24, ore, clay, obsidian, geode);
                        println!("{}: {}", next + 1, ret);
                        ret
                    }));
                    all_none = false;
                    next += 1;
                }
            }
        }
        if all_none {
            return sum;
        }
        thread::sleep(Duration::from_millis(2));
    }
}

fn max_geodes(minutes: usize, ore_collector: usize, clay_collector: usize, obsidian_collector: (usize, usize), geode_collector: (usize, usize)) -> usize {
    let mut queue = PriorityQueue::new();


    let mut seen = vec![Vec::new(); minutes + 1];
    queue.push(((0, 0, 0, 0), 1, 0, 0, 0), (0, 0, 0, 0, 0));

    let mut max = 0;
    while !queue.is_empty() {
        let ((_, ore_bots, clay_bots, obsidian_bots, geode_bots), (time, geode, obsidian, clay, ore)) = queue.pop().unwrap();
        //let time = time.0;
        let time_to_end = minutes-time;
        let max_geodes = geode+time_to_end*geode_bots+(time_to_end-1)*time_to_end/2;
        if max_geodes <= max {
            continue;
        }

        if seen[time].iter().any(|(o, c, ob, g, o_b, c_b, ob_b, g_b)| {
            *o >= ore && *c >= clay && *ob >= obsidian && *g >= geode &&
                *o_b >= ore_bots && *c_b >= clay_bots && *ob_b >= obsidian_bots && *g_b >= geode_bots
        }) {
            continue;
        }
        if time == minutes {
            max = max.max(geode);
            continue;
        }
        seen[time].push((ore, clay, obsidian, geode, ore_bots, clay_bots, obsidian_bots, geode_bots));

        if obsidian_bots >= 1 {
            let extra_time = ((geode_collector.1 - obsidian)/obsidian_bots); // This needs to be like... rounded up or something. It's late man, screw this...
            let new_ore = ore_bots * extra_time - geode_collector.0;
            let new_obsidian
            queue.push_increase((
                                    (new_ore - geode_collector.0, new_clay, new_obsidian - geode_collector.1, new_geode),
                                    ore_bots, clay_bots, obsidian_bots, geode_bots + 1
                                ), (time + 1, new_geode, new_obsidian - geode_collector.1, new_clay, new_ore - geode_collector.0));
        }

        if clay_bots >= 1 {
            queue.push_increase((
                                    (new_ore - obsidian_collector.0, new_clay - obsidian_collector.1, new_obsidian, new_geode),
                                    ore_bots, clay_bots, obsidian_bots + 1, geode_bots
                                ), (time + 1, new_geode, new_obsidian, new_clay - obsidian_collector.1, new_ore - obsidian_collector.0));
        }

        let mut build_clay_and_ore = true;
        if ore >= clay_collector {
            if !seen[time+1].iter().any(|(o, c, ob, g, o_b, c_b, ob_b, g_b)| {
                *o >= new_ore - clay_collector && *c >= new_clay && *ob >= obsidian && *g >= new_geode &&
                    *o_b >= ore_bots && *c_b >= clay_bots + 1 && *ob_b >= obsidian_bots && *g_b >= geode_bots
            }) {
                queue.push_increase((
                                        (new_ore - clay_collector, new_clay, new_obsidian, new_geode),
                                        ore_bots, clay_bots + 1, obsidian_bots, geode_bots
                                    ), (time + 1, new_geode, new_obsidian, new_clay, new_ore - clay_collector));
            }
        } else {
            build_clay_and_ore = false;
        }

        if ore >= ore_collector {
            if !seen[time+1].iter().any(|(o, c, ob, g, o_b, c_b, ob_b, g_b)| {
                *o >= new_ore - ore_collector && *c >= new_clay && *ob >= obsidian && *g >= new_geode &&
                    *o_b >= ore_bots + 1 && *c_b >= clay_bots && *ob_b >= obsidian_bots && *g_b >= geode_bots
            }) {
                queue.push_increase((
                                        (new_ore - ore_collector, new_clay, new_obsidian, new_geode),
                                        ore_bots + 1, clay_bots, obsidian_bots, geode_bots
                                    ), (time + 1, new_geode, new_obsidian, new_clay, new_ore - ore_collector));
            }
        } else {
            build_clay_and_ore = false;
        }

        if !(build_clay_and_ore && (clay_bots == 0 || build_obsidian) && (obsidian_bots == 0 || build_geode)) {
            if !seen[time+1].iter().any(|(o, c, ob, g, o_b, c_b, ob_b, g_b)| {
                *o >= new_ore && *c >= new_clay && *ob >= obsidian && *g >= new_geode &&
                    *o_b >= ore_bots && *c_b >= clay_bots && *ob_b >= obsidian_bots && *g_b >= geode_bots
            }) {
                queue.push_increase((
                                        (new_ore, new_clay, new_obsidian, new_geode),
                                        ore_bots, clay_bots, obsidian_bots, geode_bots
                                    ), (time + 1, new_geode, new_obsidian, new_clay, new_ore));
            }
        }
    }

    max
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let data = &data[..3];
    let mut threads: [Option<JoinHandle<usize>>; 6] = [0; 6].map(|_| None);
    let mut next = 0;
    let mut sum = 1;
    loop {
        let mut all_none = true;
        for i in 0..threads.len() {
            match &threads[i] {
                Some(thread) if thread.is_finished() => {
                    let v = std::mem::take(&mut threads[i]).unwrap();
                    sum *= v.join().unwrap();
                    all_none = false;
                },
                Some(_) => all_none = false,
                None => if next < data.len() {
                    let (ore, clay, obsidian, geode) = data[next];
                    println!("Running {}", next + 1);
                    threads[i] = Some(thread::spawn(move || {
                        let ret = max_geodes(33, ore, clay, obsidian, geode);
                        println!("{}: {}", next + 1, ret);
                        ret
                    }));
                    all_none = false;
                    next += 1;
                }
            }
        }
        if all_none {
            return sum;
        }
        thread::sleep(Duration::from_millis(2));
    }
}

type ConvertData<'a> = Vec<&'a str>;
// type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| {
        let costs: Vec<usize> = x.split(' ').map(|v| v.parse().unwrap()).collect();
        (costs[0], costs[1], (costs[2], costs[3]), (costs[4], costs[5]))
    }).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
    // data.iter().map(|x| x.as_bytes()).collect()
}