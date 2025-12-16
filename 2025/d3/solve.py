import sys
from itertools import islice


def process_line(line: str, totaldigits: int) -> int:
    remaining = totaldigits
    digits = []

    first = 0
    last = len(line)
    while remaining > 0:
        best_pos = 0
        best = '0'
        for i in range(first, last):
            c = line[i]
            if c > best:
                best = c
                best_pos = i


        digits.append((best_pos, best))
        remaining -= 1

        available_right = (last - best_pos - 1)
        if available_right <= remaining:
            remaining -= available_right
            digits.extend((i, line[i]) for i in range(best_pos + 1, last))

            first = first
            last = best_pos
        else:
            first = best_pos + 1
            last = last


    digits.sort(key=lambda t: t[0])
    return int(''.join(t[1] for t in digits))


def part1(file):
    print(sum(process_line(l.strip(), 2) for l in file))


def part2(file):
    print(sum(process_line(l.strip(), 12) for l in file))


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
