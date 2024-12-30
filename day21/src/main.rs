use std::{
    collections::{HashMap, VecDeque},
    env,
    fs,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos {x, y}
    }
}

fn hash(start_pos: &Pos, end_pos: &Pos, num_robots: usize) -> u64 {
    let mut hash_val = start_pos.x as u64 * 4;
    hash_val += start_pos.y as u64;
    hash_val *= 4;
    hash_val += end_pos.x as u64;
    hash_val *= 4;
    hash_val += end_pos.y as u64;
    hash_val *= 30;
    hash_val += num_robots as u64;
    hash_val
}

fn cheapest_dir_pad(start_pos: &Pos, end_pos: &Pos, num_robots: usize, dirpad: &HashMap<char, Pos>, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(cheapest_path) = cache.get(&hash(start_pos, end_pos, num_robots)) {
        return *cheapest_path;
    }
    let mut cheapest_path = u64::MAX;
    let mut q = VecDeque::new();
    q.push_back((start_pos.clone(), String::from("")));
    while let Some((curr_pos, mut curr_sequence)) = q.pop_front() {
        if curr_pos == *end_pos {
            curr_sequence += &"A";
            let result = cheapest_robot(&curr_sequence, num_robots - 1, dirpad, cache);
            cheapest_path = cheapest_path.min(result);
        } else {
            if curr_pos.x != 0 || curr_pos.y != 0 {
                if curr_pos.x > end_pos.x {
                    q.push_back((Pos::new(curr_pos.x - 1, curr_pos.y), curr_sequence.clone() + &"<"));
                } else if curr_pos.x < end_pos.x {
                    q.push_back((Pos::new(curr_pos.x + 1, curr_pos.y), curr_sequence.clone() + &">"));
                }
                if curr_pos.y > end_pos.y {
                    q.push_back((Pos::new(curr_pos.x, curr_pos.y - 1), curr_sequence.clone() + &"^"));
                } else if curr_pos.y < end_pos.y {
                    q.push_back((Pos::new(curr_pos.x, curr_pos.y + 1), curr_sequence.clone() + &"v"));
                }
            }
        }
    }
    cache.insert(hash(start_pos, end_pos, num_robots), cheapest_path);
    cheapest_path
}

fn cheapest_robot(sequence: &String, num_robots: usize, dirpad: &HashMap<char, Pos>, cache: &mut HashMap<u64, u64>) -> u64 {
    if num_robots == 1 {
        return sequence.len() as u64;
    }

    let mut result = 0;

    let mut curr_pos = dirpad.get(&'A').unwrap();

    for ch in sequence.chars() {
        result += cheapest_dir_pad(&curr_pos, dirpad.get(&ch).unwrap(), num_robots, dirpad, cache);
        curr_pos = dirpad.get(&ch).unwrap();
    }

    result
}

fn cheapest_sequence(start_key: char, end_key: char, num_robots: usize, number_pad: &HashMap<char, Pos>, dirpad: &HashMap<char, Pos>, cache: &mut HashMap<u64, u64>) -> u64 {
    let mut cheapest_path = u64::MAX;
    let mut q = VecDeque::new();
    q.push_back((number_pad.get(&start_key).unwrap().clone(), String::from("")));
    let end_pos = number_pad.get(&end_key).unwrap().clone();
    while let Some((curr_pos, mut curr_sequence)) = q.pop_front() {
        if curr_pos == end_pos {
            curr_sequence += &"A";
            cheapest_path = cheapest_path.min(cheapest_robot(&curr_sequence, num_robots, &dirpad, cache));
        } else if curr_pos != *number_pad.get(&'X').unwrap() {
            if curr_pos.x > end_pos.x {
                q.push_back((Pos::new(curr_pos.x - 1, curr_pos.y), curr_sequence.clone() + &"<"));
            } else if curr_pos.x < end_pos.x {
                q.push_back((Pos::new(curr_pos.x + 1, curr_pos.y), curr_sequence.clone() + &">"));
            }
            if curr_pos.y > end_pos.y {
                q.push_back((Pos::new(curr_pos.x, curr_pos.y - 1), curr_sequence.clone() + &"^"));
            } else if curr_pos.y < end_pos.y {
                q.push_back((Pos::new(curr_pos.x, curr_pos.y + 1), curr_sequence.clone() + &"v"));
            }
        }
    }

    cheapest_path
}
fn main() {
    let args_list: Vec<String> = env::args().collect();
    let input = &args_list[1];
    let num_robots = args_list[2].parse().unwrap();
    let number_pad: HashMap<char, Pos> = 
        [('7', Pos::new(0, 0)),
        ('8', Pos::new(1, 0)),
        ('9', Pos::new(2, 0)),
        ('4', Pos::new(0, 1)),
        ('5', Pos::new(1, 1)),
        ('6', Pos::new(2, 1)),
        ('1', Pos::new(0, 2)),
        ('2', Pos::new(1, 2)),
        ('3', Pos::new(2, 2)),
        ('X', Pos::new(0, 3)),
        ('0', Pos::new(1, 3)),
        ('A', Pos::new(2, 3))]
        .iter().cloned().collect();
    
    let dir_pad: HashMap<char, Pos> = 
        [('X', Pos::new(0, 0)),
        ('^', Pos::new(1, 0)),
        ('A', Pos::new(2, 0)),
        ('<', Pos::new(0, 1)),
        ('v', Pos::new(1, 1)),
        ('>', Pos::new(2, 1))]
        .iter().cloned().collect();
    
    let mut cache = HashMap::new();
    let mut net_complexity = 0;
    for line in fs::read_to_string(
            &format!("E:\\dev\\AoC2024\\day21\\{input}")
            ).unwrap().lines() {
        
        println!("Looking for command sequence to produce {line}:");
        let mut sequence_length = 0;
        let mut prev_char = 'A';
        for ch in line.chars() {
            /* Get the moves required  */
            sequence_length += cheapest_sequence(prev_char, ch, num_robots, &number_pad, &dir_pad, &mut cache);
            prev_char = ch;
        }

        let number = line[0..line.len()-1].parse::<u64>().unwrap();
        net_complexity += number * sequence_length;
        println!("Sequence is {sequence_length} commands long, therefore complexity {}", number * sequence_length);
    }

    println!("Total complexity is {net_complexity}");
}
