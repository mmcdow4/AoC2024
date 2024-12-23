from collections import deque

class Deltas:
    def __init__(self):
        self.deltas = deque()
        self.output_strings = set()

    def append(self, new_value):
        self.deltas.append(new_value)
        if len(self.deltas) == 5:
            self.deltas.popleft()
        
    
    def as_string(self):
        if len(self.deltas) == 4:
            output = "{},{},{},{}".format(self.deltas[0], self.deltas[1], self.deltas[2], self.deltas[3])
            if not output in self.output_strings:
                self.output_strings.add(output)
                return output
        return ""
    
def next_secret_number(secret_number):
    next_number = (secret_number ^ (secret_number << 6)) % 16777216
    next_number = (next_number ^ (next_number >> 5)) % 16777216
    next_number = (next_number ^ (next_number << 11)) % 16777216
    return next_number


total = 0
possible_totals = dict()
with open("E:\\dev\\AoC2024\\day22\\input.txt") as file:
    for line in file:
        prev_number = int(line)
        deltas = Deltas()
        for _ in range(2000):
            next_number = next_secret_number(prev_number)
            price = (next_number % 10)
            deltas.append(price - (prev_number % 10))
            sequence = deltas.as_string()
            if len(sequence) > 0:
                if sequence in possible_totals.keys():
                    possible_totals[sequence] += price
                else:
                    possible_totals.update({sequence: price})
            prev_number = next_number

        total += prev_number

best_total = 0
best_sequence = ""
for sequence, profit in possible_totals.items():
    if profit > best_total:
        best_total = profit
        best_sequence = sequence

print(f"The total sum of all 2000th secret numbers is {total}")
print(f"The best profit {best_total} can be made with sequence {best_sequence}")