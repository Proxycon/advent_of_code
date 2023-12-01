

def d01p1():
    filename = "aoc23/inputs/input01.txt"
    sum = 0

    with open(filename) as file:
        while line := file.readline():
            sum += filter_first_and_last_number(line)
    print(sum)

def filter_first_and_last_number(line):
    numbers = list(filter(str.isdigit, line))
    return int(numbers[0]+ numbers[-1])

if __name__ == "__main__":
    d01p1()