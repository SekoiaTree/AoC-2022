use std::time::Instant;

type Data = Vec<Vec<u32>>;

pub(crate) fn run(data : Data) -> u32 {
    let iterator = data.iter().map(|x| x.iter().sum());
    let mut max = 0;
    for i in iterator {
        if i > max {
            max = i;
        }
    }

    max
}

#[cfg(feature = "part-two")]
pub(crate) fn run_step2(data: Data) -> u32 {
    /* This is O(nlog n). Cringe.
    let mut sorted : Vec<u32> = data.iter().map(|x| x.iter().sum()).collect();
    sorted.sort_by(|x,y| Ord::cmp(y, x));

    sorted[..3].iter().sum()*/
    // This is O(n).
    let iterator = data.iter().map(|x| x.iter().sum());

    let mut max = [0; 3];
    for i in iterator {
        if i > max[0] {
            max[0] = i;
            // A call to sort? Bad, but tbh max is so small it doesn't matter.
            max.sort();
        }
    }
    max.iter().sum()
    // This is also O(n), but slower because it does 3 passes.
    /*let mut sparse_sums : Vec<Option<u32>> =  data.iter().map(|x| Some(x.iter().sum())).collect();

    let mut sum = 0;
    for i in 0..3 {
        let (index, value) = sparse_sums.iter().enumerate().max_by_key(|&(_, item)| *item).unwrap();
        sum += value.unwrap();
        sparse_sums[index] = None;
    }
    sum*/
}

type ConvertData<'a> = Vec<&'a str>;

pub(crate) fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.split(|x| x.is_empty()).map(|x| x.iter().map(|y| y.parse::<u32>().unwrap()).collect()).collect()
}

pub(crate) fn free_convert<'a>(data: Vec<&'a str>) -> ConvertData<'a> {
    data
}