import sys
from enum import Enum
from collections import Iterable
from itertools import chain, combinations, starmap
from operator import add, eq, sub


INPUT = open('input.txt')


def part1():
    return None


def part2():
    return None


class Element(Enum):
    Tm = 'thulium'
    Pu = 'plutonium'
    Sr = 'strontium'
    Pm = 'promethium'
    Ru = 'ruthenium'


class Object:
    def __init__(self, element):
        self.el = element

    def is_compatible(self, other):
        return type(self) == type(other) or self.el == other.el

    def __repr__(self):
        return '{}({!r})'.format(type(self).__name__, self.el)

    def __str__(self):
        return '{}-{}'.format(type(self).__name__[0], self.el.name)


class Microchip(Object):
    pass


class Generator(Object):
    pass


class Floor:
    def __init__(self, objects=[]):
        self._bags = {
            Generator: {o for o in objects if isinstance(o, Generator)},
            Microchip: {o for o in objects if isinstance(o, Microchip)},
        }

    def __repr__(self):
        return '{}({!r})'.format(type(self).__name__, self.get_objects())

    def __str__(self):
        return ' '.join(sorted(map(str, self)))

    def __eq__(self, other):
        return (type(self) == type(other) and
                self.get_objects() == other.get_objects())

    def __contains__(self, key):
        return key in self._get_bag(key)

    def __iter__(self):
        return iter(self.get_objects())

    def _get_bag(self, obj):
        return self._bags[type(obj)]

    def _filter_by_element(self, bag, element):
        if element:
            return {o for o in bag if o.el == element}
        else:
            return set(bag)

    def get_generators(self, element=None):
        return self._filter_by_element(self._bags[Generator], element)

    def get_microchips(self, element=None):
        return self._filter_by_element(self._bags[Microchip], element)

    def get_objects(self, element=None):
        return set(chain(self.get_generators(element),
                         self.get_microchips(element)))

    def add(self, objects):
        objects = objects if isinstance(objects, Iterable) else (objects,)
        new_objs = list(chain(self.get_objects(), objects))
        return type(self)(new_objs)

    def remove(self, objects):
        objects = objects if isinstance(objects, Iterable) else (objects,)
        new_objs = [o
                    for o in self.get_objects()
                    if o not in objects]
        return type(self)(new_objs)

    def is_possible(self):
        """
        Possible iff:
            there are no generators or
            for each microchip there exists a generator with same element
        """
        if self.get_generators():
            for microchip in self.get_microchips():
                if not self.get_generators(microchip.el):
                    return False
        return True


class Building:
    def __init__(self, floors):
        self.floors = floors
        self.object_index = {o: floor_id
                             for floor_id, floor in enumerate(floors)
                             for o in floor}

    def __eq__(self, other):
        return (type(self) == type(other) and
                all(starmap(eq, zip(self.floors, other.floors))))

    def __repr__(self):
        return '{}({!r})'.format(type(self).__name__, self.floors)

    def __str__(self):
        floor_strs = list(map(str, self.floors[::-1]))
        max_len = max(map(len, floor_strs))
        frame = ['_' * (max_len + 2)]
        middle = ['|{}{}|'.format(floor_str, ' ' * (max_len - len(floor_str)))
                  for floor_str in floor_strs]

        return '\n'.join(frame + middle + frame)

    def is_possible(self):
        return all(floor.is_possible() for floor in self.floors)

    def _move(self, position, objects, op):
        to = op(position, 1)

        if to in (-1, 4):
            return None
        else:
            new_floors = list(self.floors)
            new_floors[position] = new_floors[position].remove(objects)
            new_floors[to] = new_floors[to].add(objects)

            return to, type(self)(new_floors)

    def move_objects_up(self, position, objects):
        return self._move(position, objects, add)

    def move_objects_down(self, position, objects):
        return self._move(position, objects, sub)

    def get_possible_buildings(self, position):
        possibilities = []

        combs = chain(combinations(self.floors[position], 1),
                      combinations(self.floors[position], 2))

        for objects in combs:
            for mover in (self.move_objects_up, self.move_objects_down):
                move = mover(position, objects)
                if move and move[1].is_possible():
                    possibilities.append(move)
        return possibilities


BUILDING = [
    Floor([Generator(Element.Tm), Microchip(Element.Tm),
           Generator(Element.Pu), Generator(Element.Sr)]),

    Floor([Microchip(Element.Pu), Microchip(Element.Sr)]),

    Floor([Generator(Element.Pm), Microchip(Element.Pm),
           Generator(Element.Ru), Microchip(Element.Ru)]),

    Floor(),
]


if __name__ == '__main__':
    print((part1 if sys.argv[1] == '1' else part2)())
