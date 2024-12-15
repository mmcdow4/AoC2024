import numpy as np
import re


total_tokens_1 = 0
total_tokens_2 = 0

offset = np.array([[10000000000000], [10000000000000]])
with open("E:\\dev\\AoC2024\\day13\\input.txt") as file:
    for (index, line) in enumerate(file.readlines()):
        if index % 4 == 0:
            a = list(map(int, re.findall("Button A: X\+(\d+), Y\+(\d+)", line)[0]))
        elif index % 4 == 1:
            b = list(map(int, re.findall("Button B: X\+(\d+), Y\+(\d+)", line)[0]))
        elif index % 4 == 2:
            prize = list(map(int, re.findall("Prize: X=(\d+), Y=(\d+)", line)[0]))
            prize = np.array([[prize[0]], [prize[1]]])
            offset_prize = prize + offset
        else:
            A = np.array([[a[0], b[0]], [a[1], b[1]]])
            soln = np.matmul(np.linalg.inv(A), prize)
            if np.round(soln[0]) * a[0] + np.round(soln[1]) * b[0] == prize[0] and np.round(soln[0]) * a[1] + np.round(soln[1]) * b[1] == prize[1]:
                total_tokens_1 += np.round(soln[0]) * 3 + np.round(soln[1])
            
            prize = np.array([[]])
            soln = np.matmul(np.linalg.inv(A), offset_prize)
            if np.round(soln[0]) * a[0] + np.round(soln[1]) * b[0] == offset_prize[0] and np.round(soln[0]) * a[1] + np.round(soln[1]) * b[1] == offset_prize[1]:
                total_tokens_2 += np.round(soln[0]) * 3 + np.round(soln[1])
            
print(f"Total token cost is {int(np.round(total_tokens_1))}, corrected cost is {int(np.round(total_tokens_2))}")