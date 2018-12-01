import sys
from collections import defaultdict


INPUT = open('input.txt')

WIDTH = 8


def go(reverse):
    countss = [defaultdict(int) for _ in range(WIDTH)]
    for line in INPUT.readlines():
        for i, c in enumerate(line[:-1]):  # remove \n
            countss[i][c] += 1

    return ''.join(sorted(counts.items(),
                          key=lambda t: t[1],
                          reverse=reverse)[0][0]
                   for counts in countss)


def part1():
    return go(reverse=True)


def part2():
    return go(reverse=False)


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
