use std::time::Instant;

/*
use std::collections::HashSet;

type Data = Vec<(HashSet<char>, HashSet<char>)>;

pub fn run(data: Data) -> u32 {
    data.iter().map(|(a, b)| a.intersection(b).map(|x| if x.is_lowercase() {
        *x as u32 - 'a' as u32+1
    } else {
        *x as u32 - 'A' as u32+27
    }).sum::<u32>()).sum::<u32>()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> u32 {
    data.chunks(3).map(|x| {
        let a = x[0].0.union(&x[0].1.clone()).map(|x| *x).collect::<HashSet<char>>();
        let b = x[1].0.union(&x[1].1.clone()).map(|x| *x).collect::<HashSet<char>>();
        let c = x[2].0.union(&x[2].1.clone()).map(|x| *x).collect::<HashSet<char>>();

        a.intersection(&b.intersection(&c).map(|x| *x).collect::<HashSet<char>>()).map(|x| if x.is_lowercase() {
            *x as u32 - 'a' as u32+1
        } else {
            *x as u32 - 'A' as u32+27
        }).sum::<u32>()
    }).sum()
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|s| {
        let (left, right) = s.split_at(s.len() / 2);
        let mut left_set = HashSet::new();
        let mut right_set = HashSet::new();

        for c in left {
            left_set.insert(*c as char);
        }

        for c in right {
            right_set.insert(*c as char);
        }

        (left_set, right_set)
    }).collect()
}

pub fn free_convert<'a>(data: Vec<&'a str>) -> ConvertData<'a> {
    data.iter().map(|s| s.as_bytes()).collect()
}*/

type Data = Vec<(u64, u64)>;

pub fn run(data: Data) -> u32 {
    data.iter().map(|(l, r)| {
        let intersection = l & r;

        intersection.trailing_zeros() + 1
    }).sum::<u32>()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> u32 {
    data.chunks(3).map(|x| {
        let intersection = (x[0].0 | x[0].1) & (x[1].0 | x[1].1) & (x[2].0 | x[2].1);

        intersection.trailing_zeros() + 1
    }).sum()
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|s| {
        let mut left = 0;
        let mut right = 0;

        let len = s.len();
        for c in s[..len / 2].iter() {
            let c = *c as char;
            if c >= 'a' && c <= 'z' {
                left |= 1 << (c as u32 - 'a' as u32);
            } else {
                left |= 1 << (c as u32 - 'A' as u32 + 26);
            }
        }
        for c in s[len/2..].iter() {
            let c = *c as char;
            if c >= 'a' && c <= 'z' {
                right |= 1 << (c as u32 - 'a' as u32);
            } else {
                right |= 1 << (c as u32 - 'A' as u32 + 26);
            }
        }
        (left, right)
    }).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.iter().map(|s| s.as_bytes()).collect()
}