use std::time::Instant;

type Data = Vec<Vec<u32>>;

pub(crate) fn run(data : Data) -> u32 {
    data.iter().map(|x| x.iter().sum()).max().unwrap()
}

#[cfg(feature = "part-two")]
pub(crate) fn run_step2(data: Data) -> u32 {
    let mut sorted : Vec<u32> = data.iter().map(|x| x.iter().sum()).collect();
    sorted.sort_by(|x,y| Ord::cmp(y, x));

    sorted[..3].iter().sum()
}

pub(crate) fn convert(data: Vec<String>, _profiling: Instant) -> Data {
    data.split(|x| x.is_empty()).map(|x| x.iter().map(|y| y.parse::<u32>().unwrap()).collect()).collect()
}