import sys

import numpy as np


INSTRUCTIONS = open('input.txt').read().replace(',', '').split()


ROTATE_LEFT = np.matrix([
    [0, -1],
    [1,  0],
])

ROTATE_RIGHT = - ROTATE_LEFT

ROTATION = {
    'L': ROTATE_LEFT,
    'R': ROTATE_RIGHT,
}


def read_instruction(direction, instruction):
    direction = ROTATION[instruction[0]] * direction
    return direction, int(instruction[1:]) * direction


def part1():
    position = np.matrix([[0], [0]])
    direction = np.array([[1], [0]])
    for instruction in INSTRUCTIONS:
        direction, vec = read_instruction(direction, instruction)
        position += vec

    return sum(map(abs, position))


def read_instruction_with_partials(direction, instruction):
    direction = ROTATION[instruction[0]] * direction
    length = int(instruction[1:])
    to_visit = [i * direction for i in range(1, length+1)]
    return direction, to_visit


def _to_hashable(position):
    return tuple(position.T.tolist()[0])


def part2():
    position = np.matrix([[0], [0]])
    direction = np.array([[0], [1]])
    all_visited = [_to_hashable(position)]
    for count, instruction in enumerate(INSTRUCTIONS):
        direction, to_visit = read_instruction_with_partials(direction, instruction)

        for vec in to_visit:
            partial = position + vec
            hashable = _to_hashable(partial)
            if hashable in all_visited:
                return partial
            else:
                all_visited.append(hashable)
        else:
            position = partial


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
