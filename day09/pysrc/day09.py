
memory_system = []

def find_next_free(memory_system, start_index):
    return next(idx for (idx, x) in enumerate(memory_system) if x == -1 and idx > start_index)

def find_file_block(memory_system, file_id):
    start_index = next(idx for (idx, x) in enumerate(memory_system) if x == file_id)
    for end_idx in range(start_index+1, len(memory_system)):
        if not memory_system[end_idx] == file_id:
            return (start_index, end_idx - start_index)
    return (start_index, len(memory_system) - start_index)

def find_free_space(memory_system, size, max_index):
    start_index = find_next_free(memory_system, -1)
    while start_index < max_index:
        end_index = start_index
        while memory_system[end_index] == -1:
            end_index += 1

        if end_index - start_index >= size:
            return start_index
        else:
            start_index = find_next_free(memory_system, start_index)

    return -1


file_id = 0
with open("E:\\dev\\AoC2024\\day09\\input.txt") as file:
    is_file = True
    for line in file:
        for char in line:
            block_length = ord(char) - ord("0")
            if is_file:
                insert_val = file_id
                file_id += 1
            else:
                insert_val = -1
        
            for _ in range(block_length):
                memory_system.append(insert_val)
            
            is_file = not is_file

# # Part 1 solution
# next_free_index = find_next_free(memory_system, -1)
# for index in reversed(range(len(memory_system))):
#     if index <= next_free_index:
#         break
#     if memory_system[index] > -1:
#         memory_system[next_free_index] = memory_system[index]
#         memory_system[index] = -1
#         next_free_index = find_next_free(memory_system, next_free_index)

# Part 2 solution
for current_id in reversed(range(file_id)):
    (file_start, file_length) = find_file_block(memory_system, current_id)
    free_index = find_free_space(memory_system, file_length, file_start)

    if free_index > -1:
        for block_index in range(file_length):
            memory_system[free_index + block_index] = memory_system[file_start + block_index]
            memory_system[file_start + block_index] = -1

checksum = 0
for index in range(len(memory_system)):
    if memory_system[index] > -1:
        checksum += index * memory_system[index]

print(f"Final checksum is {checksum}")