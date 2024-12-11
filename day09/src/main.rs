use std::fs;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum MemoryBlock {
    Free,
    File(u32),
}

impl MemoryBlock {
    fn block_id(&self) -> Option<u32> {
        match self {
            MemoryBlock::Free => None,
            MemoryBlock::File(x) => Some(*x),
        }
    }
}
fn write_debug_line(file: &mut fs::File, memory_system: &Vec<i64>) {
    let char_cipher = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    for block in memory_system {
        if !block.is_negative() {
            file.write(&[char_cipher.chars().nth(*block as usize).unwrap() as u8]).unwrap();
            //file.write("#".as_bytes()).unwrap();
        } else {
            file.write(".".as_bytes()).unwrap();
        }
    }
    file.write("\n".as_bytes()).unwrap();
}

fn find_next(vector: &Vec<MemoryBlock>, value: MemoryBlock, start: usize) -> usize {
    for index in start..vector.len() {
        if vector[index] == value {
            return index
        }
    };
    panic!("Unable to find an empty block starting from {start}");
}

fn find_file_block(vector: &Vec<MemoryBlock>, value: MemoryBlock) -> (usize, usize) {
    let mut start_index = None;
    for (index, block_id) in vector.iter().enumerate() {
        if *block_id == value {
            if start_index.is_none() {
                /* Found the start of the file */
                start_index = Some(index);
            }
        } else if start_index.is_some() {
            let length = index - start_index.unwrap();
            return (start_index.unwrap(), length);
        }
    };

    if start_index.is_some() {
        /* End of the block is the end of the file system */
        let length = vector.len() - start_index.unwrap();
        (start_index.unwrap(), length)
    } else {
        panic!("Unable to find a file block for ID {:?}", value);
    }
}

fn find_free_space(memory_system: &Vec<MemoryBlock>, size: usize, max_index: usize) -> Option<usize> {
    let mut index: usize = find_next(&memory_system, MemoryBlock::Free, 0);
    while index < max_index {
        let mut end = index;
        while memory_system[end] == MemoryBlock::Free {
            end += 1;
        }

        if (end - index) >= size {
            /* Found a free block large enough, return the left most index */
            return Some(index);
        } else {
            /* This block was not large enough, find the next one */
            index = find_next(&memory_system, MemoryBlock::Free, end);
        }
    }
    None
}
fn main() {
    let mut memory_system: Vec<MemoryBlock> = Vec::with_capacity(40000);
    let mut is_file = true;
    let mut file_index: u32 = 0;
    //let mut transfer_total = 0;
    for char in fs::read_to_string("E:\\dev\\AoC2024\\day09\\input.txt").unwrap().chars() {
        let mut block_size = char.to_digit(10).unwrap();
        let insert_value = match is_file {
            true => {
                MemoryBlock::File(file_index)
            },
            false => MemoryBlock::Free,
        };
        
        while block_size > 0 {
            memory_system.push(insert_value);
            block_size -= 1;
        }

        if is_file {
            file_index += 1;
        }

        is_file = !is_file;
    }

    // // Part 1 solution
    // let mut next_free_index = find_next(&memory_system, MemoryBlock::Free, 0);
    // for index in (0..memory_system.len()).rev() {
    //     if index <= next_free_index {
    //         break;
    //     }
    //     if memory_system[index] != MemoryBlock::Free {
    //         memory_system[next_free_index] = memory_system[index];
    //         memory_system[index] = MemoryBlock::Free;
    //         next_free_index = find_next(&memory_system, MemoryBlock::Free, next_free_index + 1);        
    //     }
    // }

    // Part 2 solution
    for file_id in (0..file_index).rev() {
        let (file_start, file_length) = find_file_block(&memory_system, MemoryBlock::File(file_id));
        let free_index = find_free_space(&memory_system, file_length, file_start);

        if free_index.is_some() {
            let free_index = free_index.unwrap();
            for block_count in 0..file_length {
                memory_system[free_index + block_count] = memory_system[file_start + block_count];
                memory_system[file_start + block_count] = MemoryBlock::Free;
            }
        }
    }

    let mut checksum = 0;
    for index in 0..memory_system.len() {
        if memory_system[index].block_id().is_some() {
            checksum += index * (memory_system[index].block_id().unwrap() as usize);
        }
    }

    println!("The final checksum is {checksum}");
}
