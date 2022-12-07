use std::time::Instant;

type Data<'a> = &'a [u8];

pub fn run(data: Data) -> usize {
    /*'main: for (i, b) in data.windows(4).enumerate() {
        for (index, x) in b.iter().enumerate() {
            for y in &b[index+1..] {
                if *x == *y {
                    continue 'main;
                }
            }
        }
        return i+4;
    }

    panic!("No window found!");*/
    let mut bits : u32 = 0;
    let mut count : u32 = 0;
    let mut prev_count : u32 = 0;
    let mut i = 0;
    while count < 4 && i < data.len() {
        bits |= 1 << (data[i] - b'a');
        count = bits.count_ones();
        if count == prev_count {
            i -= prev_count as usize;
            bits = 0;
            prev_count = 0;
        } else {
            prev_count = count;
        }
        i += 1;
    }

    i
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
/*    'main: for (i, b) in data.windows(14).enumerate() {
        for (index, x) in b.iter().enumerate() {
            for y in &b[index+1..] {
                if *x == *y {
                    continue 'main;
                }
            }
        }
        return i+14;
    }

    panic!("No window found!");*/
    let mut bits : u32 = 0;
    let mut count : u32 = 0;
    let mut prev_count : u32 = 0;
    let mut i = 0;
    while count < 14 && i < data.len() {
        bits |= 1 << (data[i] - b'a');
        count = bits.count_ones();
        if count == prev_count {
            i -= prev_count as usize;
            bits = 0;
            prev_count = 0;
        } else {
            prev_count = count;
        }
        i += 1;
    }

    i
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data[0]
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.iter().map(|x| x.as_bytes()).collect()
}