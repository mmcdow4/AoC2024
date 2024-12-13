import math

class Stone:
    def __init__(self, value):
        self.value = value
        self.next = None
    
    def value(self):
        return self.value
    
    def blink(self):
        if self.value == 0:
            self.value = 1
        elif (math.ceil(math.log10(self.value + 0.01)) % 2) == 0:
            num_digits = math.ceil(math.log10(self.value + 0.01)) / 2
            new_value = int(math.floor(self.value / (10 ** num_digits)))
            next_value = int(self.value - new_value * (10 ** num_digits))
            new_stone = Stone(next_value)
            new_stone.next = self.next

            self.value = new_value
            self.next = new_stone
            return True
        else:
            self.value = self.value * 2024
        
        return False

class LinkedList:
    def __init__(self):
        self.head = None
        self.tail = None
        self.len = 0
    
    def append(self, value):
        new_stone = Stone(value)
        if self.tail == None:
            self.head = new_stone
            self.tail = new_stone
            print(f"New value {value} added as the list head")
        else:
            self.tail.next = new_stone
            self.tail = new_stone
            print(f"New value {value} appended to the tail")
        self.len += 1
    
    def blink(self):
        curr_node = self.head
        while not curr_node == None:
            if curr_node.blink():
                self.len += 1
                curr_node = curr_node.next #skip the node we just added for this iteration
            curr_node = curr_node.next


def print_list(stone_line):
    out_str = ""
    curr_stone = stone_line.head
    while not curr_stone.next == None:
        out_str += str(curr_stone.value) + " "
        curr_stone = curr_stone.next
    print(f"Stone Line = [{out_str}]")

stone_line = LinkedList()

with open("E:\\dev\\AoC2024\\day11\\input.txt") as file:
    values = list(map(int, file.readline().split(' ')))
    for value in values:
        stone_line.append(value)


blink_count = 0
#print_list(stone_line)
for _ in range(25):
    stone_line.blink()
    blink_count += 1
    print(f"Blinked {blink_count} times")
    #print_list(stone_line)

print(f"After 25 blinks, the stone line is {stone_line.len} long")

for _ in range(50):
    stone_line.blink()
    blink_count += 1
    print(f"Blinked {blink_count} times")

print(f"After 75 blinks, the stone line is {stone_line.len} long")