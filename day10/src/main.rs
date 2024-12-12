use std::{fs, collections::HashSet};

fn seek_peaks(topographic_map: &Vec<Vec<u32>>, current_point: (usize, usize)) -> (usize, HashSet<(usize, usize)>) {
    let mut peaks: HashSet<(usize, usize)> = HashSet::new();
    let mut rating = 0;
    let current_height = topographic_map[current_point.1][current_point.0];
    if current_height == 9 {
        peaks.insert(current_point);
        rating = 1;
        return (rating, peaks);
    }
    if current_point.0 > 0 && topographic_map[current_point.1][current_point.0-1] == current_height + 1{
        /* Try going west */
        let (new_rating, new_peaks) = seek_peaks(topographic_map, (current_point.0 - 1, current_point.1));
        rating += new_rating;
        peaks.extend(new_peaks);
    }
    if current_point.0 < topographic_map[current_point.1].len()-1 && topographic_map[current_point.1][current_point.0+1] == current_height + 1{
        /* Try going east */
        let (new_rating, new_peaks) = seek_peaks(topographic_map, (current_point.0 + 1, current_point.1));
        rating += new_rating;
        peaks.extend(new_peaks);
    }
    if current_point.1 < topographic_map.len()-1 && topographic_map[current_point.1+1][current_point.0] == current_height + 1{
        /* Try going north */
        let (new_rating, new_peaks) = seek_peaks(topographic_map, (current_point.0, current_point.1+1));
        rating += new_rating;
        peaks.extend(new_peaks);
    }
    if current_point.1 > 0 && topographic_map[current_point.1-1][current_point.0] == current_height + 1{
        /* Try going south */
        let (new_rating, new_peaks) = seek_peaks(topographic_map, (current_point.0, current_point.1 - 1));
        rating += new_rating;
        peaks.extend(new_peaks);
    }
    (rating, peaks)
}

fn main() {
    let mut topographic_map = vec![];

    for line in fs::read_to_string("E:\\dev\\AoC2024\\day10\\input.txt").unwrap().lines() {
        topographic_map.push(vec![]);
        for char in line.chars() {
            topographic_map.last_mut().unwrap().push(char.to_digit(10).unwrap());
        }
    }

    let mut score_sum = 0;
    let mut rating_sum = 0;
    for y in 0..topographic_map.len() {
        for x in 0..topographic_map[y].len() {
            if topographic_map[y][x] == 0 {
                /* This is a potential trail head, search for reachable peaks */
                let (rating, unique_peaks) = seek_peaks(&topographic_map, (x, y));
                rating_sum += rating;
                score_sum += unique_peaks.len();
            }
        }
    }

    println!("Sum of trailhead scores is {score_sum}, sum of ratings is {rating_sum}");
}
