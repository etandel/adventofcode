import sys
from enum import Enum
from collections import Iterable
from itertools import chain


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

    __str__ = __repr__


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

    def __eq__(self, other):
        return (type(self) == type(other) and
                self.get_objects() == other.get_objects())

    __str__ = __repr__

    def __contains__(self, key):
        return key in self._get_bag(key)

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
