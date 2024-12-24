from collections import Counter

def next_value(value):
    if value == 0:
        return [1]
    num_digits = len(str(value))
    if num_digits % 2 == 0:
        divisor = 10 ** (num_digits // 2)
        return [value // divisor, value % divisor]
    else:
        return [value * 2024]

def blink(stone_line, cache):
    new_stones = Counter()
    for stone, count in stone_line.items():
        if stone not in cache:
            cache[stone] = next_value(stone)
        for value in cache[stone]:
            new_stones[value] += count
    return new_stones


with open("E:\\dev\\AoC2024\\day11\\input.txt") as file:
    values = list(map(int, file.readline().split(' ')))

stone_line = Counter(values)
cache = {}
blink_count = 1
for _ in range(25):
    stone_line = blink(stone_line, cache)
    blink_count += 1

stone_count = sum(stone_line.values())

print(f"After 25 blinks, the stone line is {stone_count} long")

for _ in range(50):
    stone_line = blink(stone_line, cache)

stone_count = sum(stone_line.values())

print(f"After 75 blinks, the stone line is {stone_count} long")