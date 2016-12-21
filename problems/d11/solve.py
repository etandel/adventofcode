import sys
import unittest
from enum import Enum
from functools import partial
from itertools import chain
from operator import ne


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
        return ('{}({!r})'
                .format(type(self).__name__,
                        list(chain(self.get_generators(),
                                   self.get_microchips()))))

    def __eq__(self, other):
        return (type(self) == type(other) and
                self.get_generators() == other.get_generators() and
                self.get_microchips() == other.get_microchips())

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

    def add(self, obj):
        new_objs = list(chain(self.get_generators(),
                              self.get_microchips(),
                              (obj,)))
        return type(self)(new_objs)

    def remove(self, obj):
        new_objs = [o
                    for o in chain(self.get_generators(),
                                   self.get_microchips())
                    if o != obj]
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


class TestFloor(unittest.TestCase):
    def assert_is_possible(self, objects, expected):
        assertion = self.assertTrue if expected else self.assertFalse
        return assertion(Floor(objects).is_possible())

    def test_is_possible(self):
        # True
        self.assert_is_possible([], True)
        self.assert_is_possible([Generator(Element.Tm)], True)
        self.assert_is_possible([Microchip(Element.Tm)], True)

        self.assert_is_possible([Generator(Element.Tm),
                                 Generator(Element.Pu)],
                                True)

        self.assert_is_possible([Generator(Element.Tm),
                                 Microchip(Element.Tm)],
                                True)

        self.assert_is_possible([Generator(Element.Tm),
                                 Microchip(Element.Tm),
                                 Generator(Element.Pu),
                                 Microchip(Element.Pu)],
                                True)

        # False
        self.assert_is_possible([Generator(Element.Tm),
                                 Microchip(Element.Pu)],
                                False)

    def test_add(self):
        micro1 = Microchip(Element.Pu)
        micro2 = Microchip(Element.Sr)
        gen1 = Generator(Element.Pu)
        gen2 = Generator(Element.Sr)

        # add to empty floor
        self.assertEqual(Floor().add(micro1), Floor([micro1]))
        self.assertEqual(Floor().add(gen1), Floor([gen1]))

        # add to nonempty floor
        self.assertEqual(Floor([gen1, gen2, micro2]).add(micro1),
                         Floor([gen1, gen2, micro2, micro1]))

        self.assertEqual(Floor([micro1, gen2, micro2]).add(gen1),
                         Floor([micro1, gen2, micro2, gen1]))

        # add member
        floor = Floor([gen1, gen2, micro2])
        self.assertEqual(floor.add(micro2), floor)
        self.assertEqual(floor.add(gen2), floor)

    def test_remove(self):
        micro1 = Microchip(Element.Pu)
        micro2 = Microchip(Element.Sr)
        gen1 = Generator(Element.Pu)
        gen2 = Generator(Element.Sr)

        # remove from empty floor
        self.assertEqual(Floor().remove(micro1), Floor())
        self.assertEqual(Floor().remove(gen1), Floor())

        # remove non-member
        floor = Floor([gen2, micro2])
        self.assertEqual(floor.remove(micro1), floor)
        self.assertEqual(floor.remove(gen1), floor)

        # remove member
        self.assertEqual(floor.remove(micro2), Floor([gen2]))
        self.assertEqual(floor.remove(gen2), Floor([micro2]))


if __name__ == '__main__':
    if '--test' in sys.argv:
        sys.argv.pop(sys.argv.index('--test'))
        unittest.main()
    else:
        print((part1 if sys.argv[1] == '1' else part2)())
