import sys

from collections import Counter
from functools import wraps
from hashlib import md5
from itertools import count


SALT = 'cuanljph'


def memoize(f):
    mem = {}

    @wraps(f)
    def newf(*args):
        try:
            return mem[args]
        except KeyError:
            r = f(*args)
            mem[args] = r
            return r

    return newf


@memoize
def gen_key(i):
    return md5(f'{SALT}{i}'.encode('utf-8')).hexdigest()


@memoize
def gen_stretched_key(i):
    k = f'{SALT}{i}'.encode('utf-8')
    for _ in range(2017):
        k = md5(k).hexdigest().encode('utf-8')
    return k


def get_first_triplet(s: str) -> str:
    for a, b, c in zip(s, s[1:], s[2:]):
        if a == b == c:
            return a
    return ''


def contains_quintuplet_of(string, char):
    for a, b, c, d, e in zip(string, string[1:], string[2:], string[3:], string[4:]):
        if char == a == b == c == d == e:
            return True
    return False


class Solver:
    def __init__(self, hasher):
        self.hasher = hasher

    def next_1000_contains_5_of(self, char, from_index):
        for i in range(from_index + 1, from_index + 1000 + 1):
            if contains_quintuplet_of(self.hasher(i), char):
                return True
        return False

    def is_key(self, i: int) -> bool:
        key = self.hasher(i)
        counts = Counter(key)
        c = get_first_triplet(key)
        return bool(c) and self.next_1000_contains_5_of(c, i)

    def solve(self):
        key_index = 0
        for i in count():
            if self.is_key(i):
                key_index += 1
                if key_index == 64:
                    return i


def part1():
    return Solver(gen_key).solve()

def part2():
    return Solver(gen_stretched_key).solve()


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())

