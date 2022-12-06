use std::time::Instant;
use itertools::Itertools;

type Data = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

pub fn run(data: Data) -> String {
    let (mut stacks, instructions) = data;
    for i in instructions {
        assert_ne!(i.1, i.2);
        let count = i.0;
        let from = i.1-1;
        let to = i.2-1;

        let stack = std::mem::take(&mut stacks[from]);
        let len = stack.len();
        stacks[to].extend(stack[len-count..].iter().rev());
        stacks[from] = stack;

        stacks[from].truncate(len-count);
    }
    let mut result = String::with_capacity(stacks.len());
    for i in stacks {
        result.push(*i.last().unwrap());
    }

    result
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> String {
    let (mut stacks, instructions) = data;
    for i in instructions {
        let count = i.0;
        let from = i.1-1;
        let to = i.2-1;

        let stack = std::mem::take(&mut stacks[from]);
        let len = stack.len();
        stacks[to].extend_from_slice(&stack[len-count..]);
        stacks[from] = stack;

        stacks[from].truncate(len-count);
    }
    let mut result = String::with_capacity(stacks.len());
    for i in stacks {
        result.push(*i.last().unwrap());
    }

    result
}

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    let (init, instructions) = data.split_at(9);
    let init = init.iter().map(|x| x.split_whitespace().map(|x| x.chars().next().unwrap()).collect()).collect();
    let instructions = instructions[1..].iter()
        .map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect_tuple().unwrap())
        .collect();
    (init, instructions)
}

pub fn free_convert<'a>(data: Vec<&'a str>) -> ConvertData<'a> {
    data
}