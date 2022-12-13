use std::cmp::Ordering;
use std::time::Instant;

use aoc_util::Tree;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct PacketData(u32);

type Data = Vec<Tree<PacketData>>;

pub fn run(data: Data) -> usize {
    data.chunks(2).enumerate().filter(|(_, x)| x[0] <= x[1]).map(|(index, _)| index + 1).sum()
}

#[cfg(feature = "part-two")]
pub fn run_step2(mut data: Data) -> usize {
    data.sort_unstable();
    (data.binary_search(&Tree::Node(vec![Tree::Leaf(PacketData(2))], ())).unwrap_or_else(|x| x)+1) *
        (data.binary_search(&Tree::Node(vec![Tree::Leaf(PacketData(6))], ())).unwrap_or_else(|x| x)+2)
}

type ConvertData<'a> = Vec<&'a str>;

fn parse_tree(tree: &str) -> Tree<PacketData> {
    if tree.starts_with('[') {
        if tree.len() == 2 {
            return Tree::Node(vec![], ());
        }
        let mut children = Vec::new();
        let mut depth = 0;
        let mut start_index = 1;
        for (end_index, i) in tree.chars().enumerate() {
            if i == '[' {
                depth += 1;
            } else if i == ']' {
                depth -= 1;
            } else if i == ',' && depth == 1 {
                children.push(parse_tree(&tree[start_index..end_index]));
                start_index = end_index + 1;
            }
        }
        let end = tree.len() - 1;
        children.push(parse_tree(&tree[start_index..end]));
        Tree::Node(children, ())
    } else {
        Tree::Leaf(PacketData(tree.parse::<u32>().unwrap()))
    }
}

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().filter(|x| !x.is_empty()).map(|x| {
        parse_tree(*x)
    }).collect()
}

pub fn free_convert<'a>(data: Vec<&'a str>) -> ConvertData<'a> {
    data
}