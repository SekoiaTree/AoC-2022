use std::time::Instant;

type Data<'a> = Vec<&'a [u8]>;

pub fn run(data: Data) -> usize {
    let height = data.len();
    let width = data[0].len();
    let mut count = 2*height+2*(width-2);

    let mut visible = vec![vec![false; width]; height];
    for (y, i) in data[..height-1].iter().enumerate().skip(1) {
        let mut highest_yet = i[0];
        for (x, j) in i[..width-1].iter().enumerate().skip(1) {
            if *j > highest_yet {
                count += 1;
                visible[y][x] = true;
                highest_yet = *j;
            }
        }

        let mut highest_yet = i[width-1];
        for (x, j) in i[..width-1].iter().enumerate().skip(1).rev() {
            if *j > highest_yet {
                if !visible[y][x] {
                    count += 1;
                    visible[y][x] = true;
                }
                highest_yet = *j;
            }
        }
    }

    for x in 1..width-1 {
        let mut highest_yet = data[0][x];
        for y in 1..height-1 {
            if data[y][x] > highest_yet {
                if !visible[y][x] {
                    count += 1;
                    visible[y][x] = true;
                }
                highest_yet = data[y][x];
            }
        }

        let mut highest_yet = data[height-1][x];
        for y in (1..height-1).rev() {
            if data[y][x] > highest_yet {
                if !visible[y][x] {
                    count += 1;
                    visible[y][x] = true;
                }
                highest_yet = data[y][x];
            }
        }
    }

    count
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let height = data.len();
    let width = data[0].len();
    let mut max = 0;
    for (y, i) in data[..height-1].iter().enumerate().skip(1) {
        for (x, j) in i[..width-1].iter().enumerate().skip(1) {
            let this_height = *j;
            let mut score = 1;

            let mut x_offset = 1;
            while x >= x_offset {
                if data[y][x-x_offset] >= this_height {
                    break;
                }
                x_offset += 1;
            }
            score *= x_offset.min(x);

            let mut x_offset = 1;
            while x+x_offset < data[y].len() {
                if data[y][x+x_offset] >= this_height {
                    break;
                }
                x_offset += 1;
            }

            score *= x_offset.min(data[y].len()-1-x);

            let mut y_offset = 1;
            while y >= y_offset {
                if data[y-y_offset][x] >= this_height {
                    break;
                }
                y_offset += 1;
            }
            score *= y_offset.min(y);

            let mut y_offset = 1;
            while y+y_offset < data.len() {
                if data[y + y_offset][x] >= this_height {
                    break;
                }
                y_offset += 1;
            }
            score *= y_offset.min(data.len()-y-1);

            if score > max {
                max = score;
            }
        }
    }

    max
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.iter().map(|x| x.as_bytes()).collect()
}