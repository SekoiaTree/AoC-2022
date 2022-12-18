extern crate core;
include!(concat!(env!("OUT_DIR"), "/linker.rs"));

macro_rules! mod_day_dummy {
    ($n:literal) => {
        #[cfg(feature = "dummy-feature")]
        #[path = "code/day18.rs"]
        mod day$n;
    };
}

#[cfg(feature = "dummy-feature")]
#[path = "code/day18.rs"]
mod day18;