use std::{fs, mem, rc::Rc, cell::RefCell, ops::Deref, ops::DerefMut};

#[derive(Clone)]
enum Address {
    Address(Box<Stone>),
    Nil
}

#[derive(Clone)]
struct Stone {
    value: u64,
    next: Address,
}

impl Stone {
    fn append(&mut self, item: u64) {
        match self.next {
            Address::Address(ref mut next_stone) => {
                next_stone.append(item);
            },
            Address::Nil => {
                let stone = Stone {
                    value: item,
                    next: Address::Nil,
                };
                self.next = Address::Address(Box::new(stone));
            }
        }
    }

    fn blink(&mut self) -> bool {
        //println!("Calling blink with value {}", self.value);
        if self.value == 0 {
            // Value is 0, replace with 1
            self.value = 1;
        } else if (f32::log10(self.value as f32 + 0.01).ceil() as u32) % 2 == 0 {
            // Value has an even number of digits, split in half
            let num_digits = (f32::log10(self.value as f32 + 0.01).ceil()) as u32 / 2;
            let new_value = self.value / 10u64.pow(num_digits);
            let next_value = self.value - new_value * 10u64.pow(num_digits);
            
            self.value = new_value;
            
            let next_stone = Stone {
                value: next_value,
                next: mem::replace(&mut self.next, Address::Nil),
            };
            self.next = Address::Address(Box::new(next_stone));
            return true
        } else {
            self.value *= 2024;
        }

        false
    }
}

fn print_stone_line(head: &Stone) {
    let mut string = String::new();
    let mut next_stone = head.clone();
    loop {
        for char in next_stone.value.to_string().chars() {
            string.push(char);
        }
        string.push(',');
        string.push(' ');
        match next_stone.next {
            Address::Address(ref mut stone) => next_stone = *stone.clone(),
            Address::Nil => break,
        }
    }
    println!("Stone line is [{string}]");
}

// For the inefficient vector implementation
// fn blink(values: &mut Vec<u64>) {
//     let mut index = 0;
//     while index < values.len() {
//         if values[index] == 0 {
//             values[index] = 1;
//         } else if (f32::log10(values[index] as f32 + 0.01).ceil() as u32) % 2 == 0 {
//             let num_digits = (f32::log10(values[index] as f32 + 0.01).ceil()) as u32 / 2;
//             let new_value = values[index] / 10u64.pow(num_digits);
//             let next_value = values[index] - new_value * 10u64.pow(num_digits);

//             /* Replace the value at index */
//             values[index] = new_value;
            
//             /* Insert the new value */
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

fn main() {
    
    //Vector implementation
    // let mut stone_line: Vec<u64> = fs::read_to_string("E:\\dev\\AoC2024\\day11\\input.txt").unwrap().replace("\n\r", "").split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

    //Part 1
    // let num_blinks = 25;
    // for _ in 0..num_blinks {
    //     blink(stone_line.as_mut());
    // }
    // println!("Final stone count after 25 blinks is {}", stone_line.len());

    // Part 2
    // let num_blinks = 50;
    // for _ in 0..num_blinks {
    //     blink(stone_line.as_mut());
    // }
    // println!("Final stone count after 75 blinks is {}", stone_line.len());

    let mut input_values: Vec<u64> = fs::read_to_string("E:\\dev\\AoC2024\\day11\\input.txt").unwrap().replace("\n\r", "").split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
    let mut head = Stone {
        value: input_values[0],
        next: Address::Nil,
    };

    for index in 1..input_values.len() {
        head.append(input_values[index]);
    }

    //Part 1
    let mut blink_count = 0;
    for _ in 0..25 {
        let mut count = 1;
        let mut skip_next = head.blink();
        let mut current_address = &mut head.next;
        while let Address::Address(stone) = current_address {
            if skip_next {
                // Skip this stone and move to the next one
                current_address = &mut stone.next;
                skip_next = false;
            } else {
                // Process the current stone
                skip_next = stone.blink();
                current_address = &mut stone.next;
            }
    
            count += 1;
        }
        blink_count += 1;
        println!("The length is {count} after blinking {blink_count} times");
        //print_stone_line(&head);
    }

    //Part 2
    for _ in 0..50 {
        let mut count = 1;
        let mut skip_next = head.blink();
        let mut current_address = &mut head.next;
        while let Address::Address(stone) = current_address {
            if skip_next {
                // Skip this stone and move to the next one
                current_address = &mut stone.next;
                skip_next = false;
            } else {
                // Process the current stone
                skip_next = stone.blink();
                current_address = &mut stone.next;
            }
    
            count += 1;
        }
        blink_count += 1;
        println!("The length is {count} after blinking {blink_count} times");
    }


}
