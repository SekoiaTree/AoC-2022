use std::time::Instant;

type Data = Vec<i64>;

fn lookup_print(data: &Vec<i64>, index_vec: &Vec<usize>) {
    for i in index_vec.iter().map(|x| data[*x]) {
        print!("{} ", i);
    }
    println!();
}

pub fn run(data: Data) -> i64 {    let mut index_vec : Vec<usize> = (0..data.len()).collect();
    for i in 0..data.len() {
        //lookup_print(&data, &index_vec);
        let index_element = index_vec.iter().position(|x| *x == i).unwrap();
        let new_position = (index_element as i64 + data[i]).rem_euclid(data.len() as i64 - 1) as usize;
        if new_position > index_element {
            index_vec[index_element..=new_position].rotate_left(1);
        } else {
            index_vec[new_position..=index_element].rotate_right(1);
        }
    }
    let zero_index = index_vec.iter().position(|x| data[*x] == 0).unwrap();
    [1000, 2000, 3000].into_iter().map(|x| data[index_vec[(zero_index+x) % data.len()]]).sum::<i64>()
}

#[cfg(feature = "part-two")]
pub fn run_step2(mut data: Data) -> i64 {
    data.iter_mut().for_each(|x| *x *= 811589153);
    let mut index_vec : Vec<usize> = (0..data.len()).collect();
    for _ in 0..10 {
        for i in 0..data.len() {
            //lookup_print(&data, &index_vec);
            let index_element = index_vec.iter().position(|x| *x == i).unwrap();
            let new_position = (index_element as i64 + data[i]).rem_euclid(data.len() as i64 - 1) as usize;
            if new_position > index_element {
                index_vec[index_element..=new_position].rotate_left(1);
            } else {
                index_vec[new_position..=index_element].rotate_right(1);
            }
        }
    }
    let zero_index = index_vec.iter().position(|x| data[*x] == 0).unwrap();
    [1000, 2000, 3000].into_iter().map(|x| data[index_vec[(zero_index+x) % data.len()]]).sum::<i64>()
}

type ConvertData<'a> = Vec<&'a str>;
// type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    aoc_util::ints(data)
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
    // data.iter().map(|x| x.as_bytes()).collect()
}