import math

def compute(a, b, operation):
    if operation == 0:
        return a + b
    elif operation == 1:
        return a * b
    elif operation == 2:
        exponent = math.ceil(math.log10(b + 0.01))
        return a * (10 ** exponent) + b
    else:
        print("INVALID OPERATOR")
        return math.nan
    
def test_permutation(test_value, inputs, operations):
    result = compute(inputs[0], inputs[1], operations[0])
    for op_index in range(1, len(operations)):
        result = compute(result, inputs[op_index+1], operations[op_index])
        if result > test_value:
            return False
    return test_value == result

def next_permutation(operations, operation_modulus):
    for op_index in range(len(operations)):
        operations[op_index] = (operations[op_index] + 1) % operation_modulus
        if not operations[op_index] == 0:
            # do not need to carry over 1 from the sum, exit now
            break
    return operations

calibration_total = 0
with open("E:\\dev\\AoC2024\\day07\\input.txt") as file:
    for line in file:
        input_output = line.split(": ")
        test_value = int(input_output[0])
        inputs = list(map(int, input_output[1].split()))
        operations = [0] * (len(inputs) - 1)
        while True:
            if test_permutation(test_value, inputs, operations):
                calibration_total += test_value
                break
            operations = next_permutation(operations, operation_modulus=2)
            if all(o == 0 for o in operations):
                # All permutations have been tested
                break

print(f"Final calibration sum is {calibration_total}")

calibration_total = 0
with open("E:\\dev\\AoC2024\\day07\\input.txt") as file:
    for line in file:
        input_output = line.split(": ")
        test_value = int(input_output[0])
        inputs = list(map(int, input_output[1].split()))
        operations = [0] * (len(inputs) - 1)
        while True:
            if test_permutation(test_value, inputs, operations):
                calibration_total += test_value
                break
            operations = next_permutation(operations, operation_modulus=3)
            if all(o == 0 for o in operations):
                # All permutations have been tested
                break

print(f"Final calibration sum when allowing concatenation is {calibration_total}")