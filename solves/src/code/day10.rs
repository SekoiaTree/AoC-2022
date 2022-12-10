use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Noop,
    Addx(i64),
}

pub struct Computer {
    x: i64,
    pc: usize,
    instructions: Vec<Instruction>,
}

impl Computer {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Computer {
            x: 1,
            pc: 0,
            instructions,
        }
    }

    pub fn run(&mut self) -> i64 {
        let mut signal_strength = 0;
        let mut cycle_count = 1;
        //let mut strength_tester = 0;
        let mut strength_tester_absolute: usize = 20;
        while self.pc < self.instructions.len() {
            cycle_count += self.instruction_timing();
            // This is the general formula; in this case, n_hits is only equal to 1 or 0 so we don't care
/*            let n_hits = (cycle_count.saturating_sub(strength_tester_absolute) + 39) / 40;
            if n_hits != 0 {
                signal_strength += self.x * ((40 * n_hits * (n_hits + 2 * strength_tester - 1) / 2 + 20 * n_hits) as i64);
            }
            strength_tester += n_hits;
            strength_tester_absolute += n_hits * 40;*/
            if cycle_count > strength_tester_absolute {
                signal_strength += self.x * strength_tester_absolute as i64;
                strength_tester_absolute += 40;
            }
            self.run_instruction();
        }

        signal_strength
    }

    pub fn run_print(&mut self) {
        let mut cycle_count = 1;
        while self.pc < self.instructions.len() {
            let timing = self.instruction_timing();
            for i in 0..timing {
                if self.x + 1 >= 0 && ((cycle_count + i - 1).rem_euclid(40) + 1).abs_diff((self.x + 1) as usize) <= 1 {
                    print!("█");
                } else {
                    print!(" ");
                }
                if (cycle_count + i) % 40 == 0 {
                    println!();
                }
            }
            cycle_count += timing;
            self.run_instruction();
        }
        println!()
    }

    pub fn instruction_timing(&mut self) -> usize {
        match self.instructions[self.pc] {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2
        }
    }

    pub fn run_instruction(&mut self) {
        match self.instructions[self.pc] {
            Instruction::Noop => {}
            Instruction::Addx(x) => self.x += x
        }
        self.pc += 1
    }
}

type Data = Vec<Instruction>;

pub fn run(data: Data) -> i64 {
    let mut computer = Computer::new(data);
    computer.run()
}

#[cfg(feature = "part-two")]
pub fn run_step2(data: Data) -> () {
    let mut computer = Computer::new(data);
    computer.run_print()
}

type ConvertData<'a> = Vec<&'a [u8]>;

pub fn convert(data: ConvertData, _profiling: Instant) -> Data {
    data.iter().map(|x| {
        if x[0] == b'n' {
            Instruction::Noop
        } else {
            Instruction::Addx(String::from_utf8_lossy(&x[5..]).parse().unwrap())
        }
    }).collect()
}

pub fn free_convert(data: Vec<&str>) -> ConvertData {
    data.iter().map(|x| x.as_bytes()).collect()
}