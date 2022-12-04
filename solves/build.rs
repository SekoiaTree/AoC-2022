use std::{env, fs};
use std::path::Path;
#[cfg(not(feature = "day-override"))]
use chrono::Local;
#[cfg(not(feature = "day-override"))]
use chrono::Datelike;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("linker.rs");

    #[cfg(feature = "day-override")]
        let day = {
            let override_path = Path::new("src").join("day_override.txt");
            let day = fs::read_to_string(override_path).unwrap();
            day.trim().to_string().parse().unwrap_or(1)
        };
    #[cfg(not(feature = "day-override"))]
        let day = Local::now().day();

    let code_path = Path::new("src").join(format!("code/day{}.rs", day));
    let day_code = fs::read_to_string(code_path).unwrap();

    let input_path = Path::new("src").join(format!("input/day{}.txt", day));
    let input = fs::read_to_string(input_path).unwrap();
    let code = format!("pub mod day {{
{}
}}
pub const DAY: u32 = {};
pub const INPUT : &str = \"{}\";", day_code, day, input);
    fs::write(
        &dest_path,
        code
    ).unwrap();
}