import math
import sys
from itertools import batched


def part1(file):
    total = 0
    for interval in file.read().split(","):
        first, last = tuple(map(int, interval.split("-")))
        total += process_interval(first, last) 

    print(total)


def process_interval(a, b):
    first = find_first(a)
    last = find_last(b)

    if last < first:
        return 0

    if get_scale(first) != get_scale(last):
        raise ValueError(
            f"different scales between {a}-{b}, "
            f"{first}-{last}: {get_scale(first)} != {get_scale(last)}"
        )

    return sum_in_interval_same_scale(first, last)


def find_first(x):
    scale = get_scale(x)
    if scale % 2 == 0:
        return 10 ** (scale // 2)

    top_half, bot_half = split_halfs(x)
    if top_half < bot_half:
        return top_half + 1
    else:  # if top_half > or if they are the same
        return top_half


def find_last(x):
    scale = get_scale(x)
    if scale % 2 == 0:
        return 10**(scale//2) - 1

    top_half, bot_half = split_halfs(x)

    if top_half > bot_half:
        return top_half - 1
    else:  # if top_half < or if they are the same
        return top_half


def split_halfs(x):
    scale = get_scale(x)
    top_half = x // (10**((scale+1) / 2))
    bot_half = x - (top_half * 10**((scale+1)/2))
    return top_half, bot_half


def sum_in_interval_same_scale(first, last):
    sum_half = (first + last) * (last - first + 1) / 2
    return sum_half + sum_half * (10**(get_scale(first)+1))


def get_scale(x):
    if x == 0:
        return 0
    return math.floor(math.log10(x))


def part2(file):
    total = 0
    for interval in file.read().split(","):
        first, last = tuple(map(int, interval.split("-")))
        for i in range(first, last+1):
            stri = str(i)
            for l in range(1, len(stri) // 2 + 1):
                cand = set(batched(stri, l))
                if len(cand) == 1:
                    total += i
                    break


    print(total)


def main():
    part, fname = sys.argv[1:]
    with open(fname) as f:
        match part:
            case "1":
                part1(f)
            case "2":
                part2(f)
            case _:
                raise ValueError(f"invalid part number: {part}")

if __name__ == "__main__":
    main()
