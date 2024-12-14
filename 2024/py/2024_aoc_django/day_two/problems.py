import os


def is_constant_acs_or_desc(arr, tolerance=0, used_tolerance=0):
    dir = None
    res = True
    curr = int(arr[0])
    next = int(arr[1])
    if curr < next:
        dir = "asc"
    elif curr > next:
        dir = "desc"
    for i in range(len(arr) - 1):
        curr = int(arr[i])
        next = int(arr[i + 1])
        if dir == "asc":
            res = curr < next

        elif dir == "desc":
            res = curr > next
        if not res:
            break
    return res


def within_step_distance(arr, tolerance=0, used_tolerance=0):
    res = True
    for i in range(len(arr) - 1):
        diff = abs(int(arr[i]) - int(arr[i + 1]))
        res = diff > 0 and diff <= 3
        if not res:
            break
    return res


L_INPUT = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""


def part_one():
    module_dir = os.path.dirname(__file__)  # get current directory
    file_path = os.path.join(module_dir, "./data/day_two")
    file = open(file_path)
    data = file.read()
    lines = data.split("\n")
    # lines = L_INPUT.split("\n")
    # trim
    if lines[len(lines) - 1] == "":
        lines.pop()
    score = 0

    for line in lines:
        data = line.split(" ")
        if is_constant_acs_or_desc(data) and within_step_distance(data):
            score += 1

    return "Part One: Number of safe reports: " + str(score)


def get_file_input(file_name):
    module_dir = os.path.dirname(__file__)  # get current directory
    file_path = os.path.join(module_dir, "./data/" + file_name)
    file = open(file_path)
    return file.read()


def part_two():
    tolerance = 1
    used_tolerance = 0
    module_dir = os.path.dirname(__file__)  # get current directory
    file_path = os.path.join(module_dir, "./data/day_two")
    file = open(file_path)
    data = file.read()
    lines = data.split("\n")
    # lines = L_INPUT.split("\n")
    # trim
    if lines[len(lines) - 1] == "":
        lines.pop()
    score = 0

    for line in lines:
        data = line.split(" ")
        if is_constant_acs_or_desc(
            data, tolerance, used_tolerance
        ) and within_step_distance(data, tolerance, used_tolerance):
            score += 1

    return "Part One: Number of safe reports: " + str(score)


def day_three(pt):
    if pt == "one":
        day_three_pt_one()
    elif pt == "two":
        day_three_pt_two()
    else:
        print("Error: invalid part")


DAY_THREE_L_INPUT = (
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
)


seq = ["m", "u", "l", "(", True, True, ")"]
valid = {"m": True, "u": True, "l": True, "(": True, ")": True}
valid_open = {"m": True, "u": True, "l": True, "(": True}
open_pattern = "mul("
close_pattern = ")"


class MulDeCourr:
    def __init__(self, str_arr):
        self.seq = ["m", "u", "l", "(", "one", ",", "two", ")"]
        self.count = 0
        self.init_nums()
        self.str = str_arr
        self.score = 0

    def init_nums(self):
        self.num = {"one": "", "two": ""}

    def check(self):
        for char in self.str:
            self._check(char)
        print("SCORE: ", self.score)

    def reset(self, char):
        # print("Dropping out of seq...: ", char)
        self.count = 0
        self.init_nums()

    def is_int(self, char):
        try:
            int(char)
        except ValueError:
            return False
        else:
            return True

    def _check(self, char):
        if self.count > len(self.seq) - 1:
            # print("COMPLETED MUL")
            self.calc()
        # print("CHECKING...", char, self.seq[self.count])
        ref = self.seq[self.count]
        if ref == "two" and char == ")":
            # print("COMPLETED MUL")
            self.calc()

        if ref == "one" or ref == "two":
            if ref == "one" and char == ",":
                self.count += 2
                # print("INC to num two")
                # print(char, self.num)
                return True
            # print("calculating nums for position: ", ref)
            is_digit = self.is_int(char)
            if not is_digit:
                # print("drop out of seq: invalid int: ", char)
                self.reset(char)
                return False
            self.num[ref] += char
            return True
        elif ref is not char:
            # print("drop out of seq: ", char)
            self.reset(char)
            return False
        # print("Incrementing position: ", char)
        self.count += 1
        return True

    def calc(self):
        # print("cALC: ", self.num)
        one = int(self.num["one"])
        two = int(self.num["two"])
        self.score += one * two
        print("CALC: ", one, two, self.score)
        self.reset("Completed")


def day_three_pt_one():
    data = get_file_input("day_three")
    decorr = MulDeCourr(data)
    # decorr = MulDeCourr(DAY_THREE_L_INPUT)
    decorr.check()


def day_three_pt_two():
    print("Unfinished")
