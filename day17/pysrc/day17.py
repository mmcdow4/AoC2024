import re

class Computer:

    def __init__(self, register_a):
        self.register_a = register_a
        self.register_b = 0
        self.register_c = 0
        self.instr_ptr = 0
        self.output = list()

    def reset(self, register_a):
        self.register_a = register_a
        self.register_b = 0
        self.register_c = 0
        self.instr_ptr = 0
        self.output = list()

    def combo_operand(self, operand):
        if operand < 4:
            return operand
        elif operand == 4:
            return self.register_a
        elif operand == 5:
            return self.register_b
        elif operand == 6:
            return self.register_c
        else:
            print(f"Invalid operand {operand}")
            exit(-1)
    
    # OpCode 0
    def adv(self, operand):
        self.register_a = self.register_a >> self.combo_operand(operand)
        self.instr_ptr += 2

    # OpCode 1
    def bxl(self, operand):
        self.register_b = self.register_b ^ operand
        self.instr_ptr += 2

    # OpCode 2
    def bst(self, operand):
        self.register_b = self.combo_operand(operand) & 0x7
        self.instr_ptr += 2

    # OpCode 3
    def jnz(self, operand):
        if self.register_a == 0:
            self.instr_ptr += 2
        else:
            self.instr_ptr = operand

    # OpCode 4
    def bxc(self, operand):
        self.register_b = self.register_b ^ self.register_c
        self.instr_ptr += 2

    # OpCode 5
    def out(self, operand):
        new_output = self.combo_operand(operand) & 0x7
        self.output.append(new_output)
        self.instr_ptr += 2

    # OpCode 6
    def bdv(self, operand):
        self.register_b = self.register_aa >> self.combo_operand(operand)
        self.instr_ptr += 2

    # OpCode 7
    def cdv(self, operand):
        self.register_c = self.register_a >> self.combo_operand(operand)
        self.instr_ptr += 2

    def execute(self, program):
        self.instr_ptr = 0
        self.output = list()
        while self.instr_ptr < len(program):
            if program[self.instr_ptr] == 0:
                self.adv(program[self.instr_ptr + 1])
            elif program[self.instr_ptr] == 1:
                self.bxl(program[self.instr_ptr + 1])
            elif program[self.instr_ptr] == 2:
                self.bst(program[self.instr_ptr + 1])
            elif program[self.instr_ptr] == 3:
                self.jnz(program[self.instr_ptr + 1])
            elif program[self.instr_ptr] == 4:
                self.bxc(program[self.instr_ptr + 1])
            elif program[self.instr_ptr] == 5:
                self.out(program[self.instr_ptr + 1])
            elif program[self.instr_ptr] == 6:
                self.bdv(program[self.instr_ptr + 1])
            elif program[self.instr_ptr] == 7:
                self.cdv(program[self.instr_ptr + 1])
            else:
                print(f"Illegal opcode {program[self.instr_ptr]}")

        return self.output


register_a = 0
program = list()

with open("E:\\dev\\AoC2024\\day17\\input.txt") as file:
    line = file.readline()
    matches = re.findall("Register A: (\d+)", line)
    register_a = int(matches[0])
    line = file.readline()
    line = file.readline()
    line = file.readline()
    line = file.readline()
    program = list(map(int, line.replace("Program: ", "").split(",")))

computer = Computer(register_a)

output = computer.execute(program)

print(f"output with initial A value: {output}")

a_candidates = [0]
for index in range(len(program)-1, -1, -1):
    item = program[index]
    new_candidates = []
    for a in a_candidates:
        for bottom_bits in range(8):
            temp_a = (a << 3) + bottom_bits
            computer.reset(temp_a)
            output = computer.execute(program)
            if output == program[index:]:
                new_candidates.append(temp_a)
    a_candidates = new_candidates

print(f"Now validating {len(a_candidates)} candidates")
for a in a_candidates:
    computer.reset(a)
    output = computer.execute(program)
    if output == program:
        print(f"Program was reproduced for A initialzied to {a}")
        break