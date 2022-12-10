use std::fmt::Debug;
use std::str::FromStr;

pub const BLOCK_CHAR : char = '█';
pub const EMPTY_CHAR : char = ' ';
pub const BLOCK_STR : &str = "█";
pub const EMPTY_STR : &str = " ";

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

pub fn parse_in_grid<T, F>(data: Vec<&str>, parse: F) -> Vec<Vec<T>>
    where F: FnMut(char) -> T + Copy {
    data.iter().map(|x| x.chars().map(parse).collect()).collect()
}

#[derive(Clone, Debug)]
pub enum Tree<T, S=()> {
    Node(Vec<Tree<T>>, S),
    Leaf(T)
}

impl<T, S> Tree<T, S> {
    pub fn is_node(&self) -> bool {
        if let Tree::Leaf(_) = self {
            false
        } else {
            true
        }
    }
}
/*
#[derive(Clone, Debug)]
pub struct TreeIterator<T> {
    index_stack: Vec<usize>,
    tree: Tree<T>
}

impl<T> Iterator for TreeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let last = *self.index_stack.last().unwrap();
    }
}

impl<T> IntoIterator for Tree<T> {
    type Item = T;
    type IntoIter = TreeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}*/