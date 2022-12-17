use std::collections::{HashMap};
use std::time::Instant;

type Data = Vec<bool>;

const PIECES : [&[u8]; 5] = [
    &[0b0011110], // ####
    &[0b0001000, 0b0011100, 0b0001000], // +
    &[0b0011100, 0b0000100, 0b0000100], // Flipped L
    &[0b0010000, 0b0010000, 0b0010000, 0b0010000], // I
    &[0b0011000, 0b0011000], // []
];

pub fn run(data: Data) -> usize {
    let mut board  : Vec<u8> = vec![];

    let mut offset = 0;
    let mut instruction_pointer = 0;
    for p in 0..2022 {
        let mut piece = PIECES[p % PIECES.len()].clone().to_vec();

        for _ in 0..(3+piece.len()) {
            board.push(0);
        }
/*        for i in (0..board.len()).rev() {
            for j in (0..7).rev() {
                if board[i] & (1 << j) != 0 {
                    print!("{}", aoc_util::BLOCK_CHAR);
                } else {
                    print!("{}", aoc_util::EMPTY_CHAR);
                }
            }
            println!();
        }
        println!();*/
        let mut pointer = board.len() - piece.len(); // Bottom edge position

        'piece: loop {
            if data[instruction_pointer] {
                if piece.iter().enumerate().all(|(i, x)| (*x & 0b1000000 == 0) && ((board[pointer+i] & (*x << 1)) == 0)) {
                    for x in &mut piece {
                        *x <<= 1;
                    }
                }
            } else {
                if piece.iter().enumerate().all(|(i, x)| (*x & 0b0000001 == 0) && ((board[pointer+i] & (*x >> 1)) == 0)) {
                    for x in &mut piece {
                        *x >>= 1;
                    }
                }
            }
            instruction_pointer = (instruction_pointer + 1) % data.len();

            let mut collision = false;
            if pointer > 0 {
                for i in 0..piece.len() {
                    /*if (pointer - 1) + i >= board.len() {
                        break;
                    }*/
                    if board[(pointer - 1) + i] & piece[i] != 0 {
                        collision = true;
                        break;
                    }
                }
            } else {
                collision = true;
            }

            if collision {
                let mut adjustment_pointer = None;
                for i in 0..piece.len() {
                    board[pointer + i] |= piece[i];
                    if board[pointer + i] == 0b1111111 {
                        adjustment_pointer = Some(pointer + i);
                    }
                }

                if let Some(adjustment_pointer) = adjustment_pointer {
                    board = board[adjustment_pointer+1..].to_vec();
                    offset += adjustment_pointer + 1;
                }

                let mut len = board.len();
                while len > 0 && board[len - 1] == 0 {
                    len -= 1;
                }
                board.truncate(len);
                break 'piece;
            } else {
                pointer -= 1;
            }
        }
    }

    offset + board.len()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> usize {
    const TOTAL_STEPS : usize = 1000000000;
    let mut board  : Vec<u8> = vec![];

    let mut offset = 0;
    let mut instruction_pointer = 0;
    let mut map = HashMap::new();

    let mut p = 0;
    while p < TOTAL_STEPS {
        let mut piece = PIECES[p % PIECES.len()].clone().to_vec();

        for _ in 0..(3+piece.len()) {
            board.push(0);
        }
        let mut pointer = board.len() - piece.len(); // Bottom edge position

        'piece: loop {
            // Move left or right/check if we should move left or right.
            if data[instruction_pointer] {
                if piece.iter().enumerate().all(|(i, x)| (*x & 0b1000000 == 0) && ((board[pointer+i] & (*x << 1)) == 0)) {
                    for x in &mut piece {
                        *x <<= 1;
                    }
                }
            } else {
                if piece.iter().enumerate().all(|(i, x)| (*x & 0b0000001 == 0) && ((board[pointer+i] & (*x >> 1)) == 0)) {
                    for x in &mut piece {
                        *x >>= 1;
                    }
                }
            }
            instruction_pointer = (instruction_pointer + 1) % data.len();

            // Collision check when moving down
            let mut collision = false;
            if pointer > 0 {
                for i in 0..piece.len() {
                    if board[(pointer - 1) + i] & piece[i] != 0 {
                        collision = true;
                        break;
                    }
                }
            } else {
                collision = true;
            }

            // Place piece.
            if collision {
                for i in 0..piece.len() {
                    board[pointer + i] |= piece[i];
                }

                let mut len = board.len();
                while len > 0 && board[len - 1] == 0 {
                    len -= 1;
                }

                board.truncate(len);
                if offset == 0 {
                    let identifier = board_identifier(&board);
                    if let Some((last_p,  last_board_len)) = map.get(&(p % PIECES.len(), identifier, instruction_pointer)) {
                        // We got the exact same pattern
                        let diff = board.len() - last_board_len;
                        let diff_p = p - last_p;
                        let remaining = TOTAL_STEPS - p;
                        let remaining_p = remaining % diff_p;
                        p = TOTAL_STEPS - remaining_p;
                        offset = (remaining / diff_p) * diff;
                    } else {
                        map.insert((p % PIECES.len(), identifier, instruction_pointer), (p, board.len()));
                    }
                }
                break 'piece;
            } else {
                pointer -= 1;
            }
        }
        p += 1;
    }

    offset + board.len()
}

fn board_identifier(board: &Vec<u8>) -> [usize; 7] {
    let mut result = [0; 7];
    for i in 0..7 {
        let mut accumulator = 0;
        for j in 0..board.len() {
            accumulator |= board[board.len()-1-j];
            if accumulator & (1 << i) != 0 {
                result[i] = j;
                break;
            }
        }
    }
    result
}

type ConvertData<'a> = &'a [u8];

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| *x == b'<').collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data[0].as_bytes()
}