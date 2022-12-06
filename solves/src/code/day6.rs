use std::time::Instant;

type Data<'a> = Vec<&'a [u8]>;

pub fn run(data: Data) -> usize {
    'main: for (i, b) in data[0].windows(4).enumerate() {
        for (index, x) in b.iter().enumerate() {
            for y in &b[index+1..] {
                if *x == *y {
                    continue 'main;
                }
            }
        }
        return i+4;
    }

    panic!("No window found!");
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    'main: for (i, b) in data[0].windows(14).enumerate() {
        for (index, x) in b.iter().enumerate() {
            for y in &b[index+1..] {
                if *x == *y {
                    continue 'main;
                }
            }
        }
        return i+14;
    }

    panic!("No window found!");
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.iter().map(|x| x.as_bytes()).collect()
}