import re
import sys
from itertools import chain


INPUT = open('input.txt')

HYPERNET = re.compile(r'\[(\w+)\]')
SUPERNET = re.compile(r'(\w+)(?:\[|$)')


def is_abba_4(s):
    return s[0] != s[1] and s[:2] == s[4:1:-1]


def is_abba_partial(s):
    for i in range(len(s) - 3):
        partial = s[i:i+4]
        if is_abba_4(partial):
            return True
    return False


def tls_matches(line):
    return (not any(map(is_abba_partial, HYPERNET.findall(line))) and
            is_abba_partial(line))


def is_aba_3(s):
    return s[0] == s[2] and s[0] != s[1]


def get_abas(s):
    abas = []
    for i in range(len(s) - 2):
        partial = s[i:i+3]
        if is_aba_3(partial):
            abas.append(partial)
    return abas


def has_bab(aba, s):
    for candidate in get_abas(s):
        if candidate[0] == aba[1] and candidate[1] == aba[0]:
            return True
    return False


def ssl_matches(line):
    supers = SUPERNET.findall(line)
    abas = chain.from_iterable(map(get_abas, supers))
    for aba in abas:
        for hyper in HYPERNET.findall(line):
            if has_bab(aba, hyper):
                return True
    return False


def part1():
    return len(list(filter(tls_matches, INPUT.readlines())))


def part2():
    return len(list(filter(ssl_matches, INPUT.readlines())))


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
