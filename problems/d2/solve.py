import sys

import numpy as np


INSTRUCTIONS = open('input.txt').read().split()


TRANSLATE_RIGHT = np.matrix([
    [1],
    [0],
])

TRANSLATE_DOWN = np.matrix([
    [0],
    [1],
])

TRANSLATE = {
    'R': TRANSLATE_RIGHT,
    'L': -TRANSLATE_RIGHT,
    'D': TRANSLATE_DOWN,
    'U': -TRANSLATE_DOWN,
}


def cartesian2digit(position):
    x, y = position.T.tolist()[0]
    return 1 + x + 3 * y


def part1():
    password = []
    position = np.matrix([[1], [1]])
    for line in INSTRUCTIONS:
        for instruction in line:
            move = TRANSLATE[instruction]
            new_position = position + move
            if not (-1 in new_position or 3 in new_position):
                position = new_position

        password.append(cartesian2digit(position))

    return ''.join(map(str, password))


GRID = np.matrix([
    [0x0, 0x0, 0x1, 0x0, 0x0],
    [0x0, 0x2, 0x3, 0x4, 0x0],
    [0x5, 0x6, 0x7, 0x8, 0x9],
    [0x0, 0xA, 0xB, 0xC, 0x0],
    [0x0, 0x0, 0xD, 0x0, 0x0],
])


def get_value(position):
    if -1 in position or 5 in position:
        return 0
    else:
        return GRID.T[position.tolist()][0, 0]


def part2():
    password = []
    position = np.matrix([[0], [2]])
    for line in INSTRUCTIONS:
        for instruction in line:
            move = TRANSLATE[instruction]
            new_position = position + move
            value = get_value(new_position)
            if value:
                position = new_position

        password.append(get_value(position))

    return ''.join(map(str, password))


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
