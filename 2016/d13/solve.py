import sys
from collections import defaultdict
from dataclasses import dataclass, field
from functools import wraps
from queue import Queue, PriorityQueue
from typing import Any


FAVORITE_NUMBER = 1352


def memoize(f):
    mem = {}

    @wraps(f)
    def newf(*args):
        try:
            return mem[args]
        except KeyError:
            r = f(*args)
            mem[args] = r
            return r

    return newf


@memoize
def count_ones(v):
    return sum(1 for d in bin(v) if d == '1')


@memoize
def is_empty(x, y):
    v = x*x + 3*x + 2*x*y + y + y*y + FAVORITE_NUMBER
    return count_ones(v) % 2 == 0


@memoize
def get_edges(x, y):
    return [p
            for p in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1))
            if p >= (0, 0) and is_empty(*p) ]


@dataclass(order=True)
class PrioritizedItem:
    priority: int
    item: Any=field(compare=False)


def dijkstra(start, goal):
    best_score = defaultdict(lambda: float('inf'), {start: 0})
    visited = set()
    to_visit = PriorityQueue()

    current = start
    while current != goal:
        current_score = best_score[current]

        for neighbor in get_edges(*current):
            if best_score[neighbor] > current_score + 1:
                best_score[neighbor] = current_score + 1
            
            if neighbor not in visited:
                to_visit.put(PrioritizedItem(best_score[neighbor], neighbor))

        visited.add(current)

        current = to_visit.get().item

    return best_score[goal]


def print_room(max_x, max_y):
    for y in range(max_y+1):
        print(''.join('.' if is_empty(x, y) else '#' for x in range(max_x+1)))


def part1():
    print_room(40, 40)
    return dijkstra((1, 1), (31, 39))


def bfs(start, max_depth):
    found = set()


    to_visit = Queue()

    depth, current = 1, start
    while depth <= max_depth:
        found.add(current)

        for neighbor in get_edges(*current):
            if neighbor not in found:
                to_visit.put((depth + 1, neighbor))

        depth, current = to_visit.get()

    return len(found)


def part2():
    return bfs((1, 1), 50)


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
