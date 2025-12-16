import sys
from functools import reduce
from operator import add, mul

OPS = {
    "+": add,
    "*": mul,
}

def apply(op, xs):
    return reduce(OPS[op], xs)


def part1(file):
    rows = []
    ops = []
    for l in file:
        cols = l.strip().split()
        # last line
        if cols[0] in OPS:
            ops = cols
        else:
            rows.append(list(map(int, cols)))
        
    total = 0
    for i in range(len(ops)):
        total += apply(ops[i], (r[i] for r in rows))

    print(total)


def part2(file):
    raise NotImplementedError()


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
