use std::collections::VecDeque;
use std::time::Instant;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Old,
    Constant(u64),
}

impl Value {
    pub fn get_value(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Constant(x) => *x
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Add(Value, Value),
    Multiply(Value, Value),
}

impl Operation {
    pub fn get_result(&self, worry: u64) -> u64 {
        match self {
            Operation::Add(x, y) => x.get_value(worry) + y.get_value(worry),
            Operation::Multiply(x, y) => x.get_value(worry) * y.get_value(worry)
        }
    }

    pub fn from_str(string: &str) -> Self {
        let mut parts = string.split_whitespace();
        let x = parts.next().unwrap();
        let op = parts.next().unwrap();
        let y = parts.next().unwrap();
        let x = if x == "old" {
            Value::Old
        } else {
            Value::Constant(x.parse().unwrap())
        };
        let y = if y == "old" {
            Value::Old
        } else {
            Value::Constant(y.parse().unwrap())
        };
        match op {
            "+" => Operation::Add(x, y),
            "*" => Operation::Multiply(x, y),
            _ => panic!("Unknown operation {}", op)
        }
    }
}

pub struct MonkeyBuilder {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    case_true: usize,
    case_false: usize,
    activity: usize,
}

impl MonkeyBuilder {
    pub fn new() -> Self {
        Self {
            items: vec![],
            operation: Operation::Add(Value::Constant(0), Value::Constant(0)),
            test: 0,
            case_true: 0,
            case_false: 0,
            activity: 0,
        }
    }

    pub fn set_items(mut self, items: Vec<u64>) -> Self {
        self.items = items;
        self
    }

    pub fn set_operation(mut self, operation: Operation) -> Self {
        self.operation = operation;
        self
    }

    pub fn set_test(mut self, test: u64) -> Self {
        self.test = test;
        self
    }

    pub fn set_case_true(mut self, case_true: usize) -> Self {
        self.case_true = case_true;
        self
    }

    pub fn set_case_false(mut self, case_false: usize) -> Self {
        self.case_false = case_false;
        self
    }

    pub fn set_activity(mut self, activity: usize) -> Self {
        self.activity = activity;
        self
    }

    pub fn build(self) -> Monkey {
        Monkey {
            items: self.items,
            operation: self.operation,
            test: self.test,
            case_true: self.case_true,
            case_false: self.case_false,
            activity: self.activity,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    case_true: usize,
    case_false: usize,
    activity: usize,
}

impl Monkey {
    pub fn new(items: Vec<u64>, operation: Operation, test: u64, case_true: usize, case_false: usize) -> Self {
        Self {
            items,
            operation,
            test,
            case_true,
            case_false,
            activity: 0,
        }
    }

    pub fn catch(&mut self, worry: u64) {
        self.items.push(worry);
    }

    fn throw_to(&self, monkeys_before: &mut [Monkey], monkeys_after: &mut [Monkey], my_index: usize, monkey: usize, value: u64) {
        if monkey < my_index {
            monkeys_before[monkey].catch(value);
        } else {
            monkeys_after[monkey - my_index - 1].catch(value);
        }
    }

    pub fn throw_items(&mut self, monkeys_before: &mut [Monkey], monkeys_after: &mut [Monkey], my_index: usize) {
        for i in &self.items {
            let new = self.operation.get_result(*i) / 3;
            if new % self.test == 0 {
                self.throw_to(monkeys_before, monkeys_after, my_index, self.case_true, new);
            } else {
                self.throw_to(monkeys_before, monkeys_after, my_index, self.case_false, new);
            }
        }
        self.activity += self.items.len();
        self.items.clear();
    }

    pub fn throw_items_step_2(&mut self, monkeys_before: &mut [Monkey], monkeys_after: &mut [Monkey], my_index: usize, modulus: u64) {
        for i in &self.items {
            let new = self.operation.get_result(*i) % modulus;
            if new % self.test == 0 {
                self.throw_to(monkeys_before, monkeys_after, my_index, self.case_true, new);
            } else {
                self.throw_to(monkeys_before, monkeys_after, my_index, self.case_false, new);
            }
        }
        self.activity += self.items.len();
        self.items.clear();
    }
}

type Data = Vec<Monkey>;

pub fn run(mut data: Data) -> usize {
    for _ in 0..20 {
        for m in 0..data.len() {
            let (monkeys_before, monkeys_after) = data.split_at_mut(m);
            let (monkey, monkeys_after) = monkeys_after.split_at_mut(1);
            monkey[0].throw_items(monkeys_before, monkeys_after, m);
        }
    }
    let first = data.iter().position_max_by_key(|m| m.activity).unwrap();
    let first_activity = data[first].activity;
    data[first].activity = 0;
    let second_activity = data.iter().max_by_key(|m| m.activity).unwrap().activity;
    first_activity * second_activity
}

#[cfg(feature = "part-two")]
pub fn run_step2(mut data: Data) -> usize {
    let lcm: u64 = data.iter().map(|x| x.test).product();
    for _ in 0..10000 {
        for m in 0..data.len() {
            let (monkeys_before, monkeys_after) = data.split_at_mut(m);
            let (monkey, monkeys_after) = monkeys_after.split_at_mut(1);
            monkey[0].throw_items_step_2(monkeys_before, monkeys_after, m, lcm);
        }
    }
    let first = data.iter().position_max_by_key(|m| m.activity).unwrap();
    let first_activity = data[first].activity;
    data[first].activity = 0;
    let second_activity = data.iter().max_by_key(|m| m.activity).unwrap().activity;
    first_activity * second_activity
}

type ConvertData<'a> = Vec<&'a str>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    let mut monkeys = vec![];
    for i in data.chunks(7) {
        let monkey_builder = MonkeyBuilder::new()
            .set_items(i[1][18..].split(", ").map(|x| x.parse::<u64>().unwrap()).collect())
            .set_operation(Operation::from_str(&i[2][19..]))
            .set_test(i[3][21..].parse().unwrap())
            .set_case_true(i[4][29..].parse().unwrap())
            .set_case_false(i[5][30..].parse().unwrap());
        monkeys.push(monkey_builder.build());
    }
    monkeys
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data
}