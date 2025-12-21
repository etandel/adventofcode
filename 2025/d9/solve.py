import sys


def read(file) -> list[tuple[int, ...]]:
    return [
        tuple(map(int, l.strip().split(",")))
        for l in file
    ]

def minmax(a, b):
    if a > b:
        return b, a
    else:
        return a, b

def area_of(p1, p2):
    x0, x1 = minmax(p1[0], p2[0])
    y0, y1 = minmax(p1[1], p2[1])
    return (y1 - y0 + 1) * (x1 - x0 + 1)


def part1(file):
    rows = read(file)

    max_area = 0
    for i in range(len(rows)):
        for j in range(i+1, len(rows)):
            area = area_of(rows[i], rows[j])
            if area > max_area:
                max_area = area

    print(max_area)


def part2(file):
    pass


def main():
    part = sys.argv[1]
    fname = sys.argv[2]
    args = sys.argv[3:]
    with open(fname) as f:
        match part:
            case "1":
                part1(f, *args)
            case "2":
                part2(f, *args)
            case _:
                raise ValueError(f"invalid part number: {part}")


if __name__ == "__main__":
    main()
