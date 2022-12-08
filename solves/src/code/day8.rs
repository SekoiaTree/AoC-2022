use std::time::Instant;

type Data = Vec<Vec<u32>>;

pub fn run(data: Data) -> usize {
    let mut count = 0;
    for (y, i) in data.iter().enumerate() {
        for (x, j) in i.iter().enumerate() {
            if x == 0 || y == 0 || x == data[y].len()-1 || y == data.len()-1 {
                count += 1;
                continue;
            }
            let this_height = *j;
            let mut valid_tree = true;

            let mut valid_tree_from_this_direction = false;
            let mut x_offset = 1;
            while x >= x_offset {
                if data[y][x-x_offset] >= this_height {
                    valid_tree_from_this_direction = true;
                    break;
                }
                x_offset += 1;
            }
            valid_tree &= valid_tree_from_this_direction;

            let mut valid_tree_from_this_direction = false;
            let mut x_offset = 1;
            while x+x_offset < data[y].len() {
                if data[y][x+x_offset] >= this_height {
                    valid_tree_from_this_direction = true;
                    break;
                }
                x_offset += 1;
            }

            valid_tree &= valid_tree_from_this_direction;

            let mut valid_tree_from_this_direction = false;
            let mut y_offset = 1;
            while y >= y_offset {
                if data[y-y_offset][x] >= this_height {
                    valid_tree_from_this_direction = true;
                    break;
                }
                y_offset += 1;
            }
            valid_tree &= valid_tree_from_this_direction;

            let mut valid_tree_from_this_direction = false;
            let mut y_offset = 1;
            while y+y_offset < data.len() {
                if data[y+y_offset][x] >= this_height {
                    valid_tree_from_this_direction = true;
                    break;
                }
                y_offset += 1;
            }

            valid_tree &= valid_tree_from_this_direction;
            if !valid_tree {
                count += 1;
            }
        }
    }
    count
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    let mut max = 0;
    for (y, i) in data.iter().enumerate() {
        for (x, j) in i.iter().enumerate() {
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
            }            if x == 2 && y==3 {
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

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect()
}

pub fn free_convert<'a>(data: Vec<&'a str>) -> ConvertData<'a> {
    data
}