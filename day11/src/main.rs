use std::{fs, collections::HashMap};


//             let temp_vec = values.split_off(index+1);
//             values.push(next_value);
//             values.extend(temp_vec);
//             index += 1; //Skip past the newly inserted value
//         } else {
//             values[index] = values[index] * 2024;
//         }
//         index += 1;
//     }
// }

fn next_value(value: u64) -> Vec<u64> {
    let mut output_values = Vec::new();
    if value == 0 {
        output_values.push(1);
    } else if value.to_string().len() % 2 == 0 {
        let value_string = value.to_string();
        output_values.push(value_string[0..(value_string.len() / 2)].parse().unwrap());
        output_values.push(value_string[(value_string.len() / 2)..].parse().unwrap());
    } else {
        output_values.push(value * 2024);
    }
    output_values
}

fn blink(stones: &HashMap<u64, u64>, cache: &mut HashMap<u64, Vec<u64>>) -> HashMap<u64, u64> {
    let mut new_stones: HashMap<u64, u64> = HashMap::with_capacity(stones.len() * 2);
    for (stone, count) in stones {
        if let Some(new_values) = cache.get(&stone) {
            for value in new_values {
                *new_stones
                    .entry(*value)
                    .or_insert(0) += count;
            }
        } else {
            let new_values = next_value(*stone);
            for value in &new_values {
                *new_stones
                    .entry(*value)
                    .or_insert(0) += count;
            }
            cache.insert(*stone, new_values);
        }
    }
    new_stones
}

fn main() {
    
    // let num_blinks = 50;
    // for _ in 0..num_blinks {
    //     blink(stone_line.as_mut());
    // }
    // println!("Final stone count after 75 blinks is {}", stone_line.len());

    let mut stones: HashMap<u64, u64> = HashMap::new();
    let mut cache: HashMap<u64, Vec<u64>> = HashMap::new();

    for value in fs::read_to_string("E:\\dev\\AoC2024\\day11\\input.txt").unwrap().replace("\n\r", "").split(' ') {
        stones.insert(value.parse().unwrap(), 1);
    }

    /* Part 1 */
    for _ in 0..25 {
        stones = blink(&stones, &mut cache);
    }

    let mut stone_count = 0;
    for (_, count) in &stones {
        stone_count += count;
    }
    println!("After 25 blinks there are {stone_count} stones");

    /* Part 2 */
    for _ in 0..50 {
        stones = blink(&stones, &mut cache);
    }

    stone_count = 0;
    for (_, count) in &stones {
        stone_count += count;
    }
    println!("After 75 blinks there are {stone_count} stones");


}
