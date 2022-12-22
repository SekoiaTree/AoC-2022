use std::collections::HashMap;
use std::ops::Div;
use std::time::Instant;
use polynomen::{One, Poly, Zero};

type Data = HashMap<String, Operation>;

#[derive(Clone, Hash)]
pub enum Operation {
    Plus(String, String),
    Minus(String, String),
    Times(String, String),
    Divide(String, String),
    Equals(String, String),
    Number(i64),
    Human
}

impl Operation {
    pub fn to_equals(self) -> Operation {
        match self {
            Operation::Plus(x, y) => Operation::Equals(x, y),
            Operation::Minus(x, y) => Operation::Equals(x, y),
            Operation::Times(x, y) => Operation::Equals(x, y),
            Operation::Divide(x, y) => Operation::Equals(x, y),
            Operation::Equals(x, y) => Operation::Equals(x, y),
            Operation::Number(_) => Operation::Number(-1),
            Operation::Human => Operation::Equals("aaaa".to_string(), "aaaa".to_string())
        }
    }

    pub fn calculate(&self, map: &Data) -> Option<i64> {
        match self {
            Operation::Plus(x, y) => {
                let x = map.get(x)?;
                let y = map.get(y)?;
                Some((*x).calculate(map)? + (*y).calculate(map)?)
            }
            Operation::Minus(x, y) => {
                let x = map.get(x)?;
                let y = map.get(y)?;
                Some((*x).calculate(map)? - (*y).calculate(map)?)
            }
            Operation::Times(x, y) => {
                let x = map.get(x)?;
                let y = map.get(y)?;
                Some((*x).calculate(map)? * (*y).calculate(map)?)
            }
            Operation::Divide(x, y) => {
                let x = map.get(x)?;
                let y = map.get(y)?;
                Some((*x).calculate(map)? / (*y).calculate(map)?)
            }
            Operation::Number(x) => Some(*x),
            Operation::Equals(x, y) => {
                let x = map.get(x)?;
                let y = map.get(y)?;
                Some(if (*x).calculate(map)? == (*y).calculate(map)? { 1 } else { 0 })
            }
            Operation::Human => None,
        }
    }

    pub fn propagate_humn(&self, map: &Data) -> Poly<f64> {
        match self {
            Operation::Plus(x, y) => {
                let x = map.get(x).unwrap();
                let y = map.get(y).unwrap();
                let x = x.propagate_humn(&map);
                let y = y.propagate_humn(&map);

                x + y
            }
            Operation::Minus(x, y) => {
                let x = map.get(x).unwrap();
                let y = map.get(y).unwrap();
                let x = x.propagate_humn(&map);
                let y = y.propagate_humn(&map);

                x - y
            }
            Operation::Times(x, y) => {
                let x = map.get(x).unwrap();
                let y = map.get(y).unwrap();
                let x = x.propagate_humn(&map);
                let y = y.propagate_humn(&map);

                x * y
            }
            Operation::Divide(x, y) => {
                let x = map.get(x).unwrap();
                let y = map.get(y).unwrap();
                let x = x.propagate_humn(&map);
                let y = y.propagate_humn(&map);

                if y.coeffs().len() > 1 {
                    panic!("Tried to divide with a non-monomial");
                }
                x / y.coeffs()[0]
            }
            Operation::Equals(a, b) => {
                let left =  map.get(a).unwrap().propagate_humn(&map);
                let right = map.get(b).unwrap().propagate_humn(&map);
                left-right
            }
            Operation::Number(x) => Poly::one() * (*x as f64),
            Operation::Human => {
                Poly::new_from_coeffs(&[0.0, 1.0])
            }
        }
    }
}

pub fn run(data: Data) -> i64 {
    data.get(&"root".to_string()).unwrap().calculate(&data).unwrap()
}

#[cfg(feature = "part-two")]
pub fn run_step2(mut data: Data) -> i64 {
    let root = data.remove(&"root".to_string()).unwrap();
    data.insert("root".to_string(), root.to_equals());
    data.insert("humn".to_string(), Operation::Human);
    let poly = data.get(&"root".to_string()).unwrap().propagate_humn(&data);
    // println!("{:?}", poly.real_roots().unwrap());
    poly.real_roots().unwrap()[0].round() as i64
}

type ConvertData<'a> = Vec<&'a str>;
// type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().fold(HashMap::new(), |mut map, x| {
        let (name, operation) = x.split_once(": ").unwrap();
        let operation = if let Ok(x) = operation.parse::<i64>() {
            Operation::Number(x)
        } else {
            let actual_operation = operation.chars().nth(5).unwrap();
            let left = operation[..4].to_string();
            let right = operation[7..].to_string();
            match actual_operation {
                '+' => Operation::Plus(left, right),
                '-' => Operation::Minus(left, right),
                '*' => Operation::Times(left, right),
                '/' => Operation::Divide(left, right),
                _ => panic!("Illegal operation {}", actual_operation)
            }
        };
        map.insert(name.to_string(), operation);
        map
    })
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
}