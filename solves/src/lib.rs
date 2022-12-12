extern crate core;
include!(concat!(env!("OUT_DIR"), "/linker.rs"));

#[path = "code/day1.rs"]
mod day1;
#[path = "code/day2.rs"]
mod day2;
#[path = "code/day3.rs"]
mod day3;
#[path = "code/day4.rs"]
mod day4;
#[path = "code/day5.rs"]
mod day5;
#[path = "code/day6.rs"]
mod day6;
#[path = "code/day7.rs"]
mod day7;
#[path = "code/day8.rs"]
mod day8;
#[path = "code/day9.rs"]
mod day9;
#[path = "code/day10.rs"]
mod day10;
#[path = "code/day11.rs"]
mod day11;
#[path = "code/day12.rs"]
mod day12;