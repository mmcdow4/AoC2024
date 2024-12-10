use std::fs;

const MAX_X: usize = 130;
const MAX_Y: usize = 130;

#[derive(PartialEq, Clone, Copy)]
enum GuardDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

#[derive(Clone, Copy)]
struct Tile {
    pub occupied: bool,
    pub loop_counted: bool,
    pub visited_north: bool,
    pub visited_east: bool,
    pub visited_south: bool,
    pub visited_west: bool,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            occupied: false,
            loop_counted: false,
            visited_north: false,
            visited_east: false,
            visited_south: false,
            visited_west: false,
        }
    }

    fn visited(&self) -> bool {
        self.visited_north || self.visited_east || self.visited_south || self.visited_west
    }

    fn update_visited(&mut self, dir: &GuardDirection) {
        match dir {
            GuardDirection::NORTH => self.visited_north = true,
            GuardDirection::EAST => self.visited_east = true,
            GuardDirection::SOUTH => self.visited_south = true,
            GuardDirection::WEST => self.visited_west = true,
        }
    }
}

#[derive(Clone, Copy)]
struct Guard {
    pos: (usize, usize),
    dir: GuardDirection,
}

impl Guard {
    fn new(pos: (usize, usize), dir: GuardDirection) -> Guard {
        Guard {
            pos,
            dir,
        }
    }

    fn pos(&self) -> (usize, usize) {
        self.pos
    }

    fn dir(&self) -> &GuardDirection {
        &self.dir
    }

    fn dir_as_str(&self) -> String {
        match self.dir {
            GuardDirection::NORTH => String::from("N"),
            GuardDirection::EAST => String::from("E"),
            GuardDirection::SOUTH => String::from("S"),
            GuardDirection::WEST => String::from("W"),
        }
    }

    fn turn(&mut self) {
        match self.dir {
            GuardDirection::NORTH => self.dir = GuardDirection::EAST,
            GuardDirection::EAST => self.dir = GuardDirection::SOUTH,
            GuardDirection::SOUTH => self.dir = GuardDirection::WEST,
            GuardDirection::WEST => self.dir = GuardDirection::NORTH,
        }
    }

    fn next_step(&self) -> Option<(usize, usize)> {
        match self.dir{
            GuardDirection::NORTH => { 
                if self.pos.1 == 0 {
                    None
                } else {
                    Some((self.pos.0, self.pos.1-1))
                }
            },
            GuardDirection::EAST => { 
                if self.pos.0+1 == MAX_X {
                    None
                } else {
                    Some((self.pos.0+1, self.pos.1))
                }
            },
            GuardDirection::SOUTH => { 
                if self.pos.1+1 == MAX_Y {
                    None
                } else {
                    Some((self.pos.0, self.pos.1+1))
                }
            },
            GuardDirection::WEST => { 
                if self.pos.0 == 0 {
                    None
                } else {
                    Some((self.pos.0-1, self.pos.1))
                }
            },
        }
    }

    fn been_here_before(&self, tile: &Tile) -> bool {
        match self.dir {
            GuardDirection::NORTH => tile.visited_north,
            GuardDirection::EAST => tile.visited_east,
            GuardDirection::SOUTH => tile.visited_south,
            GuardDirection::WEST => tile.visited_west,
        }
    }

    fn advance(&mut self) {
        self.pos = self.next_step().unwrap();
    }
}

fn main() {
    let (mut loc_array, mut guard) = parse_input(&String::from("E:\\dev\\AoC2024\\day06\\input.txt"));

    let mut space_count = 1;
    let mut loop_locations = 0;
    let mut next_pos = guard.next_step();
    //println!("The guard is starting at ({}, {}) and facing '{}'", guard.pos().0, guard.pos().1, guard.dir_as_str());
    while next_pos.is_some() {
        let (next_x, next_y) = next_pos.unwrap();
        if loc_array[next_x][next_y].occupied {
            // Next position is occupied, turn 90 degrees instead
            guard.turn();
        } else {
            // Check if turning rather than advancing would creat a loop
            if !loc_array[next_x][next_y].loop_counted && !loc_array[next_x][next_y].visited() &&
                    check_for_loop(&mut loc_array.clone(), &mut guard.clone())
            {
                loc_array[next_x][next_y].loop_counted = true;
                loop_locations += 1;
            }
            // Next position is unoccupied, advance
            guard.advance();
            if !loc_array[next_x][next_y].visited() {
                // Advancing into a previously unvisited location
                space_count += 1;
            }
        }
        loc_array[guard.pos.0][guard.pos.1].update_visited(guard.dir());

        //println!("The guard is now at ({}, {}) and facing '{}'", guard.pos().0, guard.pos().1, guard.dir_as_str());
        next_pos = guard.next_step();
    }

    println!("The guard visited {space_count} unique locations, and can be forced into a loop with an obstacle at {loop_locations} unique locations");

}


// Function to check if turning now would create an infinite loop
fn check_for_loop(loc_array: &mut [[Tile; MAX_Y]; MAX_X], guard: &mut Guard) -> bool {
    loc_array[guard.next_step().unwrap().0][guard.next_step().unwrap().1].occupied = true;
    //guard.turn();
    loc_array[guard.pos.0][guard.pos.1].update_visited(guard.dir());
    let mut next_pos = guard.next_step();

    while next_pos.is_some() {
        let (next_x, next_y) = next_pos.unwrap();
        if loc_array[next_x][next_y].occupied {
            // Next position is occupied, turn 90 degrees instead
            guard.turn();
        } else {
            // Next position is unoccupied, advance
            guard.advance();
        }

        let (curr_x, curr_y) = guard.pos();
        if guard.been_here_before(&loc_array[curr_x][curr_y]) {
            return true;
        }

        loc_array[curr_x][curr_y].update_visited(guard.dir());
        next_pos = guard.next_step();
    }
    false
}

fn parse_input(filename: &String) -> ([[Tile; MAX_Y]; MAX_X], Guard) {
    let mut loc_array = [[Tile::new(); MAX_Y]; MAX_X];
    let mut guard_dir = None;
    let mut guard_pos = (0, 0);

    let mut x: usize = 0;
    let mut y: usize = 0;
    for char in fs::read_to_string(filename).unwrap().chars() {
        let mut valid_char = true;
        match char {
            '.' => loc_array[x][y].occupied = false,
            '#' => loc_array[x][y].occupied = true,
            '^' => {
                guard_pos = (x, y);
                guard_dir = Some(GuardDirection::NORTH);
                loc_array[x][y].visited_north = true;
                loc_array[x][y].loop_counted = true;
            },
            '>' => {
                guard_pos = (x, y);
                guard_dir = Some(GuardDirection::EAST);
                loc_array[x][y].visited_east = true;
                loc_array[x][y].loop_counted = true;
            },
            'v' => {
                guard_pos = (x, y);
                guard_dir = Some(GuardDirection::SOUTH);
                loc_array[x][y].visited_south = true;
                loc_array[x][y].loop_counted = true;
            },
            '<' => {
                guard_pos = (x, y);
                guard_dir = Some(GuardDirection::WEST);
                loc_array[x][y].visited_west = true;
                loc_array[x][y].loop_counted = true;
            },
            '\n' => valid_char = false,
            '\r' => valid_char = false,
            _ => panic!("Unexpected character in input: {char}"),
        }

        if valid_char {
            x = (x + 1) % MAX_X;
            if x == 0 {
                y += 1;
            }
        }
    }

    (loc_array, Guard::new(guard_pos, guard_dir.unwrap()))
}