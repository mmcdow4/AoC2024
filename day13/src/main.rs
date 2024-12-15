use std::fs;
use regex::Regex;

struct ClawMachine {
    prize: (f64, f64),
    a_button: (f64, f64),
    b_button: (f64, f64),
}

impl ClawMachine {
    fn new(prize: (i128, i128), a_button: (i128, i128), b_button: (i128, i128)) -> ClawMachine {
        ClawMachine {
            prize: (prize.0 as f64, prize.1 as f64),
            a_button: (a_button.0 as f64, a_button.1 as f64),
            b_button: (b_button.0 as f64, b_button.1 as f64),
        }
    }

    fn solve(&self) -> (usize, usize) {
        /* Invert the matrix [x_a x_b; y_a y_b], then multiply [prize_x; prize_y] */
        let determinant = (self.a_button.0 * self.b_button.1 - self.a_button.1 * self.b_button.0) as f64;
        if determinant.abs() > 0.0 {
            let a11 = self.b_button.1 / determinant;
            let a12 = -self.b_button.0 / determinant;
            let a21 = -self.a_button.1 / determinant;
            let a22 = self.a_button.0 / determinant;
    
            let a_presses = (a11 * self.prize.0 + a12 * self.prize.1).round() as usize;
            let b_presses = (a21 * self.prize.0 + a22 * self.prize.1).round() as usize;
    
            println!("Solution produced A:{a_presses} and B:{b_presses}");
            if self.verify((a_presses, b_presses)) {
                (a_presses, b_presses)
            } else {
                (0, 0)
            }
        } else {
            panic!("Unable to invert matrix [{} {}; {} {}]", self.a_button.0, self.b_button.0, self.a_button.1, self.b_button.1);
        }
    }

    fn verify(&self, solution: (usize, usize)) -> bool {
        let final_x = solution.0 * (self.a_button.0 as usize) + solution.1 * (self.b_button.0 as usize);
        let final_y = solution.0 * (self.a_button.1 as usize) + solution.1 * (self.b_button.1 as usize);

        final_x == (self.prize.0 as usize) && final_y == (self.prize.1 as usize)
    }
}

fn main() {

    let button_a_regex = Regex::new(r"Button A: X\+?(-?\d+), Y\+?(-?\d+)").unwrap();
    let button_b_regex = Regex::new(r"Button B: X\+?(-?\d+), Y\+?(-?\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut button_a = (0, 0);
    let mut button_b = (0, 0);
    let mut prize = (0, 0);
    let mut total_tokens = 0;
    let mut part_2_total_tokens = 0;
    let prize_offset = 10000000000000;
    for (index, line) in fs::read_to_string("E:\\dev\\AoC2024\\day13\\input.txt").unwrap().lines().enumerate() {
        match index % 4 {
            0 => {
                let matches = button_a_regex.captures(line).unwrap();
                button_a = (matches[1].parse().unwrap(), matches[2].parse().unwrap());
                //println!("parsed {} {} from {line}", button_a.0, button_a.1);
            },
            1 => {
                let matches = button_b_regex.captures(line).unwrap();
                button_b = (matches[1].parse::<i128>().unwrap(), matches[2].parse::<i128>().unwrap());
                //println!("parsed {} {} from {line}", button_b.0, button_b.1);
            },
            2 => {
                let matches = prize_regex.captures(line).unwrap();
                prize = (matches[1].parse::<i128>().unwrap(), matches[2].parse::<i128>().unwrap());
                //println!("parsed {} {} from {line}", prize.0, prize.1);
            },
            3 => {
                let machine = ClawMachine::new(prize, button_a, button_b);
                let (a_presses, b_presses) = machine.solve();
                // println!("A = [{} {}; {} {}], prize = [{}; {}], x = [{}; {}] has cost {}",
                //      button_a.0, button_b.0, button_a.1, button_b.1, prize.0, prize.1, a_presses, b_presses, a_presses * 3 + b_presses);
                total_tokens += a_presses * 3 + b_presses;

                let machine = ClawMachine::new((prize.0 + prize_offset, prize.1 + prize_offset), button_a, button_b);
                let (a_presses, b_presses) = machine.solve();
                part_2_total_tokens += a_presses * 3 + b_presses;
            },
            _ => unreachable!(),
        }
    }
    let machine = ClawMachine::new(prize, button_a, button_b);
    let (a_presses, b_presses) = machine.solve();
    // println!("A = [{} {}; {} {}], prize = [{}; {}], x = [{}; {}] has cost {}",
    //     button_a.0, button_b.0, button_a.1, button_b.1, prize.0, prize.1, a_presses, b_presses, a_presses * 3 + b_presses);
    total_tokens += a_presses * 3 + b_presses;

    let machine = ClawMachine::new((prize.0 + prize_offset, prize.1 + prize_offset), button_a, button_b);
    let (a_presses, b_presses) = machine.solve();
    part_2_total_tokens += a_presses * 3 + b_presses;
    println!("Total token cost is {total_tokens}, corrected cost is {part_2_total_tokens}");
}
