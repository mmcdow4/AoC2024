import re
import numpy as np
import sys

class robot:
    def __init__(self, pos, vel, dim):
        self.pos = pos
        self.vel = vel
        self.dim = dim
    
    def take_n_steps(self, n):
        final_x = (self.pos[0] + n * self.vel[0]) % self.dim[0]
        final_y = (self.pos[1] + n * self.vel[1]) % self.dim[1]

        if final_x < 0:
            final_x += self.dim[0]
        if final_y < 0:
            final_y += self.dim[1]
        
        return (final_x, final_y)
    

def quadrant(pos, dim):
    if pos[0] < (dim[0] - 1) / 2:
        if pos[1] < (dim[1] - 1) / 2:
            return 2 # Quadrant II
        elif pos[1] > (dim[1] - 1) / 2:
            return 3 # Quadrant III
    elif pos[0] > (dim[0] - 1) / 2:
        if pos[1] < (dim[1] - 1) / 2:
            return 1 # Quadrant I
        elif pos[1] > (dim[1] - 1) / 2:
            return 4 # Quadrant IV
    return 0 # On a boundary, no quadrant

def compute_entropy(minimap, N):
    entropy = 0
    for y in range(0, len(minimap)):
        for x in range(0, len(minimap[y])):
            if minimap[y][x] > 0:
                p_i = minimap[y][x] / N
                entropy -= p_i * np.log2(p_i)
    return entropy

robots = list()
max_x = int(sys.argv[1])
max_y = int(sys.argv[2])
dim = (max_x, max_y)
with open("E:\\dev\\AoC2024\\day14\\input.txt") as file:
    for line in file:
        pos = list(map(int, re.findall("p=(\d+),(\d+)", line)[0]))
        vel = list(map(int, re.findall("v=(-?\d+),(-?\d+)", line)[0]))
        robots.append(robot(pos, vel, dim))


# Part 1
quadrant_counts = {
    0: 0,
    1: 0,
    2: 0,
    3: 0,
    4: 0
}

for robot in robots:
    final_pos = robot.take_n_steps(100)
    quadrant_counts[quadrant(final_pos, dim)] += 1
safety_factor = quadrant_counts[1] * quadrant_counts[2] * quadrant_counts[3] * quadrant_counts[4]
print(f"Final safety factor after 100 seconds = {safety_factor} from counts {quadrant_counts}")

average_entropy = 0
for seconds in range(0, 10):
    minimap = np.zeros(shape=(int(dim[1] / 3) + 1, int(dim[0] / 3) + 1))
    for robot in robots:
        final_pos = robot.take_n_steps(seconds)
        minimap[int(final_pos[1] / 3)][int(final_pos[0] / 3)] += 1
        
    average_entropy += compute_entropy(minimap, len(robots)) / 10
entropy_threshold = average_entropy * 0.9
lowest_entropy = sys.float_info.max
seconds = 101
print(f"Searching for low entropy cases, average of first 100 was {average_entropy}, creating a threshold of {entropy_threshold}")
while True:
    minimap = np.zeros(shape=(int(dim[1] / 3) + 1, int(dim[0] / 3) + 1))
    for robot in robots:
        final_pos = robot.take_n_steps(seconds)
        minimap[int(final_pos[1] / 3)][int(final_pos[0] / 3)] += 1

    lowest_entropy = min(lowest_entropy, compute_entropy(minimap, len(robots)))
    if lowest_entropy < entropy_threshold:
        print(f"I think the easter egg occurs at {seconds} seconds based on entropy of {lowest_entropy}.")
        break
    seconds += 1