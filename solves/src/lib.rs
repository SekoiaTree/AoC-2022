include!(concat!(env!("OUT_DIR"), "/linker.rs"));

macro_rules! dummy_day_mod {
    ($d:ident, $x:literal) => {
        #[cfg(feature="dummy-feature")]
        #[path = $x]
        mod $d;
    };
}

dummy_day_mod!(day1, "code/day1.rs");
dummy_day_mod!(day2, "code/day2.rs");
dummy_day_mod!(day3, "code/day3.rs");
dummy_day_mod!(day4, "code/day4.rs");
dummy_day_mod!(day5, "code/day5.rs");
dummy_day_mod!(day6, "code/day6.rs");
dummy_day_mod!(day7, "code/day7.rs");
dummy_day_mod!(day8, "code/day8.rs");
dummy_day_mod!(day9, "code/day9.rs");
dummy_day_mod!(day10, "code/day10.rs");
dummy_day_mod!(day11, "code/day11.rs");
dummy_day_mod!(day12, "code/day12.rs");
dummy_day_mod!(day13, "code/day13.rs");
dummy_day_mod!(day14, "code/day14.rs");
dummy_day_mod!(day15, "code/day15.rs");
dummy_day_mod!(day16, "code/day16.rs");
dummy_day_mod!(day17, "code/day17.rs");
dummy_day_mod!(day18, "code/day18.rs");
// dummy_day_mod!(day19, "code/day19.rs");
// dummy_day_mod!(day20, "code/day20.rs");
// dummy_day_mod!(day21, "code/day21.rs");
// dummy_day_mod!(day22, "code/day22.rs");
// dummy_day_mod!(day23, "code/day23.rs");
// dummy_day_mod!(day24, "code/day24.rs");
// dummy_day_mod!(day25, "code/day25.rs");