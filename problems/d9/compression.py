import re
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
    def __init__(self, deep):
        self.deep = deep

    def parse_marker_data(self, text, marker):
        subtext = text[:marker.length]
        sub_elements = (self.parse(subtext).element
                        if self.deep
                        else [Element(subtext, 1)])
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

