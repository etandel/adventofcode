import sys

import numpy as np


INSTRUCTIONS = open('input.txt').read().split()


TRANSLATE_RIGHT = np.array([1, 0])
TRANSLATE_DOWN = np.array([0, 1])

TRANSLATE = {
    'R': TRANSLATE_RIGHT,
    'L': -TRANSLATE_RIGHT,
    'D': TRANSLATE_DOWN,
    'U': -TRANSLATE_DOWN,
}


def cartesian2digit(position):
    x, y = position.T.tolist()[0]
    return 1 + x + 3 * y


def get_value(grid, position):
    edge = len(grid[0])
    if -1 in position or edge in position:
        return 0
    else:
        col, line = position
        return grid[line][col]


def go(grid, position):
    password = []
    for line in INSTRUCTIONS:
        for instruction in line:
            move = TRANSLATE[instruction]
            new_position = position + move
            value = get_value(grid, new_position)
            if value:
                position = new_position

        password.append(get_value(grid, position))

    return ''.join(map('{:x}'.format, password)).upper()


def part1():
    grid = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ]
    return go(grid, np.array([1, 1]))


def part2():
    grid = [
        [0x0, 0x0, 0x1, 0x0, 0x0],
        [0x0, 0x2, 0x3, 0x4, 0x0],
        [0x5, 0x6, 0x7, 0x8, 0x9],
        [0x0, 0xA, 0xB, 0xC, 0x0],
        [0x0, 0x0, 0xD, 0x0, 0x0],
    ]

    return go(grid, np.array([0, 2]))


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
