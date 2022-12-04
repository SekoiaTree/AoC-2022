use std::time::Instant;

type Data = Vec<((usize, usize), (usize, usize))>;

pub fn run(data: Data) -> usize {
    data.iter().filter(|&&(l, r)| l.0 >= r.0 && l.1 <= r.1 || r.0 >= l.0 && r.1 <= l.1).count()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    data.iter().filter(|&&(l,r)| l.1 >= r.0 && l.1 <= r.1 || r.1 >= l.0 && r.1 <= l.1).count()
}

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| {
        let (left, right) = x.split_once(',').unwrap();
        let (a, b) = left.split_once('-').unwrap();
        let left = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
        let (a, b) = right.split_once('-').unwrap();
        let right = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
        (left, right)
    }).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
}