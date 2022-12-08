use std::time::Instant;

/*
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn wins(&self, other: Move) -> bool {
        match self {
            Move::Rock => other == Move::Scissors,
            Move::Paper => other == Move::Rock,
            Move::Scissors => other == Move::Paper,
        }
    }

    pub fn loses(&self, other: Move) -> bool {
        match self {
            Move::Rock => other == Move::Paper,
            Move::Paper => other == Move::Scissors,
            Move::Scissors => other == Move::Rock,
        }
    }

    pub fn parse(input: &str) -> Move {
        match input {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid input: {}", input),
        }
    }

    pub fn winner(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    pub fn loser(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

type Data = Vec<(Move, String)>;

pub fn run(data: Data) -> u32 {
    data.iter().map(|(their_move, my_move)| -> u32 {
        let my_move = Move::parse(my_move);
        return if my_move.wins(*their_move) { 6 } else if my_move.loses(*their_move) { 0 } else { 3 } + my_move.score();
    }).sum()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> u32 {
    data.iter().map(|(their_move, my_move)|
        match &**my_move {
            "X" => their_move.winner().score()+0,
            "Y" => their_move.score()+3,
            "Z" => their_move.loser().score()+6,
            _ => panic!("Invalid input: {}", my_move),
        }).sum()
}

pub fn convert(data: Vec<&str>, _profiling: Instant) -> Data {
    data.iter().map(|line| {
        let (first, second) = line.split_once(" ").unwrap();
        let first = Move::parse(first);
        (first, second.to_string())
    }).collect()
}*/

type Data = Vec<(i32, i32)>;

pub fn run(data: Data) -> i32 {
    data.iter().map(|(their_move, my_move)|  (my_move-their_move+1).rem_euclid(3)*3 + my_move + 1).sum()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> i32 {
    data.iter().map(|(their_move, my_move)| (their_move+my_move-1).rem_euclid(3)+1+my_move*3).sum()
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|line| {
        (line[0] as i32-'A' as i32, line[2] as i32-'X' as i32)
    }).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.iter().map(|s| s.as_bytes()).collect()
}