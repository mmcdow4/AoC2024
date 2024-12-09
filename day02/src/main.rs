use std::fs;

fn main() {
    let safe_count = count_safe_reports(&String::from(".\\input.txt"));

    println!("Safe report count: {safe_count}");
}

fn count_safe_reports(filename: &String) -> u32 {
    let mut count = 0;
    let mut line_count = 0;

    for line in fs::read_to_string(filename).unwrap().lines() {
        let items: Vec<i32> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
        
        let mut safe = test_report(&items);
        let mut ignore_idx: usize = 0;
        while !safe && ignore_idx < items.len() {
            let mut temp_vec = vec!();
            if ignore_idx == 0 {
                temp_vec.extend_from_slice(&items[1..items.len()]);
            }
            else if ignore_idx == items.len() - 1 {
                temp_vec.extend_from_slice(&items[0..items.len()-1]);
            }
            else {
                temp_vec.extend_from_slice(&items[0..ignore_idx]);
                //println!("TP0: ignoring index {} temp_vec is now {} items long", ignore_idx, temp_vec.len());
                temp_vec.extend_from_slice(&items[ignore_idx+1..items.len()]);
                //println!("TP1: ignoring index {} temp_vec is now {} items long", ignore_idx, temp_vec.len());
            }
            //println!("TP2: report under test is {:?}", temp_vec);
            safe = test_report(&temp_vec);

            if safe {
                println!("line {line_count} made safe by ignoring index {ignore_idx}");
            }
            ignore_idx += 1;
        }
        if safe {
            count += 1;
        }
        line_count += 1;
    }

    count
}

fn test_report(report: &Vec<i32>) -> bool{
    let mut safe = true;
    let mut prev_value = report[0];
    let mut sign = 0;
    for idx in 1..report.len() {
        let delta = report[idx] - prev_value;
        
        if sign == 0 {
            sign = delta.signum();
        }
        if delta.signum() != sign || delta.abs() > 3 || delta == 0  {
            /* already failed once */
            safe = false;
            break;
        }
        prev_value = report[idx];
    }

    safe
}