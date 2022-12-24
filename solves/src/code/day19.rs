use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use priority_queue::PriorityQueue;

type Data = Vec<(usize, usize, (usize, usize), (usize, usize))>;

pub fn run(data: Data) -> usize {
    // let mut threads: [Option<JoinHandle<usize>>; 8] = [0; 8].map(|_| None);
    // let mut next = 0;
    // let mut sum = 0;
    // loop {
    //     let mut all_none = true;
    //     for i in 0..threads.len() {
    //         match &threads[i] {
    //             Some(thread) if thread.is_finished() => {
    //                 let v = std::mem::take(&mut threads[i]).unwrap();
    //                 sum += v.join().unwrap();
    //                 all_none = false;
    //             },
    //             Some(_) => all_none = false,
    //             None => if next < data.len() {
    //                 let (ore, clay, obsidian, geode) = data[next];
    //                 println!("Running {}", next + 1);
    //                 threads[i] = Some(thread::spawn(move || {
    //                     let ret = (next + 1) * max_geodes(24, ore, clay, obsidian, geode);
    //                     println!("{}: {}", next + 1, ret);
    //                     ret
    //                 }));
    //                 all_none = false;
    //                 next += 1;
    //             }
    //         }
    //     }
    //     if all_none {
    //         return sum;
    //     }
    //     thread::sleep(Duration::from_millis(2));
    // }
    let mut sum = 0;
    for (i, (ore, clay, obsidian, geode)) in data.iter().enumerate() {
        sum += (i + 1)*max_geodes(24, *ore, *clay, *obsidian, *geode);
        println!("DONE {}", i);
    }
    sum
}

fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

fn max_geodes(minutes: usize, ore_collector: usize, clay_collector: usize, obsidian_collector: (usize, usize), geode_collector: (usize, usize)) -> usize {
    let mut queue = PriorityQueue::new();
    let max_ore_bots = ore_collector.max(clay_collector).max(obsidian_collector.0).max(geode_collector.0);
    let max_clay_bots = obsidian_collector.1;
    let max_obsidian_bots = geode_collector.1;

    queue.push((1, 0, 0, 0, 0, 0, 0), 0);

    let mut max = 0;
    while let Some(((ore_bots, clay_bots, obsidian_bots, ore, clay, obsidian, geode), time)) = queue.pop() {
        if time == minutes {
            if geode > max {
                max = geode;
            }
            continue;
        }

        if ore_bots < max_ore_bots {
            let needed_ore = ore_collector.saturating_sub(ore);
            let added_time = div_ceil(needed_ore, ore_bots) + 1;
            let new_time = time + added_time;
            if new_time <= minutes {
                queue.push_increase((ore_bots + 1, clay_bots, obsidian_bots,
                            ore + added_time * ore_bots - ore_collector, clay + added_time * clay_bots, obsidian + added_time * obsidian_bots, geode), new_time);
            }
        }

        if clay_bots < max_clay_bots {
            let needed_ore = clay_collector.saturating_sub(ore);
            let added_time = div_ceil(needed_ore, ore_bots) + 1;
            let new_time = time + added_time;
            if new_time <= minutes {
                queue.push_increase((ore_bots, clay_bots + 1, obsidian_bots,
                            ore + added_time * ore_bots - clay_collector, clay + added_time * clay_bots, obsidian + added_time * obsidian_bots, geode), new_time);
            }
        }

        if obsidian_bots < max_obsidian_bots && clay_bots > 0 {
            let needed_ore = obsidian_collector.0.saturating_sub(ore);
            let needed_clay = obsidian_collector.1.saturating_sub(clay);
            let added_time = div_ceil(needed_ore, ore_bots).max(div_ceil(needed_clay, clay_bots)) + 1;
            let new_time = time + added_time;
            if new_time <= minutes {
                queue.push((ore_bots, clay_bots, obsidian_bots + 1,
                            ore + added_time * ore_bots - obsidian_collector.0, clay + added_time * clay_bots - obsidian_collector.1,
                            obsidian + added_time * obsidian_bots, geode), new_time);
            }
        }

        if obsidian_bots > 0 {
            let needed_ore = geode_collector.0.saturating_sub(ore);
            let needed_obsidian = geode_collector.1.saturating_sub(obsidian);

            let added_time = div_ceil(needed_ore, ore_bots).max(div_ceil(needed_obsidian, obsidian_bots)) + 1;
            let new_time = time + added_time;
            if new_time <= minutes {
                queue.push((ore_bots, clay_bots, obsidian_bots,
                            ore + added_time * ore_bots - geode_collector.0, clay + added_time * clay_bots, obsidian + added_time * obsidian_bots - geode_collector.1,
                            geode + (minutes-new_time)), new_time);
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
                        let ret = max_geodes(32, ore, clay, obsidian, geode);
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