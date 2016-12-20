import re
import sys
import unittest

from collections import namedtuple


MARKER_RE = re.compile(r'\((\d+)x(\d+)\)')

Marker = namedtuple('Marker', 'length repeat')
BaseElement = namedtuple('Element', 'element repeat')


class Element(BaseElement):
    def count(self):
        if isinstance(self.element, str):
            length = len(self.element)
        else:
            length = sum(el.count() for el in self.element)
        return length * self.repeat

    def combine(self):
        if isinstance(self.element, str):
            el = self.element
        else:
            el = ''.join(el.combine() for el in self.element)
        return el * self.repeat


def parse_marker_data(text, marker):
    sub_elements = parse(text[:marker.length])
    return (Element(sub_elements, marker.repeat),
            text[marker.length:],
            parse_regular)


def parse_marker(text):
    match = MARKER_RE.match(text)
    marker = Marker(*map(int, match.groups()))
    return parse_marker_data(text[match.end():], marker)


def parse_regular(text):
    try:
        next_marker = text.index('(')
    except ValueError:  # '(' not found
        next_marker = len(text)

    return (Element(text[:next_marker], 1),
            text[next_marker:],
            parse_marker)


def parse(text):
    elements = []
    state = parse_regular
    while text:
        element, text, state = state(text)
        if element.element:
            elements.append(element)

    return elements


def combine(elements):
    return ''.join(el.combine() for el in elements)


def count(elements):
    return sum(el.count() for el in elements)


def decompress(text):
    return combine(parse(text))


class Tests(unittest.TestCase):
    def test_parse_marker_data(self):
        text = '(2x2)BCD(2x2)EFG'
        marker = Marker(len(text) - 1, 3)
        element = Element([Element([Element('BC', 1)], 2),

                           Element('D', 1),

                           Element([Element('EF', 1)], 2)],
                          repeat=3)

        expected = (element, 'G', parse_regular)
        self.assertEqual(parse_marker_data(text, marker), expected)

    def test_parse_marker(self):
        expected = (Element([Element('abc', 1)], 51), 'd', parse_regular)
        self.assertEqual(parse_marker('(3x51)abcd'), expected)

        self.assertEqual(parse_marker('(0x51)'),
                         (Element([], 51), '', parse_regular))

        self.assertEqual(parse_marker('(2x51)ab'),
                         (Element([Element('ab', 1)], 51), '', parse_regular))

    def test_parse_regular(self):
        self.assertEqual(parse_regular('abcd(48x51)efg'),
                         (Element('abcd', 1), '(48x51)efg', parse_marker))

        self.assertEqual(parse_regular(''),
                         (Element('', 1), '', parse_marker))

        self.assertEqual(parse_regular('abcde'),
                         (Element('abcde', 1), '', parse_marker))

    def test_parse(self):
        text = 'abcd(7x15)(1x11)efg(3x3)hijklm'
        elements = parse(text)
        expected = [Element('abcd', 1),
                    Element([Element([Element('e', 1)], 11)], 15),
                    Element('fg', 1),
                    Element([Element('hij', 1)], 3),
                    Element('klm', 1)]
        self.assertEqual(elements, expected)

    def test_combine(self):
        elements = [Element('abcd', 1),
                    Element([Element([Element('e', 1)], 11)], 15),
                    Element('fg', 1),
                    Element([Element('hij', 1)], 3),
                    Element('klm', 1)]
        expected = ''.join([
            'abcd',
            ('e' * 11) * 15,
            'fg',
            'hij' * 3,
            'klm'
        ])
        self.assertEqual(combine(elements), expected)

    def test_decompress(self):
        self.assertEqual(decompress('ADVENT'), 'ADVENT')
        self.assertEqual(decompress('A(1x5)BC'), 'ABBBBBC')
        self.assertEqual(decompress('(3x3)XYZ'), 'XYZXYZXYZ')
        self.assertEqual(decompress('A(2x2)BCD(2x2)EFG'), 'ABCBCDEFEFG')
        self.assertEqual(decompress('(6x1)(1x3)A'), 'AAA')
        self.assertEqual(decompress('X(8x2)(3x3)ABCY'), 'XABCABCABCABCABCABCY')

    def assert_count(self, text):
        self.assertEqual(count(parse(text)), len(decompress(text)))

    def test_count(self):
        self.assert_count('ADVENT')
        self.assert_count('A(1x5)BC')
        self.assert_count('(3x3)XYZ')
        self.assert_count('A(2x2)BCD(2x2)EFG')
        self.assert_count('(6x1)(1x3)A')
        self.assert_count('X(8x2)(3x3)ABCY')


if __name__ == '__main__':
    if '--test' in sys.argv:
        sys.argv.pop(sys.argv.index('--test'))
        unittest.main()
