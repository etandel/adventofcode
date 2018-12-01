import re
import sys
import unittest
from enum import Enum


INPUT = open('input.txt')

WIDTH = 50
HEIGHT = 6


class Command(Enum):
    RECT = 'rect'
    ROTATE_ROW = 'rotate row'
    ROTATE_COL = 'rotate col'


class Grid:
    def __init__(self):
        self.grid = [False] * (50 * HEIGHT)
        self._cmd_mapping = {
            Command.RECT: self.rect,
            Command.ROTATE_ROW: self.rotate_row,
            Command.ROTATE_COL: self.rotate_col,
        }

    def __getitem__(self, key):
        if isinstance(key, tuple):
            i, j = key
            return self.grid[i*WIDTH + j]
        else:
            return self.grid[key]

    def __setitem__(self, key, value):
        row, col = key
        self.grid[row*WIDTH + col] = value

    def __len__(self):
        return len(self.grid)

    def __str__(self):
        matrix = [['#' if self[i, j] else '.' for j in range(WIDTH)]
                  for i in range(HEIGHT)]
        return '\n'.join(map(''.join, matrix))

    def keys(self, height=HEIGHT, width=WIDTH):
        for i in range(height):
            for j in range(width):
                yield i, j

    def execute_command(self, command):
        cmd, val1, val2 = command
        return self._cmd_mapping[cmd](val1, val2)

    def rect(self, a, b):
        for i, j in self.keys(b, a):
            self[i, j] = True

    def rotate_row(self, row_id, count):
        new_row = [self[row_id, (j - count) % WIDTH] for j in range(WIDTH)]
        for j in range(WIDTH):
            self[row_id, j] = new_row[j]

    def rotate_col(self, col_id, count):
        new_col = [self[(i - count) % HEIGHT, col_id] for i in range(HEIGHT)]
        for i in range(HEIGHT):
            self[i, col_id] = new_col[i]


RECT_RE = re.compile(r'^(rect) (\d+)x(\d+)$')
ROTATE_ROW_RE = re.compile(r'^(rotate row) y=(\d+) by (\d+)$')
ROTATE_COL_RE = re.compile(r'^(rotate col)umn x=(\d+) by (\d+)$')


def parse_command(line):
    if line.startswith('rect'):
        regex = RECT_RE
    elif line.startswith('rotate row'):
        regex = ROTATE_ROW_RE
    elif line.startswith('rotate col'):
        regex = ROTATE_COL_RE
    else:
        raise RuntimeError('Invalid command: {}'.format(line))
    cmd, val1, val2 = regex.findall(line)[0]
    return Command(cmd), int(val1), int(val2)


def get_grid():
    g = Grid()
    for line in INPUT.readlines():
        line = line.replace('\n', '')
        if line:
            g.execute_command(parse_command(line))
    return g


def part1():
    g = get_grid()
    return len(list(filter(None, g)))


def part2():
    return str(get_grid())


class TestGrid(unittest.TestCase):
    def assert_grid(self, grid, true_keys=None):
        true_keys = true_keys or []
        for key_pair in grid.keys():
            assertion = (self.assertTrue
                         if key_pair in true_keys
                         else self.assertFalse)
            assertion(grid[key_pair], msg='{}'.format(key_pair))

    def test_rect(self):
        g = Grid()
        g.rect(0, 5)
        self.assert_grid(g)

        g = Grid()
        g.rect(1, 5)
        self.assert_grid(g, {(i, 0) for i in range(5)})

        g = Grid()
        g.rect(2, 3)
        self.assert_grid(g, {(i, j) for j in range(2) for i in range(3)})

    def test__rotate_row__shift_lt_WIDTH(self):
        g = Grid()
        g[1, 0] = True
        g[1, 25] = True
        g[1, WIDTH-1] = True
        g.rotate_row(1, 15)
        self.assert_grid(g, {(1, 15), (1, 40), (1, 14)})

    def test__rotate_row__shift_gt_WIDTH(self):
        g = Grid()
        g[1, 0] = True
        g[1, 25] = True
        g[1, WIDTH-1] = True
        g.rotate_row(1, 65)
        self.assert_grid(g, {(1, 15), (1, 40), (1, 14)})

    def test__rotate_col__shift_lt_HEIGHT(self):
        g = Grid()
        g[0, 1] = True
        g[2, 1] = True
        g[HEIGHT-1, 1] = True
        g.rotate_col(1, 2)
        self.assert_grid(g, {(2, 1), (4, 1), (1, 1)})

    def test__rotate_col__shift_gt_HEIGHT(self):
        g = Grid()
        g[0, 1] = True
        g[2, 1] = True
        g[HEIGHT-1, 1] = True
        g.rotate_col(1, 8)
        self.assert_grid(g, {(2, 1), (4, 1), (1, 1)})


class TestParse(unittest.TestCase):
    def test_parse(self):
        self.assertEqual(parse_command('rect 51x48'),
                         (Command.RECT, 51, 48))
        self.assertEqual(parse_command('rotate row y=27 by 100'),
                         (Command.ROTATE_ROW, 27, 100))
        self.assertEqual(parse_command('rotate column x=23 by 200'),
                         (Command.ROTATE_COL, 23, 200))


if __name__ == '__main__':
    if '--test' in sys.argv:
        sys.argv.pop(sys.argv.index('--test'))
        unittest.main()
    else:
        print((part1 if sys.argv[1] == '1' else part2)())
