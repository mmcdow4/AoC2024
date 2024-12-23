use std::{
    env,
    fs,
    collections::{HashMap, VecDeque, HashSet},
};


#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Path,
    End,
    Start,
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn turn(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {x, y}
    }

    fn to_index(&self, max_x: usize) -> usize {
        (self.y / max_x) + self.x
    }

    fn step(&self, direction: &Direction, max_x: usize, max_y: usize) -> Option<Pos> {
        match direction {
            Direction::North => {
                if self.y > 0 {
                    return Some(Pos::new(self.x, self.y - 1));
                }
            },
            Direction::East => {
                if self.x < max_x - 1 {
                    return Some(Pos::new(self.x + 1, self.y));
                }
            },
            Direction::South => {
                if self.y < max_y - 1 {
                    return Some(Pos::new(self.x, self.y + 1));
                }
            },
            Direction::West => {
                if self.x > 0 {
                    return Some(Pos::new(self.x - 1, self.y));
                }
            },
        }

        None
    }
    
}
struct Course {
    course: Vec<Tile>,
    visited: Vec<usize>,
    cheats: HashMap<usize, Vec<usize>>,
    max_x: usize,
    max_y: usize,
    start_pos: Pos,
}

impl Course {
    fn new(course: Vec<Tile>, max_x: usize, max_y: usize,  start_pos: Pos) -> Course {
        let num_tiles = course.len();

        Course {
            course,
            visited: vec![0; num_tiles],
            cheats: HashMap::with_capacity(num_tiles),
            max_x,
            max_y,
            start_pos,
        }
    }

    fn next_step(&self, pos: &Pos) -> Pos {
        let mut current_dir = Direction::West;
        for _ in 0..4 {
            /* Try moving in this direction */
            if let Some(new_pos) = pos.step(&current_dir, self.max_x, self.max_y) {
                let next_index = new_pos.to_index(self.max_x);
                if self.course[next_index] != Tile::Wall && self.visited[next_index] == 0 {
                    return new_pos
                }
            }

            current_dir = turn(current_dir);
        }
        
        panic!("No next step was found for ({}, {})", pos.x, pos.y);
    }

    fn find_cheats(&mut self, pos: &Pos, current_ps: usize) {
        let mut pos_queue = VecDeque::new();
        let mut checked_cheats = HashSet::new();
        pos_queue.push_back((0, false, pos.clone()));
        while let Some((ps_added, mut through_wall, current_pos)) = pos_queue.pop_front() {
            let current_idx = current_pos.to_index(self.max_x);
            checked_cheats.insert(current_pos);
            if through_wall && self.course[current_idx] != Tile::Wall && self.visited[current_idx] == 0 {
                /* This is not a wall, and has not been visited already */
                self.cheats
                    .entry(current_idx)
                    .or_insert(vec![])
                    .push(current_ps + ps_added);
            } else if self.course[current_idx] == Tile::Wall {
                through_wall = true;
            }

            if ps_added < 20 {
                let mut new_direction = Direction::East;
                for _ in 0..4 {
                    if let Some(new_pos) =
                        current_pos
                        .step(&new_direction, self.max_x, self.max_y) {
                            if !checked_cheats.contains(&new_pos) {
                                pos_queue.push_back((ps_added + 1, through_wall, new_pos));
                                checked_cheats.insert(new_pos);
                            }
                    }
                    new_direction = turn(new_direction);
                }
            }
        }
    }

    fn run_course(&mut self) -> usize {
        let mut current_pos = self.start_pos;
        let mut ps_counter = 1;
        loop {
            let current_index = current_pos.to_index(self.max_x);
            self.visited[current_index] = ps_counter;
            if self.course[current_index] == Tile::End {
                break;
            }

            self.find_cheats(&current_pos, ps_counter);
            current_pos = self.next_step(&current_pos);
            ps_counter += 1;
        }


        let mut num_good_cheats = 0;
        for (index, cheat_times) in &self.cheats {
            for time in cheat_times {
                if self.visited[*index] >= 100 && *time <= self.visited[*index] - 100 {
                    num_good_cheats += 1;
                }
            }
        }
        num_good_cheats
    }
}

fn main() {
    let args_list: Vec<String> = env::args().collect();
    let input = &args_list[1];

    let mut x = 0;
    let mut y = 0;
    let mut start_pos = Pos::new(0, 0);
    let mut course = Vec::with_capacity(19600);
    for line in fs::read_to_string(&format!("E:\\dev\\AoC2024\\day20\\{input}")).unwrap().lines() {
        x = 0;
        for ch in line.chars() {
            match ch {
                '#' => course.push(Tile::Wall),
                '.' => course.push(Tile::Path),
                'S' => {
                    start_pos = Pos::new(x, y);
                    course.push(Tile::Start);
                },
                'E' => course.push(Tile::End),
                _ => break,
            }
            x += 1;
        }
        y += 1;
    }

    println!("Found course dimensions {y} x {x}");
    let mut course = Course::new(course, x, y, start_pos);

    let num_good_cheats = course.run_course();

    println!("Number of cheats that save 100ps or more is {num_good_cheats}");
}
