use std::{env, fs, io::prelude::*, collections::VecDeque};

#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Safe,
    Corrupted,
}

struct Ram {
    memory: Vec<State>,
    order: Vec<usize>,
    next_idx: usize,
    dimensions: (usize, usize)
}

fn pos_to_index(pos: (usize, usize), max_x: usize) -> usize {
    pos.1 * max_x + pos.0
}

fn index_to_pos(index: usize, max_x: usize) -> (usize, usize) {
    let x = index % max_x;
    let y = index / max_x;
    (x, y)
}

impl Ram {
    fn new(dimensions: (usize, usize), order: Vec<usize>) -> Ram {
        Ram {
            memory: vec![State::Safe; dimensions.0 * dimensions.1],
            order,
            next_idx: 0,
            dimensions,
        }
    }

    fn last_byte(&self) -> (usize, usize) {
        index_to_pos(self.order[self.next_idx - 1], self.dimensions.0)
    }

    fn drop_next_n(&mut self, n: usize) {
        let mut counter = 0;
        while (counter < n) && (self.next_idx < self.order.len()) {
            self.memory[self.order[self.next_idx]] = State::Corrupted;
            self.next_idx += 1;
            counter += 1;
        }
    }

    fn find_best_path(&mut self) -> usize {
        let mut visited =  vec![false; self.dimensions.0 * self.dimensions.1];
        let mut path_points = VecDeque::new();
        path_points.push_back((0, 0, 0));

        while !path_points.is_empty() {
            let current_pos = path_points.pop_front().unwrap();

            if current_pos.0 == self.dimensions.0 -1 && current_pos.1 == self.dimensions.1 -1 {
                return current_pos.2;
            }

            
            if current_pos.0 > 0 {
                let next_idx = pos_to_index((current_pos.0 - 1, current_pos.1), self.dimensions.0);
                if self.memory[next_idx] == State::Safe && !visited[next_idx] {
                    visited[next_idx] = true;
                    path_points.push_back((current_pos.0 - 1, current_pos.1, current_pos.2 + 1));
                }
            }
            if current_pos.1 > 0 {
                let next_idx = pos_to_index((current_pos.0, current_pos.1 - 1), self.dimensions.0);
                if self.memory[next_idx] == State::Safe && !visited[next_idx] {
                    visited[next_idx] = true;
                    path_points.push_back((current_pos.0, current_pos.1 - 1, current_pos.2 + 1));
                }
            }
            if current_pos.0 < self.dimensions.0 - 1 {
                let next_idx = pos_to_index((current_pos.0 + 1, current_pos.1), self.dimensions.0);
                if self.memory[next_idx] == State::Safe && !visited[next_idx] {
                    visited[next_idx] = true;
                    path_points.push_back((current_pos.0 + 1, current_pos.1, current_pos.2 + 1));
                }
            }
            if current_pos.1 < self.dimensions.1 - 1 {
                let next_idx = pos_to_index((current_pos.0, current_pos.1 + 1), self.dimensions.0);
                if self.memory[next_idx] == State::Safe && !visited[next_idx] {
                    visited[next_idx] = true;
                    path_points.push_back((current_pos.0, current_pos.1 + 1, current_pos.2 + 1));
                }
            }
        }

        usize::MAX
    }

    fn print_memory(&self, filename: &String) {
        let mut file = fs::File::create(filename).expect(format!("Unable to open debug file {filename}").as_str());

        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                let index = pos_to_index((x, y), self.dimensions.0);
                match self.memory[index] {
                    State::Safe => _ = file.write(".".as_bytes()).expect(format!("Unable to write to debug file {filename}").as_str()),
                    State::Corrupted => _ = file.write("#".as_bytes()).expect(format!("Unable to write to debug file {filename}").as_str()),
                }
            }
            file.write("\n".as_bytes()).expect(format!("Unable to write to debug file {filename}").as_str());
        }
    }
}
fn main() {
    let args_list: Vec<String> = env::args().collect();

    let dimensions: (usize, usize) = (args_list[1].parse().unwrap(), args_list[2].parse().unwrap());
    let input = format!("E:\\dev\\AoC2024\\day18\\{}", args_list[3].clone());
    let n = args_list[4].parse().unwrap();

    let mut order = Vec::with_capacity(3450);
    for line in fs::read_to_string(input).unwrap().lines() {
        let next_pos: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
        let next_idx = pos_to_index((next_pos[0], next_pos[1]), dimensions.0);
        order.push(next_idx);
    }

    let mut ram = Ram::new(dimensions, order);

    ram.drop_next_n(n);
    ram.print_memory(&format!("E:\\dev\\AoC2024\\day18\\input_after_{n}.txt"));

    let mut best_path = ram.find_best_path();

    println!("Best path found was {} steps", best_path);

    while best_path < usize::MAX {
        ram.drop_next_n(1);
        best_path = ram.find_best_path();
    }

    let last_byte = ram.last_byte();
    println!("First byte to make the exit unreachable is ({}, {})", last_byte.0, last_byte.1);

}
