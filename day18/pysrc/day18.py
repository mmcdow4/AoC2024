from collections import deque
import sys
import numpy as np

def pos_to_index(pos, max_x):
    return pos[1] * max_x + pos[0]

def index_to_pos(index, max_x):
    y = int(index / max_x)
    x = index - y
    return (x, y)

def find_best_path(memory, max_x, max_y):
    pos_queue = deque()
    pos_queue.append((0, 0, 0)) # (x, y, path_length)
    end_point = (max_x - 1, max_y - 1)
    visited = np.zeros(max_x * max_y)
    while len(pos_queue) > 0:
        current_pos = pos_queue.popleft()
        if current_pos[0] == end_point[0] and current_pos[1] == end_point[1]:
            return current_pos[2]
    
        if current_pos[0] > 0:
            next_idx = pos_to_index((current_pos[0] - 1, current_pos[1]), max_x)
            if memory[next_idx] == 0 and visited[next_idx] == 0:
                next_pos = (current_pos[0] - 1, current_pos[1], current_pos[2] + 1)
                pos_queue.append(next_pos)
                visited[next_idx] = 1
        if current_pos[1] > 0:
            next_idx = pos_to_index((current_pos[0], current_pos[1] - 1), max_x)
            if memory[next_idx] == 0 and visited[next_idx] == 0:
                next_pos = (current_pos[0], current_pos[1] - 1, current_pos[2] + 1)
                pos_queue.append(next_pos)
                visited[next_idx] = 1
        if current_pos[0] < max_x - 1:
            next_idx = pos_to_index((current_pos[0] + 1, current_pos[1]), max_x)
            if memory[next_idx] == 0 and visited[next_idx] == 0:
                next_pos = (current_pos[0] + 1, current_pos[1], current_pos[2] + 1)
                pos_queue.append(next_pos)
                visited[next_idx] = 1
        if current_pos[1] < max_y - 1:
            next_idx = pos_to_index((current_pos[0], current_pos[1] + 1), max_x)
            if memory[next_idx] == 0 and visited[next_idx] == 0:
                next_pos = (current_pos[0], current_pos[1] + 1, current_pos[2] + 1)
                pos_queue.append(next_pos)
                visited[next_idx] = 1
    return -1


max_x = int(sys.argv[1])
max_y = int(sys.argv[2])
input_file = sys.argv[3]
first_bytes = int(sys.argv[4])

memory = np.zeros(max_x * max_y)
order = list()

with open("E:\\dev\\AoC2024\\day18\\" + input_file) as file:
    for line in file:
        indices = list(map(int, line.split(",")))
        order.append((indices[0], indices[1]))

for i in range(first_bytes):
    memory[pos_to_index(order[i], max_x)] = 1

best_path = find_best_path(memory, max_x, max_y)

print(f"Best path found was {best_path} steps")

next_byte = first_bytes

while next_byte < len(order):
    # Drop the next byte
    memory[pos_to_index(order[next_byte], max_x)] = 1
    # Try to find a path
    if find_best_path(memory, max_x, max_y) > 0:
        # There is a path, try again
        next_byte += 1
    else:
        # Failed to find a path to the exit
        print(f"First byte to block off the exit is ({order[next_byte][0]}, {order[next_byte][1]})")
        break
    
    
