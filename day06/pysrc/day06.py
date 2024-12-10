import numpy as np
import copy
from typing import Final

MAX_X: Final[int] = 130
MAX_Y: Final[int] = 130

NORTHWARD: Final[int] = 0x1
EASTWARD: Final[int] = 0x2
SOUTHWARD: Final[int] = 0x4
WESTWARD: Final[int] = 0x8

class Tile:
    def __init__(self, occupied=False, loop_counted=False, visited=0x0):
        self.occupied = occupied
        self.loop_counted = loop_counted
        self.visited = visited
    
    def copy(self):
        return Tile(occupied=self.occupied, loop_counted=self.loop_counted, visited=self.visited)

    def update_visited(self, direction):
        if direction == 'N':
            self.visited = self.visited | NORTHWARD
        elif direction == 'E':
            self.visited = self.visited | EASTWARD
        elif direction == 'S':
            self.visited = self.visited | SOUTHWARD
        elif direction == 'W':
            self.visited = self.visited | WESTWARD
    
    def is_revisit(self, direction):
        if direction == 'N':
            return (self.visited & NORTHWARD > 0)
        elif direction == 'E':
            return (self.visited & EASTWARD > 0)
        elif direction == 'S':
            return (self.visited & SOUTHWARD > 0)
        elif direction == 'W':
            return (self.visited & WESTWARD > 0)
        
    def is_loop_point(self, direction):
        if direction == 'N' and (self.visited & EASTWARD > 0):
            return True
        elif direction == 'E' and (self.visited & SOUTHWARD > 0):
            return True
        elif direction == 'S' and (self.visited & WESTWARD > 0):
            return True
        elif direction == 'W' and (self.visited & NORTHWARD > 0):
            return True
        else:
            return False
            

class Guard:
    def __init__(self, pos, dir):
        self.pos = pos
        self.direction = dir
    
    def copy(self):
        return Guard(self.pos, self.direction)
    
    def next_step(self):
        next_pos = (np.nan, np.nan)
        if self.direction == 'N' and self.pos[1] > 0:
            next_pos = (self.pos[0], self.pos[1]-1)
        elif self.direction == 'E' and self.pos[0]+1 < MAX_X:
            next_pos = (self.pos[0]+1, self.pos[1])
        elif self.direction ==  'S' and self.pos[1]+1 < MAX_Y:
            next_pos = (self.pos[0], self.pos[1]+1)
        elif self.direction == 'W' and self.pos[0] > 0:
            next_pos = (self.pos[0]-1, self.pos[1])
        
        return next_pos
    
    def advance(self):
        self.pos = self.next_step()
    
    def turn(self):
        if self.direction == 'N':
            self.direction = 'E'
        elif self.direction == 'E':
            self.direction = 'S'
        elif self.direction == 'S':
            self.direction = 'W'
        elif self.direction == 'W':
            self.direction = 'N'

def parse_input(filename):
    location_array = [[Tile() for i in range(MAX_X)] for j in range(MAX_Y)]

    x = 0
    y = 0
    with open(filename) as file:
        while 1:
            valid_char = True
            char = file.read(1)
            if not char:
                break
            elif char == '.':
                location_array[x][y].occupied = False
            elif char == '#':
                location_array[x][y].occupied = True
            elif char == '^':
                my_guard = Guard((x, y), 'N')
                location_array[x][y].loop_counted = True
                location_array[x][y].update_visited('N')
            elif char in "\r\n":
                valid_char = False

            if valid_char:
                x = (x + 1) % MAX_X
                if x == 0:
                    y += 1

    return (location_array, my_guard)

def test_for_guard_loop(location_array, my_guard):
    next_pos = my_guard.next_step()
    if location_array[next_pos[0]][next_pos[1]].occupied:
        return False
    location_array[next_pos[0]][next_pos[1]].occupied = True
    while not np.isnan(next_pos[0]):
        if location_array[next_pos[0]][next_pos[1]].occupied:
            my_guard.turn()
        else:
            my_guard.advance()
            if location_array[my_guard.pos[0]][my_guard.pos[1]].is_revisit(my_guard.direction):
                # print("Loop discovered!")
                return True
        ##print("Loop test guard is now at position: ", my_guard.pos)
        location_array[my_guard.pos[0]][my_guard.pos[1]].update_visited(my_guard.direction)
        next_pos = my_guard.next_step()
    # print("Loop test failed!")
    return False

(location_array, my_guard) = parse_input("E:\\dev\\AoC2024\\day06\\input.txt")

space_count = 1
loop_count = 0
next_pos = my_guard.next_step()

# print("Guard is starting at position: ", my_guard.pos)

while not np.isnan(next_pos[0]):
    if location_array[next_pos[0]][next_pos[1]].occupied:
        my_guard.turn()
    else:
        if not location_array[next_pos[0]][next_pos[1]].loop_counted and (location_array[next_pos[0]][next_pos[1]].visited > 0) and test_for_guard_loop(copy.deepcopy(location_array), my_guard.copy()):#location_array[my_guard.pos[0]][my_guard.pos[1]].is_loop_point(my_guard.direction):
            loop_count += 1
            location_array[next_pos[0]][next_pos[1]].loop_counted = True
        my_guard.advance()
        if location_array[my_guard.pos[0]][my_guard.pos[1]].visited == 0:
            space_count += 1
        
    # print("Guard is now at position: ", my_guard.pos)
    location_array[my_guard.pos[0]][my_guard.pos[1]].update_visited(my_guard.direction)
    next_pos = my_guard.next_step()

print("Guard visited ", space_count, " unique locations, and could enter a loop at ", loop_count)