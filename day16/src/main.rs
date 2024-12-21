use std::{
    fs,
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Path,
    End,
    Start,
}

#[derive(Clone, PartialEq, Eq)]
struct Path {
    index: usize,
    direction: Direction,
    cost: usize,
    path: Vec<usize>,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.index.cmp(&other.index))
    }
}

struct Maze {
    maze: Vec<Tile>,
    visited: Vec<Path>,
    max_x: usize,
    start_pos: (usize, usize),
}

impl Maze {
    fn new(maze: Vec<Tile>, max_x: usize, start_pos: (usize, usize)) -> Maze {
        let num_tiles = maze.len();

        Maze {
            maze,
            visited: Vec::with_capacity(num_tiles),
            max_x,
            start_pos,
        }
    }

    fn pos_to_index(&self, pos: (usize, usize)) -> usize {
        pos.1 * self.max_x + pos.0
    }

    fn index_to_pos(&self, index: usize) -> (usize, usize) {
        let y = index / self.max_x;
        let x = index - y * self.max_x;
        (x, y)
    }

    fn next_step(&self, index: usize, direction: Direction) -> Option<usize> {
        let pos = self.index_to_pos(index);
        // println!("Index {index} is ({}, {}) -> Moving {:?}", pos.0, pos.1, direction);
        let next_index = match direction {
            Direction::East => self.pos_to_index((pos.0 + 1, pos.1)),
            Direction::South => self.pos_to_index((pos.0, pos.1 + 1)),
            Direction::West => self.pos_to_index((pos.0 - 1, pos.1)),
            Direction::North => self.pos_to_index((pos.0, pos.1 - 1)),
        };

        if self.maze[next_index] != Tile::Wall {
            Some(next_index)
        } else {
            None
        }
    }

    fn next_step_right(&self, index: usize, direction: Direction) -> Option<(usize, Direction)> {
        // println!("Turning right at {index} from direction {:?}", direction);
        let new_direction = match direction {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        };

        if let Some(index) = self.next_step(index, new_direction) {
            Some((index, new_direction))
        } else {
            None
        }
    }

    fn next_step_left(&self, index: usize, direction: Direction) -> Option<(usize, Direction)> {
        // println!("Turning left at {index} from direction {:?}", direction);
        let new_direction = match direction {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        };

        if let Some(index) = self.next_step(index, new_direction) {
            Some((index, new_direction))
        } else {
            None
        }
    }

    fn check_visited(&mut self, new_state: &Path) -> bool {
        for state in &mut self.visited {
            if new_state.index == state.index && new_state.direction == state.direction {
                if new_state.cost <= state.cost {
                    /* We've been here coming this direction before, but this is a lower cost, so replace the path */
                    state.cost = new_state.cost;
                    return true
                } else {
                    /* We've been here coming this direction before, and with a better cost, so don't allow the add */
                    return false
                }
            }
        }
        /* We've never been here coming this direction before, allow the add */
        self.visited.push(new_state.clone());
        true
    }

    fn run_maze(&mut self) -> (usize, usize) {
        let mut priority_queue = BinaryHeap::new();
        let mut best_path_spaces: HashSet<usize> = HashSet::new();
        let start_index = self.pos_to_index(self.start_pos);
        let mut best_cost = 88416;
        priority_queue.push(Path {index: start_index, direction: Direction::East, cost: 0, path: vec![start_index]});

        while let Some(Path {index, direction, cost, path}) = priority_queue.pop() {
            if self.maze[index] == Tile::End && cost == best_cost {
                // Reached the end with the best possible score
                println!("Reached the end with the best score along {} spaces", path.len());
                best_cost = cost;
                best_path_spaces.extend(&path);
                continue;
            }

            if let Some(next_index) = self.next_step(index, direction) {
                /* Try moving forward with cost 1 */
                let mut new_state = Path {index: next_index, direction, cost: cost + 1, path: path.clone()};
                if self.check_visited(&new_state) {
                    new_state.path.push(next_index);
                    priority_queue.push(new_state);
                }
            }
            if let Some((next_index, new_direction)) = self.next_step_right(index, direction) {
                /* Try turning right and moving forward with cost 1000 + 1 */
                let mut new_state = Path {index: next_index, direction: new_direction, cost: cost + 1001, path: path.clone()};
                if self.check_visited(&new_state) {
                    new_state.path.push(next_index);
                    priority_queue.push(new_state);
                }
            }
            if let Some((next_index, new_direction)) = self.next_step_left(index, direction) {
                /* Try turning left and moving forward with cost 1000 + 1 */
                let mut new_state = Path {index: next_index, direction: new_direction, cost: cost + 1001, path: path.clone()};
                if self.check_visited(&new_state) {
                    new_state.path.push(next_index);
                    priority_queue.push(new_state);
                }
            }
        }

        (best_cost, best_path_spaces.len())
    }
}

fn main() {
    
    let mut x = 0;
    let mut y = 0;
    let mut start_pos = (0, 0);
    let mut maze = Vec::with_capacity(20164);
    for line in fs::read_to_string("E:\\dev\\AoC2024\\day16\\input.txt").unwrap().lines() {
        x = 0;
        for ch in line.chars() {
            match ch {
                '#' => maze.push(Tile::Wall),
                '.' => maze.push(Tile::Path),
                'S' => {
                    start_pos = (x, y);
                    maze.push(Tile::Start);
                },
                'E' => maze.push(Tile::End),
                _ => break,
            }
            x += 1;
        }
        y += 1;
    }

    let mut maze = Maze::new(maze, x, start_pos);

    //maze.print_path(&String::from("input_read.txt"));
    let (best_score, num_spaces) = maze.run_maze();

    println!("Best score acheived was {best_score}, with {num_spaces} unique spaces");
}

