use std::{
    fs,
    env,
    collections::{HashMap, HashSet, VecDeque},
    io::{prelude::*, BufReader},
};
use regex::Regex;

#[derive(Debug)]
enum Function {
    And,
    Or,
    Xor,
}

struct Gate {
    input_a: String,
    input_b: String,
    function: Function,
}

impl Gate {
    fn new(input_a: &str, input_b: &str, function: Function) -> Gate {
        // println!("Creating gate for {input_a} {:?} {input_b} => {output}", function);
        Gate {
            input_a: input_a.to_string(),
            input_b: input_b.to_string(),
            function,
        }
    }

    fn compute(&self, wires: &HashMap<String, Option<bool>>) -> Option<bool> {
        // println!("Trying to compute {} {:?} {} => {}", self.input_a, self.function, self.input_b, self.output);
        if wires[&self.input_a].is_some() && wires[&self.input_b].is_some() {
            let input_a = wires[&self.input_a].unwrap();
            let input_b = wires[&self.input_b].unwrap();
            return match self.function {
                Function::And => Some(input_a && input_b),
                Function::Or => Some(input_a || input_b),
                Function::Xor => Some(input_a ^ input_b),
            };
        }
        None
    }
}

fn compute_gates(gates: &mut HashMap<String, Gate>, wires: &mut HashMap<String, Option<bool>>, max_z: u64) -> bool {
    
    let mut new_wires = HashMap::with_capacity(wires.len());

    for (output, gate) in gates {
        new_wires.insert(output, gate.compute(wires));
    }

    for (wire, value) in new_wires {
        wires.entry(wire.to_string()).and_modify(|v| *v = value);
    }
    
    for z_idx in 0..=max_z {
        let z_str = format!("z{z_idx:02}");
        if wires[&z_str].is_none() {
            return false;
        }
    }

    return true
}

fn get_z_value(wires: &HashMap<String, Option<bool>>, max_z: u64) -> u64 {
    let mut z_value = 0;
    for z_idx in 0..=max_z {
        let z_str = format!("z{z_idx:02}");
        // println!("checking z_idx {z_idx} with string {z_str} : {}", wires[&z_str].unwrap());
        if wires[&z_str].unwrap() {
            // println!("Adding {} i.e. 1 << {z_idx} to zvalue", 1 << z_idx);
            z_value += 1 << z_idx;
        }
    }
    z_value
}

fn print_wires(wires: &HashMap<String, Option<bool>>, index: usize) {
    let mut file = fs::File::create(&format!("E:\\dev\\AoC2024\\day24\\output{index:02}.txt")).unwrap();
    for (wire, value) in wires {
        match value {
            Some(true) => _ = file.write(&format!("{wire} : 1\n").as_bytes()).expect("Error writing to debuf file"),
            Some(false) => _ = file.write(&format!("{wire} : 0\n").as_bytes()).expect("Error writing to debuf file"),
            None => _ = file.write(&format!("{wire} : ?\n").as_bytes()).expect("Error writing to debuf file"),
        }
    }
}

fn reset_wires(wires: &mut HashMap<String, Option<bool>>, test_bit: usize) {
    for (wire_label, wire_value) in wires {
        if wire_label.starts_with(&"x") || wire_label.starts_with(&"y") {
            if wire_label[1..].parse::<usize>().unwrap() == test_bit {
                *wire_value = Some(true);
            } else {
                *wire_value = Some(false);
            }
        } else {
            *wire_value = None;
        }
    }
}
fn main() {
    let argslist: Vec<String> = env::args().collect();
    let input_filename = &argslist[1];

    let file = fs::File::open(&format!("E:\\dev\\AoC2024\\day24\\{input_filename}")).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(...) (.+) (...) -> (...)").unwrap();

    let mut wires = HashMap::new();
    let mut gates = HashMap::new();
    let mut max_z: u64 = 0;
    let mut x_value: u64 = 0;
    let mut y_value: u64 = 0;
    for line in reader.lines() {
        let line = line.unwrap().replace("\n", "");
        if line.is_empty() {
            continue;
        }

        let matches = re.captures(&line);
        if let Some(match_strings) = matches {
            let input_a = &match_strings[1];
            let input_b = &match_strings[3];
            let output = &match_strings[4];
            let function = match &match_strings[2] {
                "AND" => Function::And,
                "XOR" => Function::Xor,
                "OR" => Function::Or,
                _ => unreachable!("Unexpected function string!"),
            };
            wires.entry(output.to_string()).or_insert(None);
            if output.starts_with("z") {
                max_z = max_z.max(output[1..3].parse().unwrap());
            }
            gates.insert(output.to_string(), Gate::new(input_a, input_b, function));
        } else {
            let items: Vec<&str> = line.split(": ").collect();
            if items[1] == "0" {
                wires.insert(items[0].to_string(), Some(false));
            } else if items[1] == "1" {
                if items[0].starts_with("x") {
                    let index = items[0][1..=2].parse::<u64>().unwrap();
                    x_value += 1 << index;
                } else if items[0].starts_with("y") {
                    let index = items[0][1..=2].parse::<u64>().unwrap();
                    y_value += 1 << index;
                }
                wires.insert(items[0].to_string(), Some(true));
            }
        }
    }

    let expected_z = x_value + y_value;
    println!("Found max_z {max_z}, looking to produce {x_value}  + {y_value} = {expected_z}");
    let mut clock_idx = 0;
    print_wires(&wires, clock_idx);
    /* Part 1 */
    while !compute_gates(&mut gates, &mut wires, max_z) {
        clock_idx += 1;
        print_wires(&wires, clock_idx);
    }
    clock_idx += 1;
    print_wires(&wires, clock_idx);
    let z_value = get_z_value(&wires, max_z);
    println!("Z value is {z_value}");

    /* Part 2 */
    for test_bit in 0..=44 {
        reset_wires(&mut wires, test_bit);
        while !compute_gates(&mut gates, &mut wires, max_z) {
            continue;
        }
        let z_value = get_z_value(&wires, max_z);
        let expected_z = 2 << test_bit;
        let mut bad_bits: Vec<u64> = Vec::new();
        let discrepancy = z_value ^ expected_z;
        if discrepancy > 0 {
            for z_index in 0..=max_z {
                if (discrepancy >> z_index) & 0x1 > 0 {
                    bad_bits.push(z_index);
                }
            }
            println!("Test bit: {test_bit}, Bad bits are {:?}", bad_bits);
        }
    }
}
