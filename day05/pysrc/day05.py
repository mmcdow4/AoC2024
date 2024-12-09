import numpy as np

reading_rules = True

rules_vec = list()
updates_vec = list()
with open("E:\\dev\\advent_of_code_2024\\day05\\input.txt") as file:
    line = file.readline()
    while line:
        if line == '\n':
            reading_rules = False
        elif reading_rules:
            rules_vec.append(list(map(int, line.split("|"))))
        else:
            updates_vec.append(list(map(int, line.split(","))))
        line = file.readline()

mid_total = 0
corrected_mid_total = 0
for update in updates_vec:
    correct = True
    rerun = True
    while rerun:
        rerun = False
        for rule in rules_vec:
            if rule[0] in update and rule[1] in update and update.index(rule[0]) > update.index(rule[1]):
                idx1 = update.index(rule[0])
                idx2 = update.index(rule[1])
                correct = False
                rerun = True
                temp_copy = []
                # copy whatever was before the second rule element
                if idx2 > 0:
                    temp_copy.extend(update[0:idx2])
                #insert the first rule element into the corrected position
                temp_copy.append(rule[0])
                #copy whatever was between them
                temp_copy.extend(update[idx2:idx1])
                temp_copy.extend(update[idx1+1:])
                update = temp_copy
    
    midpoint_idx = int((len(update) - 1) / 2)
    if correct:
        mid_total += update[midpoint_idx]
    else:
        corrected_mid_total += update[midpoint_idx]


print("Sum of correct midpoints is ", mid_total)
print("Sum of corrected midpoints is ", corrected_mid_total)
