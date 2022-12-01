# AoC-2022
My solutions in Rust for advent of code 2022.

To run, run `cargo run --release`. If part 2 is included, you'll want to do `--features part-two`.

By default the build script will take today's date as the day to run (i.e. on the third of December it will run day 3). Override this by enabling the feature `day-override` with `--features day-override` (don't add `--features` again if you're running part two), and with the file `day_override.txt` (which is the day to run).
