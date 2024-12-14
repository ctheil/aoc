L_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3"""


def part_one():
    file = open("./data/day_one", "r")
    data = file.read()
    lines = data.split("\n")
    if lines[len(lines) - 1] == "":
        lines.pop()
    # lines = L_INPUT.split("\n")

    l_list = []
    r_list = []
    for line in lines:
        data = line.split("   ")
        l_list.append(int(data[0]))
        r_list.append(int(data[1]))

    l_list.sort()
    r_list.sort()

    total_dist = 0
    print(len(l_list), len(r_list))

    for i in range(len(l_list)):
        dist = abs(r_list[i] - l_list[i])
        total_dist += dist
    print("Total Distance: ", total_dist)


# part_one()


def part_two():
    file = open("./data/day_one", "r")
    data = file.read()
    lines = data.split("\n")
    if lines[len(lines) - 1] == "":
        lines.pop()
    # lines = L_INPUT.split("\n")

    r_data = {}
    # l_list = []
    l_list = []
    for line in lines:
        data = line.split("   ")
        if r_data.get(int(data[1])):
            r_data[int(data[1])] += 1
        else:
            r_data[int(data[1])] = 1
        # l_list.append(int(data[0]))
        l_list.append(int(data[0]))

    total_dist = 0
    for p in l_list:
        dist = p * (r_data.get(p) or 0)
        total_dist += dist

    print(total_dist)


part_two()
