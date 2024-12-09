import string

with open('E:\\dev\\advent_of_code_2024\\day03\\input.txt') as file:
    A = 0
    B = 0
    total = 0
    expected_function = "mul("
    state = 0
    enabled = True
    err_str = ""
    next_char = file.read(1)
    char_count = 1
    while next_char:
        corruption = True
        
        if state == 0:
            if expected_function[0] == next_char:
                corruption = False
                expected_function = expected_function[1:]
                if len(expected_function) == 0:
                    state = state + 1 % 3
        elif state == 1:
            if next_char.isnumeric():
                corruption = False
                A = A * 10 + ord(next_char) - ord('0')
            elif next_char == ',':
                corruption = False
                state = state + 1 % 3
            else:
                print("Encountered interrupting corruption at character: ", char_count)
        elif state == 2:
            if next_char.isnumeric():
                corruption = False
                B = B * 10 + ord(next_char) - ord('0')
            elif next_char == ')':
                corruption = False
                if A < 1000 and B < 1000 and enabled:
                    print("Adding product ", A, " * ", B, " = ", A * B)
                    total += A * B
                expected_function = "mul("
                state = (state + 1) % 3
                A = 0
                B = 0
            else:
                print("Encountered interrupting corruption at character: ", char_count)
        else:
            print("BAD STATE! ", state)
            break
        
        if corruption:
            A = 0
            B = 0
            state = 0
            expected_function = "mul("
            err_str += next_char
            if "do()" in err_str:
                enabled = True
                err_str = ""
                print("Enabled ON at character: ", char_count)
            elif "don't()" in err_str:
                enabled = False
                err_str = ""
                print("Enabled OFF at character: ", char_count)

        
        next_char = file.read(1)
        char_count += 1

    print('Final total is ', total)

