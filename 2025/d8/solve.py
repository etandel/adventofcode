import sys
from operator  import itemgetter


def read(file) -> list[tuple[int, ...]]:
    return [
        tuple(map(int, l.strip().split(",")))
        for l in file
    ]


def distance2(p1, p2):
    dx = p2[0] - p1[0]
    dy = p2[1] - p1[1]
    dz = p2[2] - p1[2]
    return dx * dx + dy * dy + dz * dz


def part1(file, n_connections):
    n_connections = int(n_connections)
    rows = read(file)

    pairs_with_dist = []
    for i in range(len(rows)):
        for j in range(i+1, len(rows)):
            p1 = rows[i]
            p2 = rows[j]
            d2 = distance2(p1, p2)
            pairs_with_dist.append((d2, p1, p2))

    pairs_with_dist.sort(key=itemgetter(0))

    circuits: list[set[tuple[int, ...]]] = []
    n = 0
    for _, p1, p2 in pairs_with_dist:
        if n >= n_connections:
            break

        withp1_idx = -1
        withp2_idx = -1
        for i, circuit in enumerate(circuits):
            if p1 in circuit:
                withp1_idx = i

            if p2 in circuit:
                withp2_idx = i

        if withp1_idx == withp2_idx and withp1_idx == -1:
            circuits.append({p1, p2})
        elif withp1_idx == withp2_idx:
            pass
        elif withp1_idx == -1:
            circuits[withp2_idx].add(p1)
        elif withp2_idx == -1:
            circuits[withp1_idx].add(p2)
        else:
            withp1 = circuits[withp1_idx]
            circuits[withp2_idx].update(withp1)
            del circuits[withp1_idx]

        n += 1

    circuits.sort(key=len, reverse=True)

    prod = 1
    for c in circuits[:3]:
        prod *= len(c)

    print(prod)


def part2(file):
    rows = read(file)

    pairs_with_dist = []
    for i in range(len(rows)):
        for j in range(i+1, len(rows)):
            p1 = rows[i]
            p2 = rows[j]
            d2 = distance2(p1, p2)
            pairs_with_dist.append((d2, p1, p2))

    pairs_with_dist.sort(key=itemgetter(0))

    notseen = set(rows)
    circuits: list[set[tuple[int, ...]]] = []

    latest = ((0, 0, 0), (0, 0, 0))
    for _, p1, p2 in pairs_with_dist:
        if not notseen:
            break

        withp1_idx = -1
        withp2_idx = -1
        for i, circuit in enumerate(circuits):
            if p1 in circuit:
                withp1_idx = i

            if p2 in circuit:
                withp2_idx = i

        added = True
        if withp1_idx == withp2_idx and withp1_idx == -1:
            circuits.append({p1, p2})
        elif withp1_idx == withp2_idx:
            added = False
        elif withp1_idx == -1:
            circuits[withp2_idx].add(p1)
        elif withp2_idx == -1:
            circuits[withp1_idx].add(p2)
        else:
            withp1 = circuits[withp1_idx]
            circuits[withp2_idx].update(withp1)
            del circuits[withp1_idx]

        if p1 in notseen:
            notseen.remove(p1)
        if p2 in notseen:
            notseen.remove(p2)

        if added:
            latest = (p1, p2)


    p1, p2 = latest
    print(p1[0] * p2[0])


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
