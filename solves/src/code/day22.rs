use std::time::Instant;
use itertools::Itertools;
use aoc_util::IPoint;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Wall,
    Nothing,
    Empty
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn to_point(&self) -> IPoint {
        match self {
            Direction::Up => IPoint::new(0, -1),
            Direction::Down => IPoint::new(0, 1),
            Direction::Left => IPoint::new(-1, 0),
            Direction::Right => IPoint::new(1, 0)
        }
    }

    pub fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }

    pub fn mirror_x(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            _ => *self
        }
    }

    pub fn mirror_y(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            _ => *self
        }
    }

    pub fn swap(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }
}

type Data = (Vec<Vec<Cell>>, Vec<usize>, Vec<(usize, bool)>);

pub fn run(data: Data) -> i32 {
    let mut position = IPoint::new(data.1[0] as i32, 0);
    let mut direction = Direction::Right;

    for (len, left) in data.2 {
        for _ in 0..len {
            let new_position = position + direction.to_point();
            let mut new_position = IPoint::new(new_position.x(), new_position.y().rem_euclid(data.0.len() as i32));

            if let Direction::Left | Direction::Right = direction {
                new_position = IPoint::new((new_position.x() - data.1[new_position.y() as usize] as i32).rem_euclid(data.0[new_position.y() as usize].len() as i32 - data.1[new_position.y() as usize] as i32)
                                           + data.1[new_position.y() as usize] as i32, new_position.y());
            } else {
                if new_position.x() >= data.0[new_position.y() as usize].len() as i32 || data.0[new_position.y() as usize][new_position.x() as usize] == Cell::Nothing {
                    if direction == Direction::Up {
                        new_position = IPoint::new(new_position.x(), data.0.len() as i32 - 1);
                        while new_position.x() >= data.0[new_position.y() as usize].len() as i32 || data.0[new_position.y() as usize][new_position.x() as usize] == Cell::Nothing {
                            new_position += IPoint::new(0, -1);
                        }
                    } else {
                        new_position = IPoint::new(new_position.x(), 0);
                        while new_position.x() >= data.0[new_position.y() as usize].len() as i32 || data.0[new_position.y() as usize][new_position.x() as usize] == Cell::Nothing {
                            new_position += IPoint::new(0, 1);
                        }
                    }
                }
            }
            
            if data.0[new_position.y() as usize][new_position.x() as usize] == Cell::Wall {
                break;
            }

            position = new_position;
        }

        direction = if left {
            direction.left()
        } else {
            direction.right()
        }
    }

    1000*(position.y() + 1) + 4*(position.x() + 1) + match direction {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0
    }
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> i32 {
    let mut faces: [Vec<Vec<Cell>>; 6] = [0; 6].map(|_| vec![]);
    faces[0] = data.0[150..200].iter().map(|x| x[0..50].to_owned()).collect_vec();
    faces[1] = data.0[100..150].iter().map(|x| x[0..50].to_owned()).collect_vec();
    faces[2] = data.0[100..150].iter().map(|x| x[50..100].to_owned()).collect_vec();
    faces[3] = data.0[0..50].iter().map(|x| x[100..150].to_owned()).collect_vec();
    faces[4] = data.0[0..50].iter().map(|x| x[50..100].to_owned()).collect_vec();
    faces[5] = data.0[50..100].iter().map(|x| x[50..100].to_owned()).collect_vec();
    let adjacencies = [
        [1, 2, 3, 4],
        [5, 2, 0, 4],
        [5, 3, 0, 1],
        [0, 2, 5, 4],
        [0, 3, 5, 1],
        [4, 3, 2, 1],
    ];
    let transition_params = [
        [(false, true, false), (false, false, true), (false, true, false), (false, false, true)],
        [(false, false, true), (true, false, false), (false, true, false), (false, true, false)],
        [(false, true, false), (false, true, false), (false, false, true), (true, false, false)],
        [(false, true, false), (false, true, false), (false, false, true), (true, false, false)],
        [(false, false, true), (true, false, false), (false, true, false), (false, true, false)],
        [(false, true, false), (false, false, true), (false, true, false), (false, false, true)],
    ];

    fn map_faces(point: IPoint, direction: Direction, params: (bool, bool, bool)) -> (IPoint, Direction) {
        let new_x = if params.2 { point.y() } else { point.x() }.clamp(0, 49);
        let new_y = if params.2 { point.x() } else { point.y() }.clamp(0, 49);
        let mut new_direction = direction;
        if params.2 {
            new_direction = new_direction.swap();

        }
        if params.1 {
            new_direction = new_direction.mirror_y();
        }
        if params.0 {
            new_direction = new_direction.mirror_x();
        }

        (IPoint::new(
            if params.0 { 49-new_x } else { new_x },
            if params.1 { 49-new_y } else { new_y },
        ), new_direction)
    }


    let mut position = IPoint::new(0, 0);
    let mut face = 4;
    let mut direction = Direction::Right;

    for (len, left) in data.2 {
        for _ in 0..len {
            let mut new_position = position + direction.to_point();
            let mut new_face = face;
            let mut new_direction = direction;

            if !(new_position.x() < 50 && new_position.x() >= 0 && new_position.y() < 50 && new_position.y() >= 0) {
                // CHANGE FACES!
                let face_off = if new_position.y() < 0 {
                    0
                } else if new_position.x() >= 50 {
                    1
                } else if new_position.y() >= 50 {
                    2
                } else {
                    3
                };

                new_face = adjacencies[face][face_off];
                let params = transition_params[face][face_off];
                (new_position, new_direction) = map_faces(new_position, direction, params);
                // dbg!(new_position, new_face, new_direction);
                // println!();
            }

            if faces[new_face][new_position.y() as usize][new_position.x() as usize] == Cell::Wall {
                break;
            }

            position = new_position;
            face = new_face;
            direction = new_direction;
        }

        direction = if left {
            direction.left()
        } else {
            direction.right()
        };
        // dbg!(position, direction, face);
        // println!();
        // println!("WJOIJ");
    }

    // dbg!(position, face, direction);
    let position_lookup = [(0, 150), (0, 100), (50, 100), (100, 0), (50, 0), (50, 50)];

    1000*(position.y() + position_lookup[face].1 + 1) + 4*(position.x() + position_lookup[face].0 + 1) + match direction {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0
    }
}

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    let (map, instructions) : (&[&str], &[&str]) = data.split(|x| x.is_empty()).collect_tuple().unwrap();
    let maps = map.into_iter().map(|x| {
        let vec = x.chars().map(|c| match c {
            ' ' | '-' => Cell::Nothing,
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            _ => panic!("Illegal char {}", c)
        }).collect_vec();
        let index = vec.iter().position(|x| *x != Cell::Nothing).unwrap();
        (vec, index)
    }).collect_vec();

    let indices = maps.iter().map(|x| x.1).collect_vec();
    let maps = maps.into_iter().map(|x| x.0).collect_vec();

    let mut instructions = instructions[0].split_inclusive(|x : char| x.is_alphabetic()).map(|v| {
        if !v.chars().last().unwrap().is_alphabetic() {
            (v.parse().unwrap(), false)
        } else {
            (v[..v.len()-1].parse().unwrap(), v.chars().last().unwrap() == 'L')
        }
    }).collect_vec();
    instructions.push((0, true));

    (maps, indices, instructions)
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
}


#[cfg(test)]
mod tests {
    use std::assert_eq;
    use super::*;
    fn map_faces(direction: Direction, params: (bool, bool, bool)) -> Direction {
        let mut new_direction = direction;
        if params.2 {
            new_direction = new_direction.swap();

        }
        if params.1 {
            new_direction = new_direction.mirror_y();
        }
        if params.0 {
            new_direction = new_direction.mirror_x();
        }
        new_direction
    }

    #[test]
    fn test_none() {
        assert_eq!(Direction::Right, map_faces(Direction::Right, (false, false, false)));
        assert_eq!(Direction::Left, map_faces(Direction::Left, (false, false, false)));
        assert_eq!(Direction::Up, map_faces(Direction::Up, (false, false, false)));
        assert_eq!(Direction::Down, map_faces(Direction::Down, (false, false, false)));

        assert_eq!(Direction::Left, map_faces(Direction::Right, (false, true, false)));
        assert_eq!(Direction::Right, map_faces(Direction::Left, (false, true, false)));
        assert_eq!(Direction::Up, map_faces(Direction::Up, (false, true, false)));
        assert_eq!(Direction::Down, map_faces(Direction::Down, (false, true, false)));

        assert_eq!(Direction::Right, map_faces(Direction::Right, (true, false, false)));
        assert_eq!(Direction::Left, map_faces(Direction::Left, (true, false, false)));
        assert_eq!(Direction::Down, map_faces(Direction::Up, (true, false, false)));
        assert_eq!(Direction::Up, map_faces(Direction::Down, (true, false, false)));
    }

    #[test]
    fn test_flip() {
        assert_eq!(Direction::Right, map_faces(Direction::Up, (false, false, true)));
        assert_eq!(Direction::Left, map_faces(Direction::Down, (false, false, true)));
        assert_eq!(Direction::Down, map_faces(Direction::Left, (false, false, true)));
        assert_eq!(Direction::Up, map_faces(Direction::Right, (false, false, true)));
        assert_eq!(Direction::Left, map_faces(Direction::Up, (true, true, true)));
        assert_eq!(Direction::Right, map_faces(Direction::Down, (true, true, true)));
        assert_eq!(Direction::Up, map_faces(Direction::Left, (true, true, true)));
        assert_eq!(Direction::Down, map_faces(Direction::Right, (true, true, true)));
    }
}