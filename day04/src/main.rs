use std::fs;

fn main() {
    let char_array = read_to_text_array(&String::from("E:\\dev\\advent_of_code_2024\\day04\\input.txt"));

    let mut xmas_count = 0;

    for line_index in 0..140 {
        for char_index in 0..140 {
            if check_for_xmas(&char_array, char_index, line_index, -1, 0) {
                // Check for leftwards
                xmas_count += 1;
            }
            if check_for_xmas(&char_array, char_index, line_index, -1, 1) {
                // Check for diagonal up-left
                xmas_count += 1;
            }
            if check_for_xmas(&char_array, char_index, line_index, 0, 1) {
                // Check for upwards
                xmas_count += 1;
            }
            if check_for_xmas(&char_array, char_index, line_index, 1, 1) {
                // Check for diagonal up-right
                xmas_count += 1;
            }
            if check_for_xmas(&char_array, char_index, line_index, 1, 0) {
                // Check for rightwards
                xmas_count += 1;
            }
            if check_for_xmas(&char_array, char_index, line_index, 1, -1) {
                // Check for diagonal down-right
                xmas_count += 1;
            }
            if check_for_xmas(&char_array, char_index, line_index, 0, -1) {
                // Check for downwards
                xmas_count += 1;
            }
            if check_for_xmas(&char_array, char_index, line_index, -1, -1) {
                // Check for diagonal down-left
                xmas_count += 1;
            }
        }
    }

    println!("xmas count is {xmas_count}");

    let mut x_mas_count = 0;
    for line_index in 1..139 {
        for char_index in 1..139 {
            x_mas_count += check_for_x_mas(&char_array, char_index, line_index);
        }
    }

    println!("x-mas count is {x_mas_count}");
}


fn read_to_text_array(filename: &String) -> [[char; 140]; 140] {
    let mut char_array: [[char; 140]; 140] = [[' '; 140]; 140];

    let mut line_idx = 0;
    for line in fs::read_to_string(filename).unwrap().lines() {
        let mut char_idx = 0;
        for new_char in line.chars() {
            char_array[line_idx][char_idx] = new_char;
            char_idx += 1;
        }
        line_idx += 1;
    }

    char_array
}

fn check_for_xmas(char_array: &[[char; 140]; 140], x: usize, y: usize, xdir: i32, ydir: i32) -> bool {
    let x_signed = x as i32;
    let y_signed = y as i32;
    let x1 = x_signed + 1*xdir;
    let y1 = y_signed + 1*ydir;
    let x2 = x_signed + 2*xdir;
    let y2 = y_signed + 2*ydir;
    let x3 = x_signed + 3*xdir;
    let y3 = y_signed + 3*ydir;

    if x3 >= 0 && x3 < 140 && y3 >= 0 && y3 < 140
    {
        char_array[x][y] == 'X' && char_array[x1 as usize][y1 as usize] == 'M' && char_array[x2 as usize][y2 as usize] == 'A' && char_array[x3 as usize][y3 as usize] == 'S'
    } else {
        false
    }
}

fn check_for_x_mas(char_array: &[[char; 140]; 140], x: usize, y: usize) -> usize {
    // let x_signed = x as i32;
    // let y_signed = y as i32;
    let mut count = 0;
    if char_array[x][y] == 'A' &&
        char_array[(x - 1) as usize][(y + 1) as usize] == 'M' && char_array[(x + 1) as usize][(y - 1) as usize] == 'S' &&
        char_array[(x + 1) as usize][(y + 1) as usize] == 'M' && char_array[(x - 1) as usize][(y - 1) as usize] == 'S'
    {
        /* Check for v\ v/ */
        count += 1;
    }
    if char_array[x][y] == 'A' &&
        char_array[(x - 1) as usize][(y + 1) as usize] == 'M' && char_array[(x + 1) as usize][(y - 1) as usize] == 'S' &&
        char_array[(x - 1) as usize][(y - 1) as usize] == 'M' && char_array[(x + 1) as usize][(y + 1) as usize] == 'S'
    {
        /* Check for v\ ^/ */
        count += 1;
    }
    if char_array[x][y] == 'A' &&
        char_array[(x + 1) as usize][(y - 1) as usize] == 'M' && char_array[(x - 1) as usize][(y + 1) as usize] == 'S' &&
        char_array[(x + 1) as usize][(y + 1) as usize] == 'M' && char_array[(x - 1) as usize][(y - 1) as usize] == 'S'
    {
        /* Check for ^\ v/ */
        count += 1;
    }
    if char_array[x][y] == 'A' &&
        char_array[(x + 1) as usize][(y - 1) as usize] == 'M' && char_array[(x - 1) as usize][(y + 1) as usize] == 'S' &&
        char_array[(x - 1) as usize][(y - 1) as usize] == 'M' && char_array[(x + 1) as usize][(y + 1) as usize] == 'S'
    {
        /* Check for ^\ ^/ */
        count += 1;
    }

    count
}