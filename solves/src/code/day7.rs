use std::time::Instant;

#[derive(Clone, Debug)]
pub enum ListObject {
    Directory(Vec<ListObject>),
    File(u32)
}

impl ListObject {
    pub fn is_directory(&self) -> bool {
        match self {
            ListObject::Directory(_) => true,
            ListObject::File(_) => false
        }
    }
}

type Data = ListObject;

pub fn run(data: Data) -> u32 {
    pub fn total_size_sum_total_sizes(object: ListObject) -> (u32, u32) {
        match object {
            ListObject::Directory(x) => {
                let mut total_sum = 0;
                let mut sum = 0;
                for y in x {
                    if y.is_directory() {
                        let (their_sum, their_total_sum) = total_size_sum_total_sizes(y);
                        sum += their_sum;
                        total_sum += their_total_sum;
                    } else {
                        sum += total_size_sum_total_sizes(y).0;
                    }
                }
                if sum < 100000 {
                    (sum, total_sum+sum)
                } else {
                    (sum, total_sum)
                }
            }
            ListObject::File(x) => (x, x)
        }
    }

    total_size_sum_total_sizes(data).1
}

fn total_size(object: &ListObject) -> u32 {
    match object {
        ListObject::Directory(x) => {
            let mut sum = 0;
            for y in x {
                if y.is_directory() {
                    let their_sum = total_size(y);
                    sum += their_sum;
                } else {
                    sum += total_size(y);
                }
            }
            sum
        }
        ListObject::File(x) => *x
    }
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> u32 {
    fn smallest_of_size(object: ListObject, min: u32) -> u32 {
        match object {
            ListObject::Directory(x) => {
                let mut min_dir = None;
                let mut sum = 0;
                for y in x {
                    if y.is_directory() {
                        let their_sum = smallest_of_size(y, min);
                        if their_sum >= min && (min_dir.is_none() || min_dir.unwrap() > their_sum) {
                            min_dir = Some(their_sum);
                        } else {
                            sum += their_sum;
                        }
                    } else {
                        sum += total_size(&y);
                    }
                }
                if min_dir.is_some() { min_dir.unwrap() } else { sum }
            }
            ListObject::File(x) => x
        }
    }
    let total_size_of_root = 30000000-(70000000-total_size(&data));
    smallest_of_size(data, total_size_of_root)
}

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    fn recursive_parse(slice: &[&str]) -> (ListObject, usize) {
        let mut map : Vec<ListObject> = Vec::new();
        let mut index = 1;
        while index < slice.len() {
            let element = slice[index];
            if element.starts_with("$ cd ") {
                let dir = element.strip_prefix("$ cd ").unwrap();
                if dir == ".." {
                    return (ListObject::Directory(map), index);
                } else {
                    index += 1;
                    let (other, end_index) = recursive_parse(&slice[index..]);
                    index += end_index;
                    map.push(other);
                }
            } else if element.starts_with("dir ") {} else {
                let len = element.find(' ').unwrap();
                map.push(ListObject::File(element[..len].parse().unwrap()));
            }
            index += 1;
        }

        (ListObject::Directory(map), index)
    }

    recursive_parse(&data[..]).0
}

pub fn free_convert<'a>(data: Vec<&'a str>) -> ConvertData<'a> {
    data
}