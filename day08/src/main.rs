use std::{
    fs,
    io::prelude::*,
    ops,
    collections::{HashMap, HashSet},
    cmp
};

fn find_gcd(a: i32, b: i32) -> i32 {
    let mut gcd = 1;
    for x in 2..cmp::min(a.abs(), b.abs()) {
        if (a % x == 0) && (b % x == 0) {
            gcd = x;
        }
    }

    gcd
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_valid(&self, max_x: i32, max_y: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < max_x && self.y < max_y
    }

    fn get_gcd(&self, other: &Point) -> i32 {
        let gcdx = find_gcd(self.x, other.x);
        let gcdy = find_gcd(self.y, other.y);
        find_gcd(gcdx, gcdy)
    }
}

impl ops::Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Point {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Antenna {
    identifier: char,
    pos: Point,
}

impl Antenna {
    fn new(identifier: char, pos: Point) -> Antenna {
        Antenna {
            identifier,
            pos,
        }
    }

    fn identifier(&self) -> char {
        self.identifier
    }
    
    fn pos(&self) -> &Point {
        &self.pos
    }

    fn find_antinodes(&self, other: &Antenna, max_x: i32, max_y: i32) -> Vec<Point> {
        let other_pos = other.pos();

        let gcd = self.pos.get_gcd(other_pos);
        let delta = (self.pos - other_pos) / gcd;

        let mut output_vec = Vec::new();
        let mut antinode = self.pos;
        loop {
            output_vec.push(antinode);
            antinode = antinode + &delta;
            if !antinode.is_valid(max_x, max_y) {
                break;
            }
        }
        antinode = self.pos - &delta;
        loop {
            output_vec.push(antinode);
            antinode = antinode - &delta;
            if !antinode.is_valid(max_x, max_y) {
                break;
            }
        }
        output_vec
    }
}

fn main() {
    
    let mut antinodes: HashSet<Point> = HashSet::new();
    let mut antenna_map: HashMap<char, Vec<Antenna>> = HashMap::new();
    let mut max_x: i32 = 0;
    let mut max_y = 0;
    for line in fs::read_to_string("E:\\dev\\AoC2024\\day08\\input.txt").unwrap().lines() {
        max_x = line.trim().len() as i32;
        let mut x = 0;
        for next_char in line.trim().chars() {
            match next_char {
                '.' => x += 1,
                '\n' => break,
                '\r' => break,
                _ => {
                    if antenna_map.contains_key(&next_char) {
                        antenna_map.get_mut(&next_char).unwrap().push(Antenna::new(next_char, Point::new(x, max_y)));
                    } else {
                        let temp_vec = vec![Antenna::new(next_char, Point::new(x, max_y))];
                        antenna_map.insert(next_char, temp_vec);
                    }
                    x += 1;
                }
            }
        }
        max_y += 1;
    }

    let mut file = fs::File::create("input_as_read.txt").unwrap();
    for y in 0..max_y {
        for x in 0..max_x {
            let mut found = false;
            for (identifier, points) in &antenna_map {
                if points.contains(&Antenna::new(*identifier, Point::new(x, y))) {
                    file.write(String::from(*identifier).as_bytes()).unwrap();
                    found = true;
                }
            }
            if !found {
                file.write(".".as_bytes()).unwrap();
            }
        }
        file.write("\n".as_bytes()).unwrap();
    }

    for (_, antennas) in antenna_map {
        for outter_index in 0..antennas.len() {
            for inner_index in outter_index+1..antennas.len() {
                let new_antinodes = antennas[outter_index].find_antinodes(&antennas[inner_index], max_x, max_y);
                antinodes.extend(new_antinodes.iter());
            }
        }
    }

    println!("Found {} unique antinode locations", antinodes.len());
}
