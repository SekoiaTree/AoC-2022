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

pub fn parse_in_grid_bytes<T, F>(data: Vec<&[u8]>, parse: F) -> Vec<Vec<T>>
    where F: FnMut(&u8) -> T + Copy {
    data.iter().map(|x| x.iter().map(parse).collect()).collect()
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

    pub fn get_child(&self, index: usize) -> &Tree<T> {
        match self {
            Tree::Node(x, _) => &x[index],
            Tree::Leaf(_) => panic!("Tried to get a child of a leaf!")
        }
    }
}


/*#[derive(Clone, Debug)]
pub struct TreeIterator<'a, T> {
    stack: Vec<&'a Tree<T>>,
    index_stack: Vec<usize>,
    tree: Tree<T>
}

impl<'a, T> Iterator for TreeIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut last = *self.stack.last()?;
        while last.is_node() {
            let child = last.get_child(self.index)
        }
    }
}

impl<'a, T> IntoIterator for Tree<T> {
    type Item = T;
    type IntoIter = TreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}*/