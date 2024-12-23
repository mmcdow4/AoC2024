use std::{fs, env, collections::{VecDeque, HashMap, HashSet}};

struct Deltas {
    deltas: VecDeque<String>,
    output_strings: HashSet<String>,
}

impl Deltas {
    fn new() -> Deltas {
        Deltas { deltas: VecDeque::with_capacity(4), output_strings: HashSet::new() }
    }

    fn as_string(&mut self) -> Option<String> {
        if self.deltas.len() == 4 {
            let output = format!("{},{},{},{}", self.deltas[0], self.deltas[1], self.deltas[2], self.deltas[3]);
            if !self.output_strings.contains(&output) {
                self.output_strings.insert(output.clone());
                return Some(output);
            }
        }
        None
    }

    fn append(&mut self, delta: i64) {
        self.deltas.push_back(delta.to_string());
        if self.deltas.len() == 5 {
            _ = self.deltas.pop_front();
        }
    }
}

fn next_secret_number(secret_number: i64) -> i64 {
    let mut next_number = (secret_number ^ (secret_number << 6)) % 16777216;

    next_number = (next_number ^ (next_number >> 5)) % 16777216;

    next_number = (next_number ^ (next_number << 11)) % 16777216;

    next_number
}

fn main() {
    let args_list: Vec<String> = env::args().collect();
    let input = &args_list[1];

    let mut total = 0;

    let mut possible_totals: HashMap<String, i64> = HashMap::new();
    for line in fs::read_to_string(&format!("E:\\dev\\AoC2024\\day22\\{input}")).unwrap().lines() {
        let mut prev_number = line.parse::<i64>().unwrap();
        let mut deltas = Deltas::new();
        // println!("Starting number: {prev_number}, price {}", prev_number % 10);
        for _ in 0..2000 {
            let new_number = next_secret_number(prev_number);
            // println!("New number {new_number}: price {}", new_number % 10);
            deltas.append((new_number % 10) - (prev_number % 10));
            if let Some(sequence) = deltas.as_string() {
                *possible_totals.entry(sequence).or_insert(0) += new_number % 10;
                // println!("Delta are now {sequence}, adding price price {} to bring total to {}", new_number % 10, possible_totals[&sequence]);
            }
            prev_number = new_number;
        }
        // println!("2000th number: {secret_number}");
        total += prev_number;
    }

    let mut best_sequence = String::new();
    let mut best_total = 0;
    for (sequence, total) in possible_totals {
        if total > best_total {
            best_total = total;
            best_sequence = sequence;
        }
    }
    println!("The total sum of all 2000th secret numbers is {total}");
    println!("The best profit {best_total} can be made with sequence {best_sequence}");
}
