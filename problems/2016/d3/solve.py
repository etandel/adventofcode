import sys
from itertools import permutations

import numpy as np


INPUT = open('input.txt')


def is_triangle(nums):
    return all(p[0] + p[1] > p[2] for p in permutations(nums))


def part1():
    count = 0
    for line in INPUT.readlines():
        nums = list(map(int, line.split()))
        if is_triangle(nums):
            count += 1
    return count


def part2():
    lines = INPUT.readlines()
    count = 0
    while lines:
        raw, lines = lines[:3], lines[3:]

        matrix = np.matrix([list(map(int, l.split()))
                            for l in raw]).T.tolist()

        for nums in matrix:
            if is_triangle(nums):
                count += 1

    return count


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
