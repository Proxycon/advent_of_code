from curses.ascii import isdigit
from unittest import result


def d01p1():
    filename = "inputs/sample03.txt"
    results = []

    with open(filename) as file:
        pre = file.readline()
        line = file.readline()
        post = file.readline()

        results.extend(scan_lines("", pre, line))
        results.extend(scan_lines(pre, line, post))
        while newline := file.readline():
            pre = line
            line = post
            post = newline
            results.extend(scan_lines(pre, line, post))

        pre = line
        line = post
        post = ""
        results.extend(scan_lines(pre, line, post))
        print(results)
        print(sum(results))

def scan_lines(pre, line, post):
    current_digit = ""
    symbol_detected = False
    results = []

    # loop over line
    for index, c in enumerate(line):
        # if digit found
        if c.isdecimal():
            # add digit to cache
            current_digit += c
            # check for adjecent symbold
            
        else:
            # if number ended, add to result and clear cache
            if current_digit != "" and check_surroundings_for_symbol(index-len(current_digit), index, pre, line, post):
                results.append(int(current_digit))
            current_digit = ""
            symbol_detected = False
    # end of line, same as number ended
    if current_digit != "" and check_surroundings_for_symbol(len(line)-len(current_digit), len(line), pre, line, post):
                results.append(int(current_digit))
    return results


def check_surroundings_for_symbol(start, end, pre, line, post):
    start = min(0, start -1)
    end = max(len(line), end+1)
    return any(is_symbol(x) for x in pre[start:end] + line[start:end] + post[start:end])
    

def is_symbol(c):
    return (not c.isdigit()) and c != "." and c != " "
        



if __name__ == "__main__":
    d01p1()