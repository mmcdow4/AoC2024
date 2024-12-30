use std::{fs, env, collections::HashMap};

fn test_key_in_lock(key: &[i32], lock: &[i32]) -> bool {
    for index in 0..5 {
        if key[index] + lock[index] > 5 {
            return false;
        }
    }
    true
}
fn main() {
    let argslist: Vec<String> = env::args().collect();
    let input_file = &argslist[1];

    let mut locks = HashMap::new();
    let mut keys = HashMap::new(); 

    let mut pins: [i32; 5]  = [0; 5];
    let mut is_lock = true;
    let mut first_line = true;
    let mut num_locks = 0;
    let mut num_keys = 0;

    for line in fs::read_to_string(&format!("E:\\dev\\AoC2024\\day25\\{input_file}")).unwrap().lines() {
        if !line.starts_with("#") && !line.starts_with(".") {
            if is_lock {
                // println!("Recording a lock: {:?}", pins);
                *locks.entry(pins.clone()).or_insert(0) += 1;
                num_locks += 1;
            } else {
                // println!("Recording a key: {:?}", pins);
                *keys.entry(pins.clone()).or_insert(0) += 1;
                num_keys += 1;
            }
            first_line = true;
        } else {
            if first_line {
                if line.starts_with('#') {
                    is_lock = true;
                    pins.fill(0);
                } else {
                    is_lock = false;
                    pins.fill(-1);
                }
                first_line = false;
            } else {
                for (index, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        pins[index] += 1;
                    }
                }
            }
        }
    }

    println!("Read {num_locks} locks and {num_keys} keys");
    let mut key_lock_combos = 0;
    for (lock, lock_count) in &locks { 
        for (key, key_count) in &keys {
            if test_key_in_lock(key, lock) {
                key_lock_combos += lock_count * key_count;
            }
        }
    }

    println!("There are {key_lock_combos} combinations of keys and locks");
}
