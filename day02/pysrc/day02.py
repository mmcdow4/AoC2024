import numpy as np

def test_report(report):
    prev_value = report[0]
    delta_sign = 0
    safe = True
    for idx in range(1,len(report)):
        delta = report[idx] - prev_value
        if delta_sign == 0:
            delta_sign = np.sign(delta)
        if delta_sign != np.sign(delta) or abs(delta) > 3 or delta == 0:
            safe = False
            break
        else:
            prev_value = report[idx]
    return safe

count = 0
safe_idx = list()
dampened_idx = list()
line_count = 1
with open('E:\\dev\\advent_of_code_2024\\day02\\input.txt') as file:
    for line in file:
        report = list(map(int, line.split(" ")))
        safe = test_report(report)

        ignore_idx = 0
        while (not safe) and (ignore_idx < len(report)):
            if ignore_idx == 0:
                temp_report = report[1:]
            elif ignore_idx == len(report)-1:
                temp_report = report[0:len(report)-1]
            else:
                temp_report = report[0:ignore_idx] + report[ignore_idx+1:]
            safe = test_report(temp_report)
            if safe:
                print("report at ", line_count, " made safe by ignoring index ", ignore_idx)
            ignore_idx += 1
        
        # deltas = np.diff(report)
        if safe: #(all(deltas > 0) | all(deltas < 0)) & all(abs(deltas) < 4) :
            count += 1
            safe_idx.append(line_count)
        line_count += 1

print("Total safe count: ", count)
