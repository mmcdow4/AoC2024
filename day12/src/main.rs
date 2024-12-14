use std::{fs, io::prelude::*};

struct Plot {
    plant: char,
    tallied: bool,
}

impl Plot {
    fn new(plant: char) -> Plot {
        Plot {
            plant,
            tallied: false,
        }
    }

    fn plant(&self) -> char {
        self.plant
    }

    fn tallied(&self) -> bool {
        self.tallied
    }

    fn mark_tallied(&mut self) {
        self.tallied = true;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Region {
    minimap: Vec<Vec<bool>>,
    visited: Vec<Vec<bool>>,
    area: usize,
    perimeter: usize,
    sides: usize,
    dims: (usize, usize),
    direction: Direction,
    current_pos: (usize, usize),
    interior_mode: bool,
}

impl Region {
    fn new(plot_positions: &Vec<(usize, usize)>) -> Region {
        let mut max_x = 0;
        let mut min_x = usize::max_value();
        let mut max_y = 0;
        let mut min_y = usize::max_value();

        for pos in plot_positions {
            max_x = max_x.max(pos.0);
            min_x = min_x.min(pos.0);
            max_y = max_y.max(pos.1);
            min_y = min_y.min(pos.1);
        }
        let dims = (max_x - min_x + 1, max_y - min_y + 1);
        let mut minimap = vec![vec![false; dims.0]; dims.1];
        let visited = minimap.clone();
        for pos in plot_positions {
            minimap[pos.1 - min_y][pos.0 - min_x] = true;
        }

        Region { 
            minimap,
            visited,
            area: plot_positions.len(),
            perimeter: 1,
            sides: 0,
            dims,
            direction: Direction::East,
            current_pos: (0, 0),
            interior_mode: false,
        }
    }

    fn compute_parameters(&mut self) -> (usize, usize, usize) {

        /* Now travers the outside of the minimap to find the perimeter and number of sides */
        /* Start by finding the top left corner of the region */
        while !self.minimap[self.current_pos.1][self.current_pos.0] {
            self.current_pos.0 += 1;
        }

        let start_point = self.current_pos;
        loop {
            self.next_step();
            if self.current_pos == start_point && self.direction == Direction::East {
                break;
            } else {
                if self.perimeter_test(self.current_pos, true) {
                    self.perimeter += 1;
                    /* Mark the perimeter point as visited to save time later */
                }
            }
        }
        
        /* Now look for interior points, and trace them with the same logic */
        self.interior_mode = true;
        for y in 1..(self.dims.1-1) {
            for x in 1..(self.dims.0-1) {
                /* Can't be an interior point on the boundary of the minimap */
                let dir = self.interior_point_test((x, y));
                if dir.is_some() {
                    self.current_pos = (x, y);
                    self.direction = dir.unwrap();
                    let start_dir = self.direction;
                    let start_point = self.current_pos;
                    loop {
                        self.next_step();
                        if self.current_pos == start_point && self.direction == start_dir {
                            self.perimeter += 1;
                            break;
                        } else {
                            if self.perimeter_test(self.current_pos, true) {
                                self.perimeter += 1;
                                /* Mark the perimeter point as visited to save time later */
                            }
                        }
                    }
                }
                self.visited[y][x] = true
            }
        }
        (self.area, self.perimeter, self.sides)
    }

    // Either move to the next point along the perimeter,
    // or change directions to keep following the border
    fn next_step(&mut self) {
        let boundary_test = match self.direction {
            Direction::East => {self.current_pos.0 == self.dims.0 - 1},
            Direction::South => {self.current_pos.1 == self.dims.1 - 1},
            Direction::West => {self.current_pos.0 == 0},
            Direction::North => {self.current_pos.1 == 0},
        };//True if cannot proceed because current_pos is the edge of the minimap

        if boundary_test {
            /* Forced to turn */
            self.turn();
        } else {
            let next_pos = match self.direction {
                Direction::East => (self.current_pos.0 + 1, self.current_pos.1),
                Direction::South => (self.current_pos.0, self.current_pos.1 + 1),
                Direction::West => (self.current_pos.0 - 1, self.current_pos.1),
                Direction::North => (self.current_pos.0, self.current_pos.1 - 1),
            };

            if self.minimap[next_pos.1][next_pos.0] == !self.interior_mode {
                self.current_pos = next_pos;
                if !self.perimeter_test(self.current_pos, false) {
                    self.turn();
                }
            } else {
                /* We advanced out of the region or we hit an internal corner, we need to stay put and turn instead */
                self.turn();
            }
        }
        self.visited[self.current_pos.1][self.current_pos.0] = true;
    }

    fn turn(&mut self) {
        /* Every time we turn it is because we hit the end of a side, so increment the side count */
        self.sides += 1;
        /* Moving around the region clockwise, so east prefers north, north prefers west, etc. */
        self.direction = match self.direction {
            Direction::East => {
                /* Turn north if able */
                if self.current_pos.1 > 0 && self.minimap[self.current_pos.1 -1][self.current_pos.0] != self.interior_mode {
                    Direction::North
                } else {
                    Direction::South
                }
            },
            Direction::South => {
                /* Turn east if able */
                if self.current_pos.0 < self.dims.0 - 1 && self.minimap[self.current_pos.1][self.current_pos.0 + 1] != self.interior_mode {
                    Direction::East
                } else {
                    Direction::West
                }
            },
            Direction::West => {
                /* Turn south if able */
                if self.current_pos.1 < self.dims.1 - 1 && self.minimap[self.current_pos.1 + 1][self.current_pos.0] != self.interior_mode {
                    Direction::South
                } else {
                    Direction::North
                }
            },
            Direction::North => {
                /* Turn west if able */
                if self.current_pos.0 > 0 && self.minimap[self.current_pos.1][self.current_pos.0 - 1]  != self.interior_mode{
                    Direction::West
                } else {
                    Direction::East
                }
            },
        };

    }


    fn perimeter_test(&mut self, pos: (usize, usize), mark_visit: bool) -> bool {
        if self.direction == Direction::East && ((!self.interior_mode && pos.1 == 0) || (self.minimap[pos.1 - 1][pos.0] == self.interior_mode)) {
            /* Moving east and the fence runs along the top of this point */
            if mark_visit && pos.1 > 0 {
                self.visited[pos.1 - 1][pos.0] = true;
            }
            return true
        } else if self.direction == Direction::South && ((!self.interior_mode && pos.0 == self.dims.0 - 1) || (self.minimap[pos.1][pos.0 + 1] == self.interior_mode)) {
            /* Moving south and the fence runs along the right of this point */
            if mark_visit && pos.0 < self.dims.0 - 1 {
                self.visited[pos.1][pos.0 + 1] = true;
            }
            return true
        } else if self.direction == Direction::West && ((!self.interior_mode && pos.1 == self.dims.1 - 1) || (self.minimap[pos.1 + 1][pos.0] == self.interior_mode)) {
            /* Moving west and the fence runs along the bottom of this point */
            if mark_visit && pos.1 < self.dims.1 - 1 {
                self.visited[pos.1 + 1][pos.0] = true;
            }
            return true
        }  else if self.direction == Direction::North && ((!self.interior_mode && pos.0 == 0) || (self.minimap[pos.1][pos.0 - 1] == self.interior_mode)) {
            /* Moving north and the fence runs along the left of this point */
            if mark_visit && pos.0 > 0 {
                self.visited[pos.1][pos.0 - 1] = true;
            }
            return true
        }
        false
    }

    fn interior_point_test(&self, pos: (usize, usize)) -> Option<Direction> {
        if !self.minimap[pos.1][pos.0] && !self.visited[pos.1][pos.0] {
            if pos.0 > 0 && self.minimap[pos.1][pos.0 - 1] {
                return Some(Direction::North)
            } else if pos.0 < self.dims.0 - 1 && self.minimap[pos.1][pos.0 + 1] {
                return Some(Direction::South)
            } else if pos.1 > 0 && self.minimap[pos.1 - 1][pos.0] {
                return Some(Direction::East)
            } else if pos.1 < self.dims.1 - 1 && self.minimap[pos.1 + 1][pos.0] {
                return Some(Direction::West)
            }
        }
        None
    }

    fn print_to_file(&self, ch: char) {
        let filename = String::from(ch) + ".txt".to_string().as_str();
        
        let mut file = fs::File::create(filename).unwrap();
        for y in 0..self.dims.1 {
            for x in 0..self.dims.0 {
                if self.minimap[y][x] {
                    file.write(ch.to_string().as_bytes()).expect("Unable to write to debug file");
                } else {
                    file.write(".".as_bytes()).expect("Unable to write to debug file");
                }
            }
            file.write("\n".as_bytes()).unwrap();
        }
    }
}

fn find_region_points(garden: &mut Vec<Vec<Plot>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    garden[pos.1][pos.0].mark_tallied();
    points.push(pos);
    let plant = garden[pos.1][pos.0].plant();
    if pos.0 < garden[pos.1].len()-1 && garden[pos.1][pos.0 + 1].plant() == plant {
        /* Space to the east is the same plant */
        if !garden[pos.1][pos.0 + 1].tallied() {
            points.extend(find_region_points(garden, (pos.0 + 1, pos.1)));
        }
    }
    if pos.1 < garden.len()-1 && garden[pos.1 + 1][pos.0].plant() == plant {
        /* Space to the south is the same plant */
        if !garden[pos.1 + 1][pos.0].tallied() {
            points.extend(find_region_points(garden, (pos.0, pos.1 + 1)));
        }
    }
    if pos.0 > 0 && garden[pos.1][pos.0 - 1].plant() == plant {
        /* Space to the west is the same plant, remove that fence tally */
        if !garden[pos.1][pos.0 - 1].tallied() {
            points.extend(find_region_points(garden, (pos.0 - 1, pos.1)));
        }
    }
    if pos.1 > 0 && garden[pos.1 - 1][pos.0].plant() == plant {
        /* Space to the north is the same plant, remove that fence tally */
        if !garden[pos.1 - 1][pos.0].tallied() {
            points.extend(find_region_points(garden, (pos.0, pos.1 - 1)));
        }
    }
    points
}

fn main() {
    let mut garden: Vec<Vec<Plot>> = Vec::with_capacity(140);

    for line in fs::read_to_string("E:\\dev\\AoC2024\\day12\\input.txt").unwrap().lines() {
        let mut new_row = Vec::with_capacity(140);
        for ch in line.chars() {
            match ch {
                '\n' => break,
                '\r' => break,
                _ => new_row.push(Plot::new(ch)),
            }
        }
        garden.push(new_row);
    }

    let mut total_price = 0;
    let mut discount_price = 0;
    for y in 0..garden.len() {
        for x in 0..garden[y].len() {
            if !garden[y][x].tallied() {
                let region_points = find_region_points(&mut garden, (x, y));
                let mut region = Region::new(&region_points);
                
                let (area, perimeter, sides) = region.compute_parameters();
                total_price += area * perimeter;
                discount_price += area * sides;
                println!("Found a {} region of area {area}, perimeter {perimeter}, and sides {sides} centered on ({x}, {y})", garden[y][x].plant());
            }
        }
    }

    println!("Final computed price is {total_price}, discounted price is {discount_price}");
}
