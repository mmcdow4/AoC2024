from enum import Enum
import numpy as np
import queue

class Direction(Enum):
    North = 1
    East = 2
    South = 3
    West = 4

class Tile(Enum):
    Wall = 1
    Path = 2
    Start = 3
    End = 4

def turn(direction):
    if direction == Direction.North:
        return Direction.East
    elif direction == Direction.East:
        return Direction.South
    elif direction == Direction.South:
        return Direction.West
    else:
        return Direction.North
    
class Pos:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __add__(self, other):
        return Pos(self.x + other.x, self.y + other.y)
    
    def __hash__(self):
        return hash(self.y * 1000 + self.x)
    
    def __eq__(self, other):
        return self.x == other.x and self.y == other.y
    
    def next_step(self, direction, max_x, max_y):
        if direction == Direction.North and self.y > 0:
            return Pos(self.x, self.y - 1)
        elif direction == Direction.East and self.x < max_x - 1:
            return Pos(self.x + 1, self.y)
        elif direction == Direction.South and self.y < max_y - 1:
            return Pos(self.x, self.y + 1)
        elif direction == Direction.West and self.x > 0:
            return Pos(self.x - 1, self.y)
        return Pos(np.nan, np.nan)

def find_cheats(course, visited, pos, ps_counter, cheats, max_x, max_y):
    pos_queue = queue.Queue()
    checked_spaces = {pos}
    pos_queue.put((0, False, pos))
    while not pos_queue.empty():
        (ps_added, through_wall, current_pos) = pos_queue.get()
        if (through_wall and
                course[current_pos.y][current_pos.x] != Tile.Wall and
                visited[current_pos.y][current_pos.x] == 0):
            if current_pos in cheats:
                cheats[current_pos].append(ps_counter + ps_added)
            else:
                cheats.update({current_pos: [ps_counter + ps_added]})
        elif course[current_pos.y][current_pos.x] == Tile.Wall:
            through_wall = True
        
        if ps_added < 20:
            direction = Direction.North
            for _ in range(4):
                next_pos = current_pos.next_step(direction, max_x, max_y)
                if not np.isnan(next_pos.x) and not next_pos in checked_spaces:
                    pos_queue.put((ps_added + 1, through_wall, next_pos))
                    checked_spaces.add(next_pos)
                direction = turn(direction)

def run_course(course, start_pos, end_pos, max_x, max_y):
    current_pos = start_pos
    ps_counter = 1
    visited = np.zeros(shape=(max_x, max_y))
    cheats = dict()
    while True:
        visited[current_pos.y][current_pos.x] = ps_counter
        if course[current_pos.y][current_pos.x] == Tile.End:
            break

        find_cheats(course, visited, current_pos, ps_counter, cheats, max_x, max_y)
        
        direction = Direction.North
        for _ in range(4):
            next_pos = current_pos.next_step(direction, max_x, max_y)
            if (not np.isnan(next_pos.x) and
                    course[next_pos.y][next_pos.x] != Tile.Wall and
                    visited[next_pos.y][next_pos.x] == 0):
                current_pos = next_pos
                break
            direction = turn(direction)
        ps_counter += 1

    num_100ps_cheats = 0
    for pos, times in cheats.items():
        for time in times:
            if visited[pos.y][pos.x] - time > 100:
                num_100ps_cheats += 1

    return num_100ps_cheats


course = list()
max_y = 0
with open("E:\\dev\\AoC2024\\day20\\input.txt") as file:
    for line in file:
        max_x = 0
        course.append(list())
        for ch in line:
            if ch == '#':
                course[max_y].append(Tile.Wall)
            elif ch == '.':
                course[max_y].append(Tile.Path)
            elif ch == 'S':
                course[max_y].append(Tile.Start)
                start_pos = Pos(max_x, max_y)
            elif ch == 'E':
                course[max_y].append(Tile.End)
                end_pos = Pos(max_x, max_y)
            else:
                break
            max_x += 1
        max_y += 1


num_100ps_cheats = run_course(course, start_pos, end_pos, max_x, max_y)
print(f"Number of cheats that save 100ps or more is {num_100ps_cheats}")