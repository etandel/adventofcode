import re
import sys
from collections import Counter
from functools import cmp_to_key
from itertools import starmap
from string import ascii_lowercase

INPUT = open('input.txt')


RE = re.compile(r'^([a-z\-]+)\-(\d+)\[(\w+)\]$')


def pair_cmp(pair1, pair2):
    # counts should be descending
    count_diff = pair2[1] - pair1[1]
    # but locale should be ascending
    return count_diff if count_diff else ord(pair1[0]) - ord(pair2[0])

pair_cmp = cmp_to_key(pair_cmp)


def calc_checksum(name):
    counts = Counter(filter(str.isalpha, name))
    first5 = sorted(counts.items(), key=pair_cmp)[:5]
    return ''.join(t[0] for t in first5)


def room_exists(entry):
    name, _, checksum = entry
    return calc_checksum(name) == checksum


def get_correct_entries():
    return filter(room_exists, (RE.findall(line)[0] for line in INPUT))


def part1():
    return sum(int(room_id) for _, room_id, _ in get_correct_entries())


ALPHABET_LEN = len(ascii_lowercase)


def decrypt_char(c, shift):
    return (' '
            if c == '-'
            else chr(ord('a') + (ord(c) - ord('a') + shift) % ALPHABET_LEN))


def decrypt(name, room_id):
    return ''.join(decrypt_char(c, room_id) for c in name)


def part2():
    for name, room_id, _ in get_correct_entries():
        if decrypt(name, int(room_id)).startswith('northpole'):
            return room_id


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
