def d01p1():
    filename = "aoc23/inputs/input01.txt"

    with open(filename) as file:
        while line := file.readline():
            process(line)

def process(line):
    pass


if __name__ == "__main__":
    d01p1()