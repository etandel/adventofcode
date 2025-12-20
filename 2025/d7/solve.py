import sys
from collections import defaultdict, Counter


def read(file):
    start = (0, 0)
    splitters = defaultdict(set)
    nrows = 0
    ncols = 0
    for y, row in enumerate(file):
        nrows = y
        for x, c in enumerate(row):
            ncols = x
            match c:
                case 'S':
                    start = (y, x)
                case '^':
                    splitters[y].add(x)

    return nrows, ncols, start, splitters


def part1(file):
    nrows, _, start, splitters = read(file)

    nsplits = 0
    previous = {start[1]}
    for y in range(start[0] + 2, nrows + 1):
        new = set()
        for x in previous:
            if x in splitters[y]:
                nsplits += 1
                new.add(x - 1)
                new.add(x + 1)
            else:
                new.add(x)

        previous = new
    
    print(nsplits)


def part2(file):
    nrows, _, start, splitters = read(file)

    previous = Counter((start[1],))
    for y in range(start[0] + 2, nrows + 1):
        new = Counter[int]()
        for x in previous:
            if x in splitters[y]:
                new[x - 1] += previous[x]
                new[x + 1] += previous[x]
            else:
                new[x] += previous[x]

        previous = new

    print(sum(previous.values()))


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
