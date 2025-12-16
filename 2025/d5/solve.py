import sys
from itertools import islice


class Range:
    def __init__(self, first, last):
        self.first = first
        self.last = last

    def length(self):
        return self.last - self.first + 1

    def __contains__(self, x):
        return self.first <= x <= self.last

    def __eq__(self, other):
        return self.first == other.first and self.last == other.last

    def __lt__(self, other):
        return self.first < other.first

    def __repr__(self):
        return f"Range({self.first}, {self.last})"

    def __str__(self):
        return f"{self.first}-{self.last}"


def minmax(a, b):
    if a < b:
        return a, b
    else:
        return b, a


def merge_ranges(r1, r2):
    """
    assumes r1.first <= r2.first
    """
    if r2.first <= r1.last:
        return [Range(r1.first, max(r1.last, r2.last))]
    else:
        return [r1, r2]


def parse_input(file):
    ranges = []
    for line in file:
        line = line.strip()
        if line == "":
            break
        
        first, last = line.split("-")
        ranges.append(Range(int(first), int(last)))

    ids = []
    for line in file:
        ids.append(int(line.strip()))

    return ranges, ids


def part1(file):
    ranges, ids = parse_input(file)
    print(sum(1 for id_ in ids if any(id_ in r for r in ranges)))


def part2(file):
    ranges, _ = parse_input(file)
    ranges.sort()

    merged = [ranges[0]]
    for r in islice(ranges, 1, None):
        r0 = merged[-1]
        merged[-1:] = merge_ranges(r0, r)

    print(sum(r.length() for r in merged))


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
