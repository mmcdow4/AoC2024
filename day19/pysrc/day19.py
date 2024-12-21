import sys

input = sys.argv[1]

def is_pattern_possible(pattern, towel_types, cache):
    if pattern in cache:
        return cache[pattern]
    combinations = 0
    for towel in towel_types:
        if pattern == towel:
            combinations += 1
        elif pattern.startswith(towel):
            combinations += is_pattern_possible(pattern[len(towel):], towel_types, cache)
    cache.update({pattern: combinations})
    return combinations


with open(f"E:\\dev\\AoC2024\\day19\\{input}") as file:
    towel_types = list(file.readline().replace("\n", "").split(", "))
    cache = dict()
    
    #skip the blank line
    file.readline()

    possible_count = 0
    combination_count = 0
    for line in file:
        line = line.replace("\n", "")
        new_combinations = is_pattern_possible(line, towel_types, cache)
        if new_combinations > 0:
            possible_count += 1
            combination_count += new_combinations
    
    print(f"{possible_count} patterns are possible, in a total of {combination_count} combinations")