
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
    "nine": "9",
    "ten": "10",
    "eleven": "11",
    "twelve": "12",
    "thirteen": "13",
    "fourteen": "14",
    "fifteen": "15",
    "sixteen": "16",
    "seventeen": "17",
    "eighteen": "18",
    "nineteen": "19",
    "twenty": "20",
    "thirty": "30",
    "forty": "40",
    "fifty": "50",
}

def d01p1():
    filename = "aoc23/inputs/input01.txt"
    sum = 0

    with open(filename) as file:
        while line := file.readline():
            sum += filter_first_and_last_number(line)
    print(sum)

def filter_first_and_last_number(line):
    digits = list(filter(str.isdigit, line))
    return int(digits[0]+ digits[-1]) if len(digits) > 0 else 0

def replace_filter_first_and_last_number(line):

    
    for key, value in reversed(numbers.items()):
        line = line.replace(key, value)
    digits = list(filter(str.isdigit, line))
    print("parsed {} to {} and took {}".format(line, digits, digits[0]+ digits[-1]))
    return int(digits[0]+ digits[-1])

def d01p2():
    filename = "aoc23/inputs/input01.txt"
    sum = 0

    with open(filename) as file:
        while line := file.readline():
            sum += replace_filter_first_and_last_number(line)
    print(sum)



if __name__ == "__main__":
    d01p1()
    d01p2()