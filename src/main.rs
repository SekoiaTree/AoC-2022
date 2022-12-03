extern crate core;
include!(concat!(env!("OUT_DIR"), "/linker.rs"));

use std::time::Instant;

fn main() {
    println!("Today is day #{}. Beginning program...", DAY);

    let contents = day::free_convert(INPUT.trim().lines().collect());

    let profiling = Instant::now();
    let processed = day::convert(contents, profiling);
    let processed_2 = processed.clone();

    let processing_time = profiling.elapsed();
    println!("Processing data complete... Time taken: {} microseconds or {} ms. \n", processing_time.as_micros(), processing_time.as_millis());
    let result = day::run(processed);
    let time_taken_p1 = profiling.elapsed() - processing_time;
    println!("Result of part 1: {:?}. This took: {} microseconds or {} ms.", result, time_taken_p1.as_micros(), time_taken_p1.as_millis());
    #[cfg(feature = "part-two")]{
        let result2 = day::run_step2(processed_2);
        let p2_time = profiling.elapsed() - time_taken_p1 - processing_time;
        println!("Result of part 2: {:?}. This took {} microseconds or {} ms.\n", result2, p2_time.as_micros(), p2_time.as_millis());
        println!("Total time taken: {} microseconds or {} ms.", (p2_time + time_taken_p1 + processing_time).as_micros(), (p2_time + time_taken_p1 + processing_time).as_millis());
    }
    #[cfg(not(feature = "part-two"))]{
        println!("Part 2 not completed. Skipping...");
    }

    day_x::free_convert(Vec::new());
    day_x::convert(Vec::new(), profiling);
    day_x::run(Vec::new());
    day_x::run_step2(Vec::new());
}

#[path = "code/day3.rs"]
mod day_x;