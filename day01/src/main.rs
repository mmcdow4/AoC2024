use std::fs;

fn main() {
    let (mut left_column, mut right_column) = parse_input_file(&String::from(".\\input.txt"));


    left_column.sort();
    right_column.sort();

    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for idx in 0..left_column.len() {
        let left_value = left_column.get(idx).unwrap();
        total1 += left_value.abs_diff(*right_column.get(idx).unwrap());
        let cnt = right_column.iter().filter(|n| *n == left_value).count() as u32;

        total2 += cnt * left_value;
    }

    println!("Total difference is {total1}, Total modified sum is {total2}");
}

fn parse_input_file(filename: &String) -> (Vec<u32>, Vec<u32>) {
    let mut left_column = Vec::new();
    let mut right_column = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        let items: Vec<u32> = line.split(' ').map(|s| s.parse::<u32>().unwrap()).collect();
        left_column.push(items[0]);
        right_column.push(items[1]);
    }

    (left_column, right_column)
}