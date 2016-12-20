import sys
import unittest
from compression import Compression


INPUT = open('input.txt')


def go(deep):
    return Compression(deep).count(INPUT.read().replace('\n', ''))


def part1():
    return go(False)


def part2():
    return go(True)


if __name__ == '__main__':
    if '--test' in sys.argv:
        sys.argv.pop(sys.argv.index('--test'))
        unittest.main()
    else:
        print((part1 if sys.argv[1] == '1' else part2)())
