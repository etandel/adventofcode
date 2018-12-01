import sys
from hashlib import md5


INPUT = b'wtnhxymk'

ZEROES = '0' * 5


def part1():
    password = []
    c = 0
    while True:
        candidate = md5(INPUT + str(c).encode('ascii')).hexdigest()
        if candidate[:5] == ZEROES:
            print(candidate)
            password.append(candidate[5])

            if len(password) == 8:
                break

        c += 1
        if c % 100000 == 0:
            print(c)
    return ''.join(password)


def part2():
    password = [None] * 8
    c = 0
    filled = 0
    while True:
        candidate = md5(INPUT + str(c).encode('ascii')).hexdigest()
        if candidate[:5] == ZEROES:
            val = int(candidate[5], 16)
            if val < 8 and password[val] is None:
                password[val] = candidate[6]
                print(password)
                filled += 1

            if filled == 8:
                break

        c += 1
        if c % 1000000 == 0:
            print(c)
    return ''.join(password)


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
