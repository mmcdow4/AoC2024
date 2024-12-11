import math
from collections import defaultdict

def is_antinode_valid(pos, max_x, max_y):
    return pos[0] >= 0 and pos[1] >= 0 and pos[0] < max_x and pos[1] < max_y

def find_antinodes_part1(pos1, pos2, max_x, max_y):
    delta = (pos1[0] - pos2[0], pos1[1] - pos2[1])

    antinode1 = (pos1[0] + delta[0], pos1[1] + delta[1])
    antinode2 = (pos2[0] - delta[0], pos2[1] - delta[1])

    valid_antinodes = []
    if is_antinode_valid(antinode1, max_x, max_y):
        valid_antinodes.append(antinode1)
    if is_antinode_valid(antinode2, max_x, max_y):
        valid_antinodes.append(antinode2)
    return valid_antinodes

def find_antinodes_part2(pos1, pos2, max_x, max_y):
    
    delta = (pos1[0] - pos2[0], pos1[1] - pos2[1])
    gcd = math.gcd(delta[0], delta[1])
    delta = (delta[0] / gcd, delta[1] / gcd)

    valid_antinodes = []
    antinode = pos1
    while True:
        valid_antinodes.append(antinode)
        antinode = (antinode[0] + delta[0], antinode[1] + delta[1])
        if not is_antinode_valid(antinode, max_x, max_y):
            break
    antinode = (pos1[0] - delta[0], pos1[1] - delta[1])
    while True:
        valid_antinodes.append(antinode)
        antinode = (antinode[0] - delta[0], antinode[1] - delta[1])
        if not is_antinode_valid(antinode, max_x, max_y):
            break
    
    return valid_antinodes

antennas = []

max_x = 0
max_y = 0
with open("E:\\dev\\AoC2024\\day08\\input.txt") as file:
    for line in file:
        max_x = 0
        for char in line:
            if char == '\n' or char == '\r':
                break
            elif not char == '.':
                antennas.append((char, (max_x, max_y)))
            max_x += 1
        max_y += 1

antinodes_part1 = set()
antinodes_part2 = set()
for outter_index in range(len(antennas)):
    for inner_index in range(outter_index+1, len(antennas)):
        if antennas[outter_index][0] == antennas[inner_index][0]:
            antinodes_part1.update(find_antinodes_part1(antennas[outter_index][1], antennas[inner_index][1], max_x, max_y))
            antinodes_part2.update(find_antinodes_part2(antennas[outter_index][1], antennas[inner_index][1], max_x, max_y))

print(f"Found {len(antinodes_part1)} antinodes originally, then {len(antinodes_part2)} when accounting for subharmonics")

