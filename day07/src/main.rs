use std::{fs, fmt};

#[derive(Clone, Copy)]
enum Operation {
    PLUS,
    TIMES,
    CAT
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Operation::PLUS => 0,
            Operation::TIMES => 1,
            Operation::CAT => 2,
        };
        write!(f, "{val}")
    }
}

fn increment_operation(operator: &Operation) -> Option<Operation> {
    match operator {
        Operation::PLUS => Some(Operation::TIMES),
        Operation::TIMES => Some(Operation::CAT),
        Operation::CAT => None,
    }
}
struct Equation {
    output: u128,
    inputs: Vec<u128>,
    operations: Vec<Operation>,
}

impl Equation {
    fn new(line: &str) -> Equation {
        let in_and_out: Vec<&str> = line.split(':').collect();
        let output = in_and_out[0].parse::<u128>().unwrap();
        // let inputs_str: Vec<&str> = in_and_out[1].split(' ').filter(|s| !s.is_empty()).collect();
        // println!("Split the inputs from {} to {:?}", in_and_out[1], inputs_str);
        let inputs: Vec<u128> = in_and_out[1].split(' ').filter(|s| !s.is_empty()).map(|s| s.trim().replace("\r\n", "").parse::<u128>().unwrap()).collect();
        let operations: Vec<Operation> = vec![Operation::PLUS; inputs.len() - 1];
        Equation {
            output,
            inputs,
            operations,
        }
    }

    fn output(&self) -> u128 {
        self.output
    }

    fn next_permutation(&mut self) -> bool {
        let mut operator_index = 0;
        //println!("permutation starting at {:?}", self.operations);
        while operator_index < self.operations.len() {
            match increment_operation(&self.operations[operator_index]) {
                Some(x) => {
                    self.operations[operator_index] = x;
                    //println!("permutation updated to {:?}", self.operations);
                    return true;
                },
                None => {
                    self.operations[operator_index] = Operation::PLUS;
                    operator_index += 1;
                }
            }
        }
        return false;
    }

    fn compute(a: u128, b: u128, operator: Operation) -> u128 {
        match operator {
            Operation::PLUS => a + b,
            Operation::TIMES => a * b,
            Operation::CAT => {
                //(a.to_string() + &b.to_string()).parse::<u128>().unwrap()
                let offset = f64::log10(b as f64 + 0.01f64).ceil() as u32;
                a * u128::pow(10u128, offset) + b
            },
        }
    }

    fn test_permutation(&self) -> bool {
        let mut result = Equation::compute(self.inputs[0], self.inputs[1], self.operations[0]);
        for index in 1..self.operations.len() {
            result = Equation::compute(result, self.inputs[index+1], self.operations[index]);
            
            if result > self.output {
                return false;
            }
        }

        //println!("Result of applying {:?} is : {}", self.operations, result);
        result == self.output
    }
}

fn main() {

    let mut calibration_sum = 0;

    for line in fs::read_to_string(String::from("E:\\dev\\AoC2024\\day07\\input.txt")).unwrap().lines() {
        let mut equation = Equation::new(line);

        loop {
            if equation.test_permutation() {
                calibration_sum += equation.output();
                break
            }
            else if !equation.next_permutation() {
                break;
            }
        }
    }

    println!("Final calibration sum is {calibration_sum}");
}
