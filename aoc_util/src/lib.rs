use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Neg, RangeInclusive, Sub, SubAssign};
use std::str::FromStr;

pub const BLOCK_CHAR: char = '█';
pub const EMPTY_CHAR: char = ' ';
pub const BLOCK_STR: &str = "█";
pub const EMPTY_STR: &str = " ";

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

pub fn small_to_big_range<T: PartialOrd>(from: T, to: T) -> RangeInclusive<T> {
    if from < to {
        from..=to
    } else {
        to..=from
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct IPoint {
    x: i32,
    y: i32,
}


impl IPoint {
    pub const NORTH : IPoint = IPoint::new(0, -1);
    pub const SOUTH : IPoint = IPoint::new(0, 1);
    pub const WEST : IPoint = IPoint::new(-1, 0);
    pub const EAST : IPoint = IPoint::new(1, 0);

    pub const fn new(x: i32, y: i32) -> Self {
        IPoint { x, y }
    }

    pub const fn from(v: (i32, i32)) -> Self {
        IPoint { x: v.0, y: v.0 }
    }

    pub const fn x(&self) -> i32 {
        return self.x;
    }

    pub const fn y(&self) -> i32 {
        return self.y;
    }

    pub const fn manhattan_distance(&self, rhs: Self) -> u32 {
        self.x.abs_diff(rhs.x)+self.y.abs_diff(rhs.y)
    }

    pub const fn norm_squared(&self) -> u32 {
        (self.x*self.x) as u32 + (self.y*self.y) as u32
    }

    pub fn neighbours(&self) -> Vec<IPoint> {
        vec![IPoint::new(self.x+1, self.y), IPoint::new(self.x, self.y+1), IPoint::new(self.x-1, self.y), IPoint::new(self.x, self.y-1)]
    }

    pub fn neighbours_8(&self) -> Vec<IPoint> {
        vec![IPoint::new(self.x+1, self.y), IPoint::new(self.x, self.y+1), IPoint::new(self.x-1, self.y), IPoint::new(self.x, self.y-1), IPoint::new(self.x+1, self.y+1), IPoint::new(self.x-1, self.y+1), IPoint::new(self.x-1, self.y-1), IPoint::new(self.x+1, self.y-1)]
    }

    pub fn north(&self) -> IPoint {
        *self + IPoint::NORTH
    }

    pub fn south(&self) -> IPoint {
        *self + IPoint::SOUTH
    }

    pub fn west(&self) -> IPoint {
        *self + IPoint::WEST
    }

    pub fn east(&self) -> IPoint {
        *self + IPoint::EAST
    }
}

impl Add for IPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for IPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl AddAssign for IPoint {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for IPoint {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for IPoint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl Mul<i32> for IPoint {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self { x: self.x*rhs, y: self.y*rhs }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tree<T, S = ()> {
    Node(Vec<Tree<T>>, S),
    Leaf(T),
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

impl<T> PartialOrd for Tree<T, ()>
    where T: PartialOrd + Clone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Tree::Node(x, _) => {
                match other {
                    Tree::Node(v, _) => x.partial_cmp(v),
                    Tree::Leaf(_) => x.partial_cmp(&vec![other.clone()])
                }
            }
            Tree::Leaf(v) => {
                match other {
                    Tree::Node(x, _) => vec![self.clone()].partial_cmp(x),
                    Tree::Leaf(x) => v.partial_cmp(x)
                }
            }
        }
    }
}

impl<T> Ord for Tree<T, ()>
    where T: Ord + Clone {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Tree::Node(x, _) => {
                match other {
                    Tree::Node(v, _) => x.cmp(v),
                    Tree::Leaf(v) => x.cmp(&vec![Tree::Leaf(v.clone())])
                }
            }
            Tree::Leaf(v) => {
                match other {
                    Tree::Node(x, _) => (vec![Tree::Leaf(v.clone())]).cmp(x),
                    Tree::Leaf(x) => v.cmp(x)
                }
            }
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