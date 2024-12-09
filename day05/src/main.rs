use std::fs;

fn main() {
    let (rules_vec, updates_vec) = parse_input(&String::from("E:\\dev\\advent_of_code_2024\\day05\\input.txt"));

    let mut total = 0;
    let mut corrected_total = 0;
    let mut update_idx = 0;
    for mut update in updates_vec {
        let mut correct = true;
        let mut rerun = true;
        while rerun {
            rerun = false;
        for rule in &rules_vec {
            let idx1 = update.iter().position(|x| *x == rule.0);
            let idx2 = update.iter().position(|x| *x == rule.1);

                if idx1.is_some() && idx2.is_some() && idx1.unwrap() > idx2.unwrap() {
                    let idx1 = idx1.unwrap();
                    let idx2 = idx2.unwrap();
                    correct = false;
                    rerun = true;
                    let mut temp_copy = vec!();
                    /* go until just before the second item */
                    if idx2 > 0 {
                        temp_copy.extend_from_slice(&update[0..idx2]);
                    }
                    /* Insert the first item */
                    temp_copy.push(rule.0);
                    /* Go until the original position of hte first item */
                    temp_copy.extend_from_slice(&update[idx2..idx1]);
                    /* Go until the end */
                    if idx1 + 1 < update.len() {
                        temp_copy.extend_from_slice(&update[(idx1+1)..]);
                    }

                    //println!("Transformed {:?} into {:?} to satisfy rule '{}|{}'", update, temp_copy, rule.0, rule.1);
                    update = temp_copy.clone();
                }
            }
        }

        let midpoint = (update.len() - 1) / 2;
        if correct {
            /* Add the middle number to the total */
            total += update[midpoint];
        } else {
            //println!("Corrected update {update_idx} into {:?}", update);
            corrected_total += update[midpoint];
        }
        update_idx += 1;
    }

    println!("Sum of correct midpoints is {total}");
    println!("Sum of corrected midpoints is {corrected_total}");
}

fn parse_input(filename: &String) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules_vec = Vec::new();
    let mut updates_vec = Vec::new();

    let mut reading_rules = true;
    for line in fs::read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            reading_rules = false;
        }
        else if reading_rules {
            let pages: Vec<&str> = line.split('|').collect();
            rules_vec.push((pages[0].parse::<u32>().unwrap(), pages[1].parse::<u32>().unwrap()));
        }
        else {
            updates_vec.push(line.split(',').map(|s| s.parse::<u32>().unwrap()).collect());
        }
    }

    (rules_vec, updates_vec)
}