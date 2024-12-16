use std::{
    fs,
    io::prelude::*,
    ops,
    env,
    collections::HashMap,
};
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Quadrant {
    I,
    II,
    III,
    IV,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}


impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y, }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Rem<Point> for Point {
    type Output = Point;

    fn rem(self, rhs: Point) -> Point {
        let mut new_x = self.x % rhs.x;
        if new_x < 0 {
            new_x += rhs.x;
        }
        let mut new_y = self.y % rhs.y;
        if new_y < 0 {
            new_y += rhs.y;
        }
        Point {
            x: new_x,
            y: new_y,
        }
    }
}

struct Robot {
    pos: Point,
    vel: Point,
    dim: Point,
}

impl Robot {
    fn new(pos: Point, vel: Point, dim: Point) -> Robot {
        Robot {pos, vel, dim}
    }

    fn take_n_steps (&self, n: i64) -> Point {
        let final_pos = self.pos + self.vel * n;
        final_pos % self.dim
    }

}

fn quadrant(pos: &Point, dim: &Point) -> Option<Quadrant> {
    if pos.x < (dim.x - 1) / 2 {
        if pos.y < (dim.y - 1) / 2 {
            return Some(Quadrant::II);
        } else if pos.y > (dim.y - 1) / 2 {
            return Some(Quadrant::III);
        }
    } else if pos.x > (dim.x - 1) / 2 {
        if pos.y < (dim.y - 1) / 2 {
            return Some(Quadrant::I);
        } else if pos.y > (dim.y - 1) / 2 {
            return Some(Quadrant::IV);
        }
    }
    None
}
fn write_map(minimap: &Vec<Vec<usize>>, index: usize) {

    let mut file = fs::File::create(format!("E:\\dev\\AoC2024\\day14\\map{index:03}.txt")).unwrap();

    for y in 0..minimap.len() {
        for x in 0..minimap[y].len() {
            if minimap[y][x] > 0 {
                file.write(minimap[y][x].to_string().as_bytes()).expect("Unable to write to debug file");
            } else {
                file.write(".".as_bytes()).expect("unable to write to debug file");
            }
        }
        file.write("\n".as_bytes()).expect("unable to write to debug file");
    }
}

fn count_region(minimap: &Vec<Vec<usize>>, pos: Point) -> usize {
    let mut count = 0;
    for y in (pos.y-1)..=(pos.y+1) {
        for x in (pos.x-1)..=(pos.x+1) {
            count += minimap[y as usize][x as usize];
        }
    }
    count
}
fn compute_entropy(minimap: &Vec<Vec<usize>>, num_robots: f64) -> f64 {
    let mut entropy = 0.0;

    for y in (1..minimap.len()-1).step_by(3) {
        for x in (1..minimap[y].len()-1).step_by(3) {
            let counts = count_region(minimap, Point::new(x as i64,y as i64));
            if counts > 0 {
                
                let p_i = (counts as f64) / num_robots;
                entropy += -1.0 * p_i * p_i.log2();
            }
        }
    }
    entropy
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let dim = Point::new( args[1].parse::<i64>().unwrap(), args[2].parse::<i64>().unwrap());
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").expect("Problem constructing the regex");

    let mut quadrant_counts: HashMap<Quadrant, usize> = HashMap::new();
    quadrant_counts.insert(Quadrant::I, 0);
    quadrant_counts.insert(Quadrant::II, 0);
    quadrant_counts.insert(Quadrant::III, 0);
    quadrant_counts.insert(Quadrant::IV, 0);
    let mut robots: Vec<Robot> = Vec::with_capacity(500);
    for line in fs::read_to_string("E:\\dev\\AoC2024\\day14\\input.txt").unwrap().lines() {
        let matches = re.captures(line).unwrap();
        let pos = Point::new(
            matches[1].parse::<i64>().unwrap(),
            matches[2].parse::<i64>().unwrap(),
        );
        let vel = Point::new(
            matches[3].parse::<i64>().unwrap(),
            matches[4].parse::<i64>().unwrap(),
        );
        let robot = Robot::new(pos, vel, dim);
        robots.push(robot);
    }

    let mut average_entropy = 0.0;
    for second_count in 1..=100 {
        let mut minimap: Vec<Vec<usize>> = vec![vec![0; dim.x as usize]; dim.y as usize];
        for robot in &robots {
            let final_pos = robot.take_n_steps(second_count);
            minimap[final_pos.y as usize][final_pos.x as usize] += 1;
            if second_count == 100 {
                let quadrant = quadrant(&final_pos, &dim);
                if quadrant.is_some() {
                    quadrant_counts.entry(quadrant.unwrap()).and_modify(|count| *count += 1);
                }
            }
        }
        average_entropy += compute_entropy(&minimap, robots.len() as f64) / 100.0;
        //write_map(&minimap, second_count as usize);
    }
    println!("After 100 seconds: Quadrant I has {} robots", quadrant_counts.get(&Quadrant::I).unwrap());
    println!("Quadrant II has {} robots", quadrant_counts.get(&Quadrant::II).unwrap());
    println!("Quadrant III has {} robots", quadrant_counts.get(&Quadrant::III).unwrap());
    println!("Quadrant IV has {} robots", quadrant_counts.get(&Quadrant::IV).unwrap());
    let safety_factor = quadrant_counts.get(&Quadrant::I).unwrap() *
        quadrant_counts.get(&Quadrant::II).unwrap() *
        quadrant_counts.get(&Quadrant::III).unwrap() *
        quadrant_counts.get(&Quadrant::IV).unwrap();

    println!("Final safety factor after 100 seconds = {safety_factor}");

    let mut second_count = 101;
    let entropy_threshold = average_entropy * 0.9;
    let mut lowest_entropy = f64::MAX;
    println!("Searching for low entropy cases, average of first 100 was {average_entropy}, creating a threshold of {entropy_threshold}");
    loop {
        let mut minimap: Vec<Vec<usize>> = vec![vec![0; dim.x as usize]; dim.y as usize];
        for robot in &robots {
            let final_pos = robot.take_n_steps(second_count);
            minimap[final_pos.y as usize][final_pos.x as usize] += 1;
        }

        lowest_entropy = lowest_entropy.min(compute_entropy(&minimap, robots.len() as f64));
        if lowest_entropy < entropy_threshold {
            println!("I think the easter egg occurs at {second_count} seconds based on entropy.");
            write_map(&minimap, second_count as usize);
            break;
        }
        second_count += 1;
    }
}
