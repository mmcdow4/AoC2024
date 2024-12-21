use std::{env,
    fs,
    io::{BufRead, BufReader},
    collections::HashMap,
    cmp,
};

fn is_pattern_possible(pattern: &String,
    towel_types: &Vec<String>,
    cache: &mut HashMap<String, usize>) -> usize /*Option<usize>*/ {
    if cache.contains_key(pattern) {
        return *cache.get(pattern).unwrap();
    }
    let mut combinations = 0;
    for towel in towel_types {
        if pattern.starts_with(towel) {
            if pattern.len() == towel.len() {
                // Towel exactly matches the pattern
                combinations += 1;
            } else {
                combinations += is_pattern_possible(
                    &pattern[towel.len()..].to_string(),
                    towel_types,
                    cache
                );
            }
        }
    }
    cache.insert(pattern.clone(), combinations);
    return combinations;
}

fn main() {
    let args_list: Vec<String> = env::args().collect();
    let input = &args_list[1];

    let file = fs::File::open(
        format!("E:\\dev\\AoC2024\\day19\\{input}")
    ).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect(
        "Unable to read towels from input"
    );
    line = line.replace("\r\n", "");
    let mut towel_types: Vec<String> = line.split(", ")
        .map(|s| s.to_string())
        .collect();
    towel_types.sort_by(|a, b|
        {
            if a.len() > b.len() {
                cmp::Ordering::Less
            } else if a.len() < b.len() {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        }
    );

    let mut num_possible = 0;
    let mut num_combinations = 0;
    let mut cache: HashMap<String, usize> = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Failed reading line from pattern list");
        let combinations = is_pattern_possible(&line, &towel_types, &mut cache);
        if combinations > 0 {
            println!("Pattern {line} is posssible in {combinations} ways");
            num_possible += 1;
            num_combinations += combinations;
        } else {
            println!("Pattern {line} is impossible");
        }
    }

    println!("{num_possible} patterns are possible, in a total of {num_combinations} combinations");
}
