from curses.ascii import isdigit
import re

numbers = {
    "zero": "0",
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9"
}

def d01p1():
    filename = "inputs/input01.txt"
    sum = 0

    with open(filename) as file:
        while line := file.readline():
            sum += filter_first_and_last_number(line)
    print(sum)

def filter_first_and_last_number(line):
    digits = list(filter(str.isdigit, line))
    return int(digits[0]+ digits[-1]) if len(digits) > 0 else 0

def replace_filter_first_and_last_number(line: str):
    digits = []
    parsed = line
    while len(parsed) > 0:
        if parsed[0].isdigit():
            digits.append(parsed[0])
            parsed = parsed[1:]
            continue
        for key, value in numbers.items():
            res = re.findall(fr'(^{key})', parsed)
            if len(res) > 0:
                digits.append(value)
        parsed = parsed[1:]

    print("parsed {} to {} and took {}".format(line, digits, digits[0]+ digits[-1]))
    return int(digits[0]+ digits[-1])

def d01p2():
    filename = "inputs/input01.txt"
    sum = 0

    with open(filename) as file:
        while line := file.readline():
            sum += replace_filter_first_and_last_number(line)
    print(sum)



if __name__ == "__main__":
    # d01p1()
    d01p2()