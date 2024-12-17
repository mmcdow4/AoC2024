use std::{fs, io::prelude::*};

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Box,
    WideBoxLeft,
    WideBoxRight,
    Robot,
    Empty,
}

#[derive(PartialEq, Debug)]
enum Move {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

struct Warehouse {
    map: Vec<Tile>,
    dimensions: (i32, i32),
    robot_pos: (i32, i32),
    step_index: usize,
}

impl Warehouse {
    fn new(map: Vec<Tile>, dimensions: (i32, i32), robot_pos: (i32, i32)) -> Warehouse {
        Warehouse { map, dimensions, robot_pos, step_index: 0 }
    }

    fn pos_to_index(&self, pos: (i32, i32)) -> usize {
        (pos.1 * self.dimensions.0 + pos.0) as usize
    }
    
    fn next_move(&mut self, next_move: Move) {
        let delta = match next_move {
            Move::NORTH => (0, -1),
            Move::EAST => (1, 0),
            Move::SOUTH => (0, 1),
            Move::WEST => (-1, 0),
        };

        let next_pos = (self.robot_pos.0 + delta.0, self.robot_pos.1 + delta.1);
        let robot_index = self.pos_to_index(self.robot_pos);
        if self.can_move(next_pos, delta) {
            self.move_tile(next_pos, &next_move, Tile::Robot);
            self.map[robot_index] = Tile::Empty;
            self.robot_pos = next_pos;
            self.step_index += 1;
        }
        
    }
 
    fn move_tile(&mut self, pos: (i32, i32), dir: &Move, incoming: Tile) {
        let main_index = self.pos_to_index(pos);
        match self.map[main_index] {
            Tile::Box => {
                let next_pos = match dir {
                    Move::NORTH => (pos.0, pos.1 - 1),
                    Move::EAST => (pos.0 + 1, pos.1),
                    Move::SOUTH => (pos.0, pos.1 + 1),
                    Move::WEST => (pos.0 - 1, pos.1),
                };
                self.move_tile(next_pos, dir, Tile::Box);
            },
            Tile::WideBoxLeft => {
                let right_pos = (pos.0 + 1, pos.1);
                let right_index = self.pos_to_index(right_pos);
                match dir {
                    Move::NORTH => {
                        /* Vertical move, handle both parts of the box */
                        let next_left = (pos.0, pos.1 - 1);
                        let next_right = (right_pos.0, right_pos.1 - 1);
                        self.map[main_index] = incoming;
                        self.map[right_index] = Tile::Empty;

                        self.move_tile(next_left, dir, Tile::WideBoxLeft);
                        self.move_tile(next_right, dir, Tile::WideBoxRight);
                    },
                    Move::EAST => {
                        /* Horizontal move */
                        let next_right = (right_pos.0 + 1, right_pos.1);
                        self.map[right_index] = Tile::WideBoxLeft;
                        self.move_tile(next_right, dir, Tile::WideBoxRight);
                    },
                    Move::SOUTH => {
                        /* Vertical move, handle both parts of the box */
                        let next_left = (pos.0, pos.1 + 1);
                        let next_right = (right_pos.0, right_pos.1 + 1);
                        
                        self.map[main_index] = incoming;
                        self.map[right_index] = Tile::Empty;

                        self.move_tile(next_left, dir, Tile::WideBoxLeft);
                        self.move_tile(next_right, dir, Tile::WideBoxRight);
                    },
                    Move::WEST => unreachable!("Should never push a left box half from the right!"),
                }
            },
            Tile::WideBoxRight => {
                let left_pos = (pos.0 - 1, pos.1);
                let left_index = self.pos_to_index(left_pos);
                match dir {
                    Move::NORTH => {
                        /* Vertical move, handle both parts of the box */
                        let next_right = (pos.0, pos.1 - 1);
                        let next_left = (left_pos.0, left_pos.1 - 1);
                        self.map[main_index] = incoming;
                        self.map[left_index] = Tile::Empty;

                        self.move_tile(next_left, dir, Tile::WideBoxLeft);
                        self.move_tile(next_right, dir, Tile::WideBoxRight);
                    },
                    Move::WEST => {
                        /* Horizontal move */
                        let next_left = (left_pos.0 - 1, left_pos.1);
                        self.map[left_index] = Tile::WideBoxRight;
                        self.move_tile(next_left, dir, Tile::WideBoxLeft);
                    },
                    Move::SOUTH => {
                        /* Vertical move, handle both parts of the box */
                        let next_right = (pos.0, pos.1 + 1);
                        let next_left = (left_pos.0, left_pos.1 + 1);
                        
                        self.map[main_index] = incoming;
                        self.map[left_index] = Tile::Empty;

                        self.move_tile(next_left, dir, Tile::WideBoxLeft);
                        self.move_tile(next_right, dir, Tile::WideBoxRight);
                    },
                    Move::EAST => unreachable!("Should never push a right box half from the left!"),
                }
            },
            Tile::Empty => {},
            _ => unreachable!("Shouldn't ever call move Box on a wall or robot tile"),
        }
        self.map[main_index] = incoming;
        //next_tiles
    }
    
    fn can_move(&self, next_pos: (i32, i32), delta: (i32, i32)) -> bool {
        match self.map[self.pos_to_index(next_pos)] {
            Tile::Empty => {
                true
            },
            Tile::Wall => {
                false
            },
            Tile::Box => {
                self.can_move((next_pos.0 + delta.0, next_pos.1 + delta.1), delta)
            },
            Tile::WideBoxLeft => {
                let right_pos = (next_pos.0 + 1, next_pos.1);
                if delta.1 != 0 {
                    /* moving veritcally, need to check both left and right */
                    self.can_move((next_pos.0, next_pos.1 + delta.1), delta) && self.can_move((right_pos.0, right_pos.1 + delta.1), delta)
                } else {
                    /* Must be pushing rightwards into a left Box, therefore just check the right part moving right */
                    self.can_move((right_pos.0 + delta.0, right_pos.1), delta)
                }
            },
            Tile::WideBoxRight => {
                let left_pos = (next_pos.0 - 1, next_pos.1);
                if delta.1 != 0 {
                    /* moving veritcally, need to check both left and right */
                    self.can_move((next_pos.0, next_pos.1 + delta.1), delta) && self.can_move((left_pos.0, left_pos.1 + delta.1), delta)
                } else {
                    /* Must be pushing leftwards into a right Box, therefore just check the left part moving left */
                    self.can_move((left_pos.0 + delta.0, left_pos.1), delta)
                }
            },
            Tile::Robot => unreachable!("You shouldn't every try to move into the Robot, YOU ARE THE Robot!"),
        }
    }

    fn compute_gps_sum(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                if self.map[self.pos_to_index((x, y))] == Tile::Box || self.map[self.pos_to_index((x, y))] == Tile::WideBoxLeft {
                    sum += (y as usize) * 100 + (x as usize);
                }
            }
        }

        sum
    }

    fn print_map(&self, filename: String) {
        let mut file = fs::File::create(filename).expect("Failed to open debug file");
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                match self.map[self.pos_to_index((x, y))] {
                    Tile::Empty => file.write(".".as_bytes()).expect("Failed to write to debug file"),
                    Tile::Wall => file.write("#".as_bytes()).expect("Failed to write to debug file"),
                    Tile::Box => file.write("O".as_bytes()).expect("Failed to write to debug file"),
                    Tile::WideBoxLeft => file.write("[".as_bytes()).expect("Failed to write to debug file"),
                    Tile::WideBoxRight => file.write("]".as_bytes()).expect("Failed to write to debug file"),
                    Tile::Robot => file.write("@".as_bytes()).expect("Failed to write to debug file"),
                };
            }
            file.write("\n".as_bytes()).expect("Failed to write to debug file");
        }
    }
}


fn main() {
    let mut map: Vec<Tile> = Vec::with_capacity(2500);
    let mut sequence: Vec<Move> = Vec::with_capacity(20000);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut robot_pos = (0, 0);
    for line in fs::read_to_string("E:\\dev\\AoC2024\\day15\\input.txt").unwrap().lines() {
        if line.contains('#') {
            max_x = 0;
            for ch in line.chars() {
                /* Part 1 parsing */
                // match ch {
                //     '#' => map.push(Tile::Wall),
                //     '.' => map.push(Tile::Empty),
                //     'O' => map.push(Tile::Box),
                //     '@' => {
                //         map.push(Tile::Robot);
                //         robot_pos = (max_x, max_y);
                //     },
                //     _ => break,
                // }
                /* Part 2 parsing */
                match ch {
                    '#' => {
                        map.push(Tile::Wall);
                        map.push(Tile::Wall);
                    },
                    '.' => {
                        map.push(Tile::Empty);
                        map.push(Tile::Empty);
                    },
                    'O' => {
                        map.push(Tile::WideBoxLeft);
                        map.push(Tile::WideBoxRight);
                    },
                    '@' => {
                        map.push(Tile::Robot);
                        map.push(Tile::Empty);
                        robot_pos = (max_x, max_y);
                    },
                    _ => break,
                }
                max_x += 2;
            }
            max_y += 1;
        }
        else if line.contains('^') {
            for ch in line.chars() {
                match ch {
                    '^' => sequence.push(Move::NORTH),
                    '>' => sequence.push(Move::EAST),
                    'v' => sequence.push(Move::SOUTH),
                    '<' => sequence.push(Move::WEST),
                    _ => break,
                }
            }
        }
    }

    let mut warehouse = Warehouse::new(map, (max_x, max_y), robot_pos);
    warehouse.print_map(String::from("E:\\dev\\AoC2024\\day15\\input_map.txt"));

    for movement in sequence {
        warehouse.next_move(movement);
    }
    warehouse.print_map(String::from("E:\\dev\\AoC2024\\day15\\final_map.txt"));
    let gps_sum = warehouse.compute_gps_sum();
    println!("GPS sum after all moves is {gps_sum}");
}
