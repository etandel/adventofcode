import sys
import string
from functools import reduce
from operator import add, mul
from typing import Callable

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


class Problem:
    def __init__(self):
        self.vals: list[int] = []
        self.op = ""

    def add_val(self, v: int):
        self.vals.append(v)

    def set_op(self, op: str):
        self.op = op

    def eval(self) -> int:
        return apply(self.op, self.vals)

    def __str__(self) -> str:
        return f" {self.op} ".join(map(str, self.vals))


def part2(file):
    data = [list(l.strip("\n")) for l in file]
    ncols = len(data[0])

    problems = []
    problem = Problem()
    for i in range(ncols-1, -1, -1):
        digits = []
        all_spaces = True
        for rowi, row in enumerate(data):
            c = row[i]
            if c in OPS:
                all_spaces = False
                problem.set_op(c)
            elif c in string.digits:
                all_spaces = False
                digits.append(c)
            elif c == " ":
                pass
            else:
                raise ValueError(f"unknown char {c} at ({rowi}, {i})")

        if digits:
            problem.add_val(int("".join(digits)))

        if all_spaces:
            problems.append(problem)
            problem = Problem()

    problems.append(problem)

    print(sum(map(Problem.eval, problems))) 


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
