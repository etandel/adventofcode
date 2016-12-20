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

class Compression(object):
    def parse_marker_data(self, text, marker):
        sub_elements = self.parse(text[:marker.length]).element
        return (Element(sub_elements, marker.repeat),
                text[marker.length:],
                self.parse_regular)

    def parse_marker(self, text):
        match = MARKER_RE.match(text)
        marker = Marker(*map(int, match.groups()))
        return self.parse_marker_data(text[match.end():], marker)

    def parse_regular(self, text):
        try:
            next_marker = text.index('(')
        except ValueError:  # '(' not found
            next_marker = len(text)

        return (Element(text[:next_marker], 1),
                text[next_marker:],
                self.parse_marker)

    def parse(self, text):
        elements = []
        parser = self.parse_regular
        while text:
            element, text, parser = parser(text)
            if element.element:
                elements.append(element)

        return Element(elements, 1)

    def count(self, text):
        return self.parse(text).count()

    def decompress(self, text):
        return self.parse(text).combine()


class TestElement(unittest.TestCase):
    def assert_element(self, element, combined):
        self.assertEqual(element.combine(), combined)
        self.assertEqual(element.count(), len(combined))

    def test_flat(self):
        self.assert_element(Element('abcd', 2), 'abcd' * 2)

    def test_nested(self):
        element = Element([Element('abcd', 2),
                           Element('efg', 2)],
                          repeat=1)
        self.assert_element(element, 'abcd' * 2 + 'efg' * 2)

    def test_multi_nested(self):
        element = Element([Element('abcd', 1),
                           Element([Element([Element('e', 1)], 11)], 15),
                           Element('fg', 1),
                           Element([Element('hij', 1)], 3),
                           Element('klm', 1)],
                          repeat=1)
        combined = ''.join([
            'abcd',
            ('e' * 11) * 15,
            'fg',
            'hij' * 3,
            'klm'
        ])
        self.assert_element(element, combined)


class TestDeepCompression(unittest.TestCase):
    def setUp(self):
        self.comp = Compression()

    def test_parse_marker_data(self):
        text = '(2x2)BCD(2x2)EFG'
        marker = Marker(len(text) - 1, 3)
        element = Element([Element([Element('BC', 1)], 2),

                           Element('D', 1),

                           Element([Element('EF', 1)], 2)],
                          repeat=3)

        expected = (element, 'G', self.comp.parse_regular)
        self.assertEqual(self.comp.parse_marker_data(text, marker), expected)

    def test_parse_marker(self):
        expected = (Element([Element('abc', 1)], 51), 'd',
                    self.comp.parse_regular)
        self.assertEqual(self.comp.parse_marker('(3x51)abcd'), expected)

        self.assertEqual(self.comp.parse_marker('(0x51)'),
                         (Element([], 51), '', self.comp.parse_regular))

        expected = (Element([Element('ab', 1)], 51),
                    '',
                    self.comp.parse_regular)
        self.assertEqual(self.comp.parse_marker('(2x51)ab'), expected)

    def test_parse_regular(self):
        expected = (Element('abcd', 1), '(48x51)efg', self.comp.parse_marker)
        self.assertEqual(self.comp.parse_regular('abcd(48x51)efg'), expected)

        self.assertEqual(self.comp.parse_regular(''),
                         (Element('', 1), '', self.comp.parse_marker))

        self.assertEqual(self.comp.parse_regular('abcde'),
                         (Element('abcde', 1), '', self.comp.parse_marker))

    def test_parse(self):
        text = 'abcd(7x15)(1x11)efg(3x3)hijklm'
        expected = Element([Element('abcd', 1),
                            Element([Element([Element('e', 1)], 11)], 15),
                            Element('fg', 1),
                            Element([Element('hij', 1)], 3),
                            Element('klm', 1)],
                           repeat=1)
        self.assertEqual(self.comp.parse(text), expected)

    def assert_decompress(self, text, expected):
        self.assertEqual(self.comp.decompress(text), expected)

    def test_decompress(self):
        self.assert_decompress('ADVENT', 'ADVENT')
        self.assert_decompress('A(1x5)BC', 'ABBBBBC')
        self.assert_decompress('(3x3)XYZ', 'XYZXYZXYZ')
        self.assert_decompress('A(2x2)BCD(2x2)EFG', 'ABCBCDEFEFG')
        self.assert_decompress('(6x1)(1x3)A', 'AAA')
        self.assert_decompress('X(8x2)(3x3)ABCY', 'XABCABCABCABCABCABCY')


if __name__ == '__main__':
    if '--test' in sys.argv:
        sys.argv.pop(sys.argv.index('--test'))
        unittest.main()
