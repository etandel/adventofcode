import re
import sys
import unittest

from collections import namedtuple


MARKER_RE = re.compile(r'\((\d+)x(\d+)\)')

Marker = namedtuple('Marker', 'length repeat')
Element = namedtuple('Element', 'element repeat')


def parse_marker_data(text, marker):
    return (Element(text[:marker.length], marker.repeat),
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
        elements.append(element)

    return elements


def combine(elements):
    return ''.join(el.element * el.repeat for el in elements)


def count(elements):
    return sum(len(el.element) * el.repeat for el in elements)


def decompress(text):
    return combine(parse(text))


class Tests(unittest.TestCase):
    def test_parse_marker_data(self):
        self.assertEqual(parse_marker_data('abcdefg', Marker(4, 3)),
                         (Element('abcd', 3), 'efg', parse_regular))

    def test_parse_marker(self):
        self.assertEqual(parse_marker('(3x51)abcd'),
                         (Element('abc', 51), 'd', parse_regular))

        self.assertEqual(parse_marker('(0x51)'),
                         (Element('', 51), '', parse_regular))

        self.assertEqual(parse_marker('(2x51)ab'),
                         (Element('ab', 51), '', parse_regular))

    def test_parse_regular(self):
        self.assertEqual(parse_regular('abcd(48x51)efg'),
                         (Element('abcd', 1), '(48x51)efg', parse_marker))

        self.assertEqual(parse_regular(''),
                         (Element('', 1), '', parse_marker))

        self.assertEqual(parse_regular('abcde'),
                         (Element('abcde', 1), '', parse_marker))

    def test_parse(self):
        text = 'abcd(8x15)(10x11)efg(3x3)hijklm'
        elements = parse(text)
        expected = [Element('abcd', 1),
                    Element('(10x11)e', 15),
                    Element('fg', 1),
                    Element('hij', 3),
                    Element('klm', 1)]
        self.assertEqual(elements, expected)

    def test_combine(self):
        elements = [Element('abcd', 1),
                    Element('(10x11)e', 3),
                    Element('fg', 2),
                    Element('', 10),
                    Element('hij', 3),
                    Element('klm', 1)]
        expected = 'abcd(10x11)e(10x11)e(10x11)efgfghijhijhijklm'
        self.assertEqual(combine(elements), expected)

    def test_decompress(self):
        self.assertEqual(decompress('ADVENT'), 'ADVENT')
        self.assertEqual(decompress('A(1x5)BC'), 'ABBBBBC')
        self.assertEqual(decompress('(3x3)XYZ'), 'XYZXYZXYZ')
        self.assertEqual(decompress('A(2x2)BCD(2x2)EFG'), 'ABCBCDEFEFG')
        self.assertEqual(decompress('(6x1)(1x3)A'), '(1x3)A')
        self.assertEqual(decompress('X(8x2)(3x3)ABCY'), 'X(3x3)ABC(3x3)ABCY')


if __name__ == '__main__':
    if '--test' in sys.argv:
        sys.argv.pop(sys.argv.index('--test'))
        unittest.main()
