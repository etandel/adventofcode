import sys
import unittest
from compression_v1 import count as count_v1, parse as parse_v1
from compression_v2 import count as count_v2, parse as parse_v2


INPUT = open('input.txt')


def part1():
    return count_v1(parse_v1(INPUT.read().replace('\n', '')))


def part2():
    return count_v2(parse_v2(INPUT.read().replace('\n', '')))


if __name__ == '__main__':
    if '--test' in sys.argv:
        sys.argv.pop(sys.argv.index('--test'))
        unittest.main()
    else:
        print((part1 if sys.argv[1] == '1' else part2)())
