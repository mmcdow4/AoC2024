import numpy as np

def check_for_xmas(array, x, y, xdir, ydir):
    if x + 3*xdir >= 0 and x + 3*ydir < 140 and y + 3*ydir >= 0 and y + 3*ydir < 140:
        return (array[x][y] == 'X' and array[x+1*xdir][y+1*ydir] == 'M' and array[x+2*xdir][y+2*ydir] == 'A' and array[x+3*xdir][y+3*ydir] == 'S')
    else:
        return False


char_array = np.char.chararray((140,140))

x = 0
y = 0
char_count = 0
with open("E:\\dev\\advent_of_code_2024\\day04\\input.txt") as file:
    new_char = file.read(1)
    while new_char:
        if new_char == 'X' or new_char == 'M' or new_char == 'A' or new_char == 'S':
            char_count += 1
            char_array[x][y] = new_char
            x += 1
            if x == 140:
                x = 0
                y += 1
        new_char = file.read(1)


xmas_count = 0
for x in range(140):
    for y in range(140):
        if check_for_xmas(char_array, x, y, -1, 0):
            # leftwards
            xmas_count += 1
        if check_for_xmas(char_array, x, y, -1, 1):
            # diagonal up-left
            xmas_count += 1
        if check_for_xmas(char_array, x, y, 1, 0):
            # upwards
            xmas_count += 1
        if check_for_xmas(char_array, x, y, 1, 1):
            # diagonal up-right
            xmas_count += 1
        if check_for_xmas(char_array, x, y, 1, 0):
            # rightwards
            xmas_count += 1
        if check_for_xmas(char_array, x, y, 1, -1):
            # diagonal down-right
            xmas_count += 1
        if check_for_xmas(char_array, x, y, 0, -1):
            # downwards
            xmas_count += 1
        if check_for_xmas(char_array, x, y, -1, -1):
            # diagonal down-left
            xmas_count += 1

print("Final xmas count is ", xmas_count)
            