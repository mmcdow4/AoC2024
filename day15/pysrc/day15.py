from enum import Enum
import numpy as np

class Tile(Enum):
    EMPTY = 1
    ROBOT = 2
    WALL = 3
    BOX = 4
    WIDE_BOX_LEFT = 5
    WIDE_BOX_RIGHT = 6

def can_move(warehouse, pos, delta):
    next_pos = (pos[0] + delta[0], pos[1] + delta[1])
    if warehouse[next_pos[1]][next_pos[0]] == Tile.EMPTY:
        return True
    elif warehouse[next_pos[1]][next_pos[0]] == Tile.WALL:
        return False
    elif warehouse[next_pos[1]][next_pos[0]] == Tile.BOX:
        return can_move(warehouse, next_pos, delta)
    elif warehouse[next_pos[1]][next_pos[0]] == Tile.WIDE_BOX_LEFT:
        right_pos = (next_pos[0] + 1, next_pos[1])
        if delta[0] == 1:
            #Moving horizontal, which should mean heading eastward, so just check the right
            return can_move(warehouse, right_pos, delta)
        elif not delta[1] == 0:
            #Moving vertical, need both sides to be able to move
            return can_move(warehouse, next_pos, delta) and can_move(warehouse, right_pos, delta)
        else:
            print(f"Error! Checking if we can move a left box with unexpected velocity ({delta[0]}, {delta[1]})")
            exit(-1)
    elif warehouse[next_pos[1]][next_pos[0]] == Tile.WIDE_BOX_RIGHT:
        left_pos = (next_pos[0] - 1, next_pos[1])
        if delta[0] == -1:
            #Moving horizontal, which should mean heading westward, so just check the left
            return can_move(warehouse, left_pos, delta)
        elif not delta[1] == 0:
            #Moving vertical, need both sides to be able to move
            return can_move(warehouse, next_pos, delta) and can_move(warehouse, left_pos, delta)
        else:
            print(f"Error! Checking if we can move a right box with unexpected velocity ({delta[0]}, {delta[1]})")
            exit(-1)
    else:
        print(f"Error! Checking if we can move into a tile holding the robot!")
        exit(-1)

def move_tile(warehouse, pos, delta, incoming):
    next_pos = (pos[0] + delta[0], pos[1] + delta[1])
    prev_value = warehouse[pos[1]][pos[0]]
    warehouse[pos[1]][pos[0]] = incoming
    if prev_value == Tile.BOX:
        move_tile(warehouse, next_pos, delta, Tile.BOX)
    elif prev_value == Tile.WIDE_BOX_LEFT:
        if delta[0] == 1:
            #Moving horizontal, which should mean heading eastward, so just move the right
            warehouse[next_pos[1]][next_pos[0]] = Tile.WIDE_BOX_LEFT
            next_pos = (next_pos[0] + delta[0], next_pos[1])
            move_tile(warehouse, next_pos, delta, Tile.WIDE_BOX_RIGHT)
        elif not delta[1] == 0:
            #Moving vertical, need to move both sides
            right_pos = (next_pos[0] + 1, next_pos[1])
            warehouse[pos[1]][pos[0] + 1] = Tile.EMPTY
            move_tile(warehouse, next_pos, delta, Tile.WIDE_BOX_LEFT)
            move_tile(warehouse, right_pos, delta, Tile.WIDE_BOX_RIGHT)
        else:
            print(f"Error! Moving a left box with unexpected velocity ({delta[0]}, {delta[1]})")
            exit(-1)
    elif prev_value == Tile.WIDE_BOX_RIGHT:
        if delta[0] == -1:
            #Moving horizontal, which should mean heading westward, so just move the left
            warehouse[next_pos[1]][next_pos[0]] = Tile.WIDE_BOX_RIGHT
            next_pos = (next_pos[0] + delta[0], next_pos[1])
            move_tile(warehouse, next_pos, delta, Tile.WIDE_BOX_LEFT)
        elif not delta[1] == 0:
            #Moving vertical, need to move both sides
            left_pos = (next_pos[0] - 1, next_pos[1])
            warehouse[pos[1]][pos[0] - 1] = Tile.EMPTY
            move_tile(warehouse, left_pos, delta, Tile.WIDE_BOX_LEFT)
            move_tile(warehouse, next_pos, delta, Tile.WIDE_BOX_RIGHT)
        else:
            print(f"Error! Moving a right box with unexpected velocity ({delta[0]}, {delta[1]})")
            exit(-1)
    elif prev_value == Tile.ROBOT:
        print(f"Error! Attempting to move into a square already containing a robot!")
        exit(-1)
    elif prev_value == Tile.WALL:
        print(f"Error! Attempting to move into a wall!")
        exit(-1)


def compute_gps_sum(warehouse):
    (max_y, max_x) = warehouse.shape
    gps_sum = 0
    for x in range(max_x):
        for y in range(max_y):
            if warehouse[y][x] == Tile.BOX or warehouse[y][x] == Tile.WIDE_BOX_LEFT:
                gps_sum += y * 100 + x
    return gps_sum

def print_warehouse(warehouse, filename):
    (max_y, max_x) = warehouse.shape
    file = open(filename, "w")
    for y in range(max_y):
        for x in range(max_x):
            if warehouse[y][x] == Tile.WALL:
                file.write("#")
            if warehouse[y][x] == Tile.EMPTY:
                file.write(".")
            if warehouse[y][x] == Tile.BOX:
                file.write("O")
            if warehouse[y][x] == Tile.WIDE_BOX_LEFT:
                file.write("[")
            if warehouse[y][x] == Tile.WIDE_BOX_RIGHT:
                file.write("]")
            if warehouse[y][x] == Tile.ROBOT:
                file.write("@")
        file.write("\n")

x = 0
y = 0
with open("E:\\dev\\AoC2024\\day15\\input.txt") as file:
    for line in file:
        if '#' in line:
            y += 1
            x = len(line.replace("\n\r", ""))
        else:
            break

warehouse = np.empty(shape=(y, 2 * x), dtype=Tile)
warehouse.fill(Tile.EMPTY)
move_sequence = list()
robot_pos = (0, 0)
with open("E:\\dev\\AoC2024\\day15\\input.txt") as file:
    y = 0
    for line in file:
        if '#' in line:
            x = 0
            # Part 1 parsing
            # for char in line:
            #     if char == '#':
            #         warehouse[y][x] = Tile.WALL
            #     elif char == 'O':
            #         warehouse[y][x] = Tile.BOX
            #     elif char == '@':
            #         warehouse[y][x] = Tile.ROBOT
            #         robot_pos = (x, y)
            #     elif not char == '.':
            #         break
            #     x += 1
            # Part 2 parsing
            for char in line:
                if char == '#':
                    warehouse[y][x] = Tile.WALL
                    warehouse[y][x+1] = Tile.WALL
                elif char == 'O':
                    warehouse[y][x] = Tile.WIDE_BOX_LEFT
                    warehouse[y][x+1] = Tile.WIDE_BOX_RIGHT
                elif char == '@':
                    warehouse[y][x] = Tile.ROBOT
                    robot_pos = (x, y)
                elif not char == '.':
                    break
                x += 2
            y += 1
        elif '^' in line:
            for char in line:
                if char == '^':
                    move_sequence.append((0, -1)) # Move up
                elif char == '>':
                    move_sequence.append((1, 0)) # Move right
                elif char == 'v':
                    move_sequence.append((0, 1)) # Move down
                elif char == '<':
                    move_sequence.append((-1, 0)) # Move left

print_warehouse(warehouse, "E:\\dev\\AoC2024\\day15\\input_map_python.txt")
for (index, move) in enumerate(move_sequence):
    if can_move(warehouse, robot_pos, move):
        warehouse[robot_pos[1]][robot_pos[0]] = Tile.EMPTY
        robot_pos = (robot_pos[0] + move[0], robot_pos[1] + move[1])
        move_tile(warehouse, robot_pos, move, Tile.ROBOT)

print_warehouse(warehouse, "E:\\dev\\AoC2024\\day15\\final_map_python.txt")
gps_sum = compute_gps_sum(warehouse)

print(f"Final gps sum is {gps_sum}")