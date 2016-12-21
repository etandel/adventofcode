import unittest
from solve import Element, Floor, Generator, Microchip


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
    unittest.main()
