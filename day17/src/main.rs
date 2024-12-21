use std::{
    fs,
    io::{prelude::*, BufReader},
};
use regex::Regex;

struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instr_ptr: usize,
    output: Vec<i64>,
}

impl Computer {
    fn new(register_a: i64, register_b: i64, register_c: i64) -> Computer {
        Computer {
            register_a,
            register_b,
            register_c,
            instr_ptr: 0,
            output: Vec::new(),
        }
    }

    fn reset(&mut self, register_a: i64) {
        self.register_a = register_a;
        self.register_b = 0;
        self.register_c = 0;
    }

    fn combo_operand(&self, operand: i64) -> i64 {
        match operand {
            0 => 0, 
            1 => 1,
            2 => 2, 
            3 => 3,
            4 => self.register_a, 
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!("Invalid adv operand {operand}"),
        }
    }

    /* Opcode 0 */
    fn adv(&mut self, operand: i64) -> usize {
        self.register_a >>= self.combo_operand(operand);
        self.instr_ptr += 2;
        self.instr_ptr
    }

    /* Opcode 1 */
    fn bxl(&mut self, operand: i64) -> usize {
        self.register_b ^= operand as i64;
        self.instr_ptr += 2;
        self.instr_ptr
    }

    /* Opcode 2 */
    fn bst(&mut self, operand: i64) -> usize {
        self.register_b = self.combo_operand(operand) & 0x7 ;
        self.instr_ptr += 2;
        self.instr_ptr
    }

    /* Opcode 3 */
    fn jnz(&mut self, operand: i64) -> usize {
        if self.register_a == 0 {
            self.instr_ptr += 2;
        } else {
            self.instr_ptr = operand as usize;
        }
        self.instr_ptr
    }

    /* Opcode 4 */
    fn bxc(&mut self, _operand: i64) -> usize {
        self.register_b ^= self.register_c;
        self.instr_ptr += 2;
        self.instr_ptr
    }

    /* Opcode 5 */
    fn out(&mut self, operand: i64) -> usize {
        let new_output = self.combo_operand(operand) & 0x7;
        self.output.push(new_output);
        // if !self.output.is_empty() {
        //     self.out += format!(",{}", new_output).as_str();
        // } else {
        //     self.out += format!("{}", new_output).as_str();
        // }
        self.instr_ptr += 2;
        self.instr_ptr
    }

    /* Opcode 6 */
    fn bdv(&mut self, operand: i64) -> usize {
        self.register_b = self.register_a >> self.combo_operand(operand);
        self.instr_ptr += 2;
        self.instr_ptr
    }

    /* Opcode 7 */
    fn cdv(&mut self, operand: i64) -> usize {
        self.register_c = self.register_a >> self.combo_operand(operand);
        self.instr_ptr += 2;
        self.instr_ptr
    }

    fn execute(&mut self, program: &Vec<i64>) -> Vec<i64> {
        self.output.clear();
        self.instr_ptr = 0;
        while self.instr_ptr < program.len() {
            let operand = program[self.instr_ptr + 1];
            let opcode = program[self.instr_ptr];
            self.instr_ptr = match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => unreachable!("Illegal opcode {opcode}"),
            }
        }

        self.output.clone()
    }

}

fn main() {
    let file = fs::File::open("E:\\dev\\AoC2024\\day17\\input.txt").expect("Could not open input file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read Register A line");
    let re = Regex::new(r"Register A: (\d+)").unwrap();
    let register_a: i64 = re
        .captures(&line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    line.clear();
    reader.read_line(&mut line).expect("Failed to read Register B line");
    let re = Regex::new(r"Register B: (\d+)").unwrap();
    let register_b: i64 = re
        .captures(&line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    line.clear();
    reader.read_line(&mut line).expect("Failed to read Register C line");
    let re = Regex::new(r"Register C: (\d+)").unwrap();
    let register_c: i64 = re
        .captures(&line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    line.clear();
    reader.read_line(&mut line).expect("Failed to read empty line");
    line.clear();
    reader.read_line(&mut line).expect("Failed to read program line");
    let program_string = line.replace("Program: ", "").replace("\n\r", "");
    let program: Vec<i64> = program_string
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut computer = Computer::new(register_a, register_b, register_c);

    /* Part 1 */
    let output = computer.execute(&program);
    println!("Output as initialized = {:?}", output);

    /* Part 2 */
    println!("Looking for register A initial value to produce {:?}", program);
    let mut a_candidates: Vec<i64> = vec![0];
    for index in (0..program.len()).rev() {
        let item = program[index];
        println!("looking to produce next bits {item}");
        let mut new_candidates = Vec::new();
        for a in a_candidates {
            for bottom_bits in 0..8 {
                let temp_a = (a << 3) + bottom_bits;
                computer.reset(temp_a);
                let output = computer.execute(&program);
                if output == program[index..] {
                    println!("Program produces {item} for a {temp_a}");
                    new_candidates.push(temp_a);
                }
            }
        }
        a_candidates = new_candidates;
    }

    println!("Now verifying the {} candidates", a_candidates.len());
    a_candidates.sort();
    for a in a_candidates {
        computer.reset(a);
        let output = computer.execute(&program);
        if output == program {
            println!("Program reproduced itself for register A initialized to {a}: {:?}", output);
            break;
        }
    }
    // println!("Looking for a register A value to reproduce ({program_string})");
    // //let mut a = 0;
    // //To output 16 digits (the length of the program), A must be zero after shifting 48 bits right,
    // // but nonzero after shifting 45 bits right. This provides a range of possible values to try

    // for a in 40212449000000..0xFFFFFFFFFFFF {//0x249249249249..0xFFFFFFFFFFFF {
    //     if a % 1000000000 == 0 {
    //         println!("Checking A = {a}");
    //     }
    //     computer.reset(a);
    //     let output = computer.execute(&program, true /* quit early at the first mismatch in output */);
    //     if !output.is_empty() { 
    //         if output.contains(&program_string) {
    //             println!("Program reproduced itself for register A initialized to {a}: {output}");
    //             //break;
    //         } else {
    //             println!("Partial match for register A initialized to {a}: {output}");
    //         }
    //     }
    //     //a += 1;
    // }
}
