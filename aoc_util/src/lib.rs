use std::fmt::Debug;
use std::str::FromStr;

pub fn ints<T>(data: Vec<&str>) -> Vec<T>
    where T: FromStr,
          T::Err: Debug {
    data.iter().map(|x| x.parse::<T>().unwrap()).collect()
}

pub fn ints_in_line<T>(data: &str) -> Vec<T>
    where T: FromStr,
          T::Err: Debug {
    data.split_whitespace().map(|x| x.parse::<T>().unwrap()).collect()
}

pub fn ints_in_line_sep<T>(data: &str, sep: &str) -> Vec<T>
    where T: FromStr,
          T::Err: Debug {
    data.split(sep).map(|x| x.parse::<T>().unwrap()).collect()
}
/*
pub trait SplitMap<T>
    where T: Iterator {
    fn split_and_map<'a, P, S>(self, split: P, map: S)
        where P: Pattern<'a>;
}*/