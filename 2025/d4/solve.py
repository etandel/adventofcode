import sys


def isroll(c):
    return c == "@"


class Mat:
    def __init__(self, lines):
        self.lines = [list(l) for l in lines]
        self.lenrows = len(self.lines)
        self.lencols = len(self.lines[0])

    def get(self, col, row):
        return self.lines[row][col]

    def remove(self, col, row):
        self.lines[row][col] = "."

    def can_remove(self, col, row):
        return (
            isroll(self.get(col, row))
            and sum(1 for p in self.iter_around(col, row) if isroll(p)) < 4
        )

    def iter_all_pos(self):
        for row in range(0, self.lenrows):
            for col in range(0, self.lencols):
                yield (col, row)

    def iter_around(self, col, row):
        left = col - 1
        top = row - 1
        right = col + 1
        bot = row + 1

        canleft = left >= 0
        cantop = top >= 0
        canright = right < len(self.lines[0])
        canbot = bot < len(self.lines)

        if canleft:
            yield self.get(left, row)

            if cantop:
                yield self.get(left, top)

        if cantop:
            yield self.get(col, top)

            if canright:
                yield self.get(right, top)

        if canright:
            yield self.get(right, row)

            if canbot:
                yield self.get(right, bot)

        if canbot:
            yield self.get(col, bot)

            if canleft:
                yield self.get(left, bot)


def part1(file):
    total = 0

    mat = Mat(l.strip() for l in file)
    for col, row in mat.iter_all_pos():
        if mat.can_remove(col, row):
            total += 1

    print(total)


def part2(file):
    total = 0

    mat = Mat(l.strip() for l in file)
    removed = True
    while removed:
        thisrun = False 
        for col, row in mat.iter_all_pos():
            if mat.can_remove(col, row):
                total += 1
                mat.remove(col, row)
                thisrun = True
        removed = thisrun

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
