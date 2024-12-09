import numpy as np

data = np.genfromtxt('E:\\dev\\advent_of_code_2024\\day01\\input.txt')

data[:,0].sort()
data[:,1].sort()

total = sum(abs(data[:,0] - data[:,1]))

print("First Total: ", total)

total = 0
for x in data[:,0]:
    cnt = sum(data[:,1] == x)
    total += x * cnt

print("Second Total: ", total)