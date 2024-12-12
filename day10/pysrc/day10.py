
def seek_peaks(topographic_map, current_point):
    peaks = set()
    rating = 0
    current_height = topographic_map[current_point[1]][current_point[0]]
    if current_height == 9:
        peaks.add(current_point)
        rating = 1
        return (rating, peaks)
    if current_point[0] > 0 and topographic_map[current_point[1]][current_point[0]-1] == current_height+1:
        # Try going west
        (new_rating, new_peaks) = seek_peaks(topographic_map, (current_point[0]-1, current_point[1]))
        rating += new_rating
        peaks.update(new_peaks)
    if current_point[0] < len(topographic_map[current_point[1]])-1 and topographic_map[current_point[1]][current_point[0]+1] == current_height+1:
        # Try going east
        (new_rating, new_peaks) = seek_peaks(topographic_map, (current_point[0]+1, current_point[1]))
        rating += new_rating
        peaks.update(new_peaks)
    if current_point[1] > 0 and topographic_map[current_point[1]-1][current_point[0]] == current_height+1:
        # Try going north
        (new_rating, new_peaks) = seek_peaks(topographic_map, (current_point[0], current_point[1]-1))
        rating += new_rating
        peaks.update(new_peaks)
    if current_point[1] < len(topographic_map)-1 and topographic_map[current_point[1]+1][current_point[0]] == current_height+1:
        # Try going south
        (new_rating, new_peaks) = seek_peaks(topographic_map, (current_point[0], current_point[1]+1))
        rating += new_rating
        peaks.update(new_peaks)
    return (rating, peaks)


topographic_map = []
with open("E:\\dev\\AoC2024\\day10\\input.txt") as file:
    y = 0
    for line in file:
        topographic_map.append([])
        for char in line:
            if char == '\n' or char == '\r':
                break
            value = ord(char) - ord('0')
            topographic_map[y].append(value)
        y += 1

rating_sum = 0
score_sum = 0
for y in range(len(topographic_map)):
    for x in range(len(topographic_map[y])):
        if topographic_map[y][x] == 0:
            (rating, peaks) = seek_peaks(topographic_map, (x, y))
            rating_sum += rating
            score_sum += len(peaks)

print(f"Sum of trailhead scores is {score_sum}, sum of ratings is {rating_sum}")