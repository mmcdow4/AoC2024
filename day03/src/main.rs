use std::fs;

const READING_MUL: usize = 0;
const READING_A: usize = 1;
const READING_B: usize = 2;
const NUMBER_STATES: usize = 3;

fn main() {
    let mut state: usize = READING_MUL;

    let mut expected_function = "mul(";
    let mut err_string = String::new();
    let mut total = 0;
    let mut A = 0;
    let mut B = 0;
    let mut enabled = true;
    for next_char in fs::read_to_string(&String::from(".\\input.txt")).unwrap().chars() {
        let mut err = true;
        let mut next_state = false;
        //if enabled {
            match state {
                READING_MUL => {
                    if expected_function.starts_with(next_char)
                    {
                        err = false;
                        expected_function = &expected_function[1..];
                        next_state = expected_function.is_empty();
                    }
                },
                READING_A => {
                    if next_char.is_ascii_digit() {
                        err = false;
                        A = 10 * A + next_char.to_digit(10).unwrap();
                    } else if next_char == ',' {
                        err = false;
                        next_state = true;
                    }
                },
                READING_B => {
                    if next_char.is_ascii_digit() {
                        err = false;
                        B = 10 * B + next_char.to_digit(10).unwrap();
                    } else if next_char == ')' {
                        err = false;
                        next_state = true;
                        if enabled {
                        let prod = A * B;
                        println!("Found a mul command: ({A} * {B}) = {prod}");
                        total += prod;
                        }
                    }
                },
                _ => panic!("Unknown state {state}!")
            }

            if err {
                state = READING_MUL;
                expected_function = "mul(";
                A = 0;
                B = 0;
                err_string.push(next_char);
                if err_string.ends_with("do()") {
                    println!("Enabled ON");
                    enabled = true;
                    err_string.clear();
                } else if err_string.ends_with("don't()") {
                    println!("Enabled OFF");
                    enabled = false;
                    err_string.clear();
                }
            } else if next_state {
                state = (state + 1) % NUMBER_STATES;
                err_string.clear();
                if state == READING_MUL {
                    expected_function = "mul(";
                    A = 0;
                    B = 0;
                }
            }
        // }
        // else {
        //     err_string.push(next_char);
        //     if err_string.ends_with("do()") {
        //         println!("Enabled ON");
        //         enabled = true;
        //         err_string.clear();
        //     } else if err_string.ends_with("don't()") {
        //         println!("Enabled OFF");
        //         enabled = false;
        //         err_string.clear();
        //     }
        // }
    }

    println!("Final total: {total}");
}
