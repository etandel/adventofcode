import sys

def part1(file):
    count = 0
    state = 50

    for l in file:
        direction = l[0]
        val = int(l[1:])

        match direction:
            case "R":
                state += val
            case "L":
                state -= val
            case _:
                raise ValueError(f"invalid direction {direction}")

        state %= 100
        if state == 0:
            count += 1

    print(count)


def part2(file):
    count = 0

    state_before = 50
    state = 50

    for l in file:
        direction = l[0]
        val = int(l[1:])

        fullturns, val = divmod(val, 100)
        count += fullturns

        match direction:
            case "R":
                state += val
            case "L":
                state -= val
            case _:
                raise ValueError(f"invalid direction {direction}")

        state %= 100

        should_add = (
            (state < state_before and direction == "R" and state_before != 0) or
            (state > state_before and direction == "L" and state_before != 0) or
            state == 0
        )
        if should_add:
            count += 1

        state_before = state

    print(count)


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
