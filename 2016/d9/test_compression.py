import unittest
from compression import Compression, Element, Marker


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


class CompressionTestCaseMixin:
    def assert_parse_market_data(self, text, marker, element, remaining):
        self.assertEqual(self.comp.parse_marker_data(text, marker),
                         (element, remaining, self.comp.parse_regular))

    def assert_parse_marker(self, text, element, remaining):
        self.assertEqual(self.comp.parse_marker(text),
                         (element, remaining, self.comp.parse_regular))

    def assert_parse_regular(self, text, element, remaining):
        self.assertEqual(self.comp.parse_regular(text),
                         (element, remaining, self.comp.parse_marker))

    def assert_decompress(self, text, expected):
        self.assertEqual(self.comp.decompress(text), expected)

    def test_parse_regular(self):
        self.assert_parse_regular('abcd(48x51)efg',
                                  Element('abcd', 1),
                                  '(48x51)efg')

        self.assert_parse_regular('', Element('', 1), '')

        self.assert_parse_regular('abcde', Element('abcde', 1), '')


class TestDeepCompression(CompressionTestCaseMixin, unittest.TestCase):
    def setUp(self):
        self.comp = Compression(True)

    def test_parse_marker_data(self):
        text = '(2x2)BCD(2x2)EFG'
        marker = Marker(len(text) - 1, 3)
        element = Element([Element([Element('BC', 1)], 2),

                           Element('D', 1),

                           Element([Element('EF', 1)], 2)],
                          repeat=3)
        self.assert_parse_market_data(text, marker, element, 'G')

    def test_parse_marker(self):
        element = Element([Element('abc', 1)], 51)
        self.assert_parse_marker('(3x51)abcd', element, 'd')

        self.assert_parse_marker('(0x51)', Element([], 51), '')

        element = Element([Element('ab', 1)], 51)
        self.assert_parse_marker('(2x51)ab', element, '')

    def test_parse(self):
        text = 'abcd(7x15)(1x11)efg(3x3)hijklm'
        expected = Element([Element('abcd', 1),
                            Element([Element([Element('e', 1)], 11)], 15),
                            Element('fg', 1),
                            Element([Element('hij', 1)], 3),
                            Element('klm', 1)],
                           repeat=1)
        self.assertEqual(self.comp.parse(text), expected)

    def test_decompress(self):
        self.assert_decompress('ADVENT', 'ADVENT')
        self.assert_decompress('A(1x5)BC', 'ABBBBBC')
        self.assert_decompress('(3x3)XYZ', 'XYZXYZXYZ')
        self.assert_decompress('A(2x2)BCD(2x2)EFG', 'ABCBCDEFEFG')
        self.assert_decompress('(6x1)(1x3)A', 'AAA')
        self.assert_decompress('X(8x2)(3x3)ABCY', 'XABCABCABCABCABCABCY')


class TestShallowCompression(CompressionTestCaseMixin, unittest.TestCase):
    def setUp(self):
        self.comp = Compression(False)

    def test_parse_marker_data(self):
        self.assert_parse_market_data('abcdefg',
                                      Marker(4, 3),
                                      Element([Element('abcd', 1)], 3),
                                      'efg')

    def test_parse_marker(self):
        self.assert_parse_marker('(3x51)abcd',
                                 Element([Element('abc', 1)], 51),
                                 'd')

        self.assert_parse_marker('(0x51)', Element([Element('', 1)], 51), '')
        self.assert_parse_marker('(2x51)ab',
                                 Element([Element('ab', 1)], 51),
                                 '')

    def test_parse(self):
        text = 'abcd(8x15)(10x11)efg(3x3)hijklm'
        expected = Element([Element('abcd', 1),
                            Element([Element('(10x11)e', 1)], 15),
                            Element('fg', 1),
                            Element([Element('hij', 1)], 3),
                            Element('klm', 1)],
                           repeat=1)
        self.assertEqual(self.comp.parse(text), expected)

    def test_decompress(self):
        self.assert_decompress('ADVENT', 'ADVENT')
        self.assert_decompress('A(1x5)BC', 'ABBBBBC')
        self.assert_decompress('(3x3)XYZ', 'XYZXYZXYZ')
        self.assert_decompress('A(2x2)BCD(2x2)EFG', 'ABCBCDEFEFG')
        self.assert_decompress('(6x1)(1x3)A', '(1x3)A')
        self.assert_decompress('X(8x2)(3x3)ABCY', 'X(3x3)ABC(3x3)ABCY')


if __name__ == '__main__':
    unittest.main()
