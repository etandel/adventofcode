import unittest
from solve import (Building, Element, Floor, Generator, Microchip,
                   InvalidMoveError)


MICRO1 = Microchip(Element.Pu)
MICRO2 = Microchip(Element.Sr)
GEN1 = Generator(Element.Pu)
GEN2 = Generator(Element.Sr)


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

    def test_add_single(self):
        # add to empty floor
        self.assertEqual(Floor().add(MICRO1), Floor([MICRO1]))
        self.assertEqual(Floor().add(GEN1), Floor([GEN1]))

        # add to nonempty floor
        self.assertEqual(Floor([GEN1, GEN2, MICRO2]).add(MICRO1),
                         Floor([GEN1, GEN2, MICRO2, MICRO1]))

        self.assertEqual(Floor([MICRO1, GEN2, MICRO2]).add(GEN1),
                         Floor([MICRO1, GEN2, MICRO2, GEN1]))

        # add member
        floor = Floor([GEN1, GEN2, MICRO2])
        self.assertEqual(floor.add(MICRO2), floor)
        self.assertEqual(floor.add(GEN2), floor)

    def test_add_multiple(self):
        self.assertEqual(Floor().add([MICRO1, GEN1]),
                         Floor([MICRO1, GEN1]))

        # add to nonempty floor
        self.assertEqual(Floor([GEN1, GEN2]).add([MICRO1, MICRO2]),
                         Floor([GEN1, GEN2, MICRO1, MICRO2]))

    def test_remove_single(self):
        # remove from empty floor
        self.assertEqual(Floor().remove(MICRO1), Floor())
        self.assertEqual(Floor().remove(GEN1), Floor())

        # remove non-member
        floor = Floor([GEN2, MICRO2])
        self.assertEqual(floor.remove(MICRO1), floor)
        self.assertEqual(floor.remove(GEN1), floor)

        # remove member
        self.assertEqual(floor.remove(MICRO2), Floor([GEN2]))
        self.assertEqual(floor.remove(GEN2), Floor([MICRO2]))

    def test_remove_multiple(self):
        self.assertEqual(Floor().remove([MICRO1, GEN1]), Floor())
        self.assertEqual(Floor([GEN2, MICRO2]).remove([MICRO1, GEN2]),
                         Floor([MICRO2]))


class TestBuilding(unittest.TestCase):
    def test_move(self):
        b = Building([Floor([MICRO1]), Floor(), Floor(), Floor([MICRO2])])

        self.assertRaises(InvalidMoveError, b.move_objects_up, [MICRO2])
        self.assertRaises(InvalidMoveError, b.move_objects_down, [MICRO1])

        new_b = b.move_objects_up([MICRO1]).move_objects_down([MICRO2])
        expected = Building([Floor(),
                             Floor([MICRO1]),
                             Floor([MICRO2]),
                             Floor()])
        self.assertEqual(new_b, expected)

    def is_possible(self):
        possible = Building([Floor(), Floor(), Floor(), Floor()])
        impossible = Building([Floor([MICRO1, GEN2]),
                               Floor(), Floor(), Floor()])
        self.assertTrue(possible.is_possible())
        self.assertFalse(impossible.is_possible())

if __name__ == '__main__':
    unittest.main()
