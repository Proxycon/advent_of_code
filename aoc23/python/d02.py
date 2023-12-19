import re

limits = {
    "red" : 12,
    "green": 13,
    "blue": 14
}


def d02p1():
    filename = "inputs/input02.txt"

    sum = 0

    with open(filename) as file:
        while line := file.readline():
            sum += game_score(line)
    print(sum)


def game_score(line):
    gameid = re.findall(r'(Game )(\d+)', line)[0][1]
    print(gameid)
    for color, limit in limits.items():
        pattern = fr'(\d+)(\s{color})'
        matches = [int(x[0]) for x in re.findall(pattern, line)]
        if any(x> limit for x in matches):
            print(f"❌ {line.strip()} invalid because {matches} over {limit}") 
            return 0
    print(f"✅ +{gameid}: {line.strip()} valid with matches: {matches}")
    return int(gameid)

def min_cubes(line):
    gameid = re.findall(r'(Game )(\d+)', line)[0][1]
    print(gameid)
    power = 1
    for color, limit in limits.items():
        pattern = fr'(\d+)(\s{color})'
        matches = [int(x[0]) for x in re.findall(pattern, line)]
        power *= max(matches)
    print(f"✅ +{power}: {line.strip()} valid with matches: {matches}")
    return int(power)

def d02p2():
    filename = "inputs/input02.txt"

    sum = 0

    with open(filename) as file:
        while line := file.readline():
            sum += min_cubes(line)
    print(sum)
                    





if __name__ == "__main__":
    d02p2()

