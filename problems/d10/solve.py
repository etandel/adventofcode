import re
import sys
import unittest
from operator import attrgetter


INPUT = open('input.txt')


class Target:
    def __init__(self, target_id):
        self.target_id = target_id

    def receive(self, val):
        raise NotImplementedError()

    def __repr__(self):
        return '{}({})'.format(type(self).__name__, self.target_id)

    __str__ = __repr__


class Bot(Target):
    def __init__(self, target_id):
        super(Bot, self).__init__(target_id)
        self.vals = []
        self.targets = []
        self.the_one = False

    def receive_instruction(self, low_target, high_target):
        self.targets = [low_target, high_target]

    def receive(self, val):
        self.vals.append(val)
        self.vals.sort()
        if self.vals == [17, 61]:
            self.the_one = True

    def can_execute(self):
        return len(self.vals) == 2

    def execute(self):
        low_target, high_target = self.targets
        low, high = self.vals

        low_target.receive(low)
        high_target.receive(high)

        self.vals = []


class Bin(Target):
    def __init__(self, target_id):
        super(Bin, self).__init__(target_id)
        self.vals = []

    def receive(self, val):
        self.vals.append(val)


class TargetRepo:
    def __init__(self):
        self.target_repos = {
            Bin: {},
            Bot: {},
        }

    @property
    def bots(self):
        return self.target_repos[Bot].values()

    @property
    def bins(self):
        return self.target_repos[Bin].values()

    def get_repo(self, target_type):
        if isinstance(target_type, str):
            target_type = Bin if target_type == 'output' else Bot

        return self.target_repos[target_type], target_type

    def get(self, target_type, target_id):
        repo, target_type = self.get_repo(target_type)

        if target_id in repo:
            return repo[target_id]
        else:
            target = target_type(target_id)
            repo[target_id] = target
            return target


BOT_INSTRUCTION_RE = re.compile(r'^bot (\d+) gives '
                                r'low to (\w+) (\d+) and '
                                r'high to (\w+) (\d+)$')

INPUT_RE = re.compile(r'value (\d+) goes to bot (\d+)')


def parse_bot(instruction):
    return BOT_INSTRUCTION_RE.findall(instruction)[0]


def parse_input(instruction):
    val, bot = INPUT_RE.findall(instruction)[0]
    return bot, int(val)


def initialize_targets():
    targets = TargetRepo()

    for instruction in INPUT.readlines():
        if instruction.startswith('bot'):
            bot_id, type_low, id_low, type_high, id_high = parse_bot(instruction)
            bot = targets.get(Bot, bot_id)
            bot.receive_instruction(targets.get(type_low, id_low),
                                    targets.get(type_high, id_high))
        else:
            bot_id, value = parse_input(instruction)
            targets.get(Bot, bot_id).receive(value)

    return targets


def go():
    targets = initialize_targets()
    to_execute = filter(Bot.can_execute, targets.bots)

    while to_execute:
        for bot in to_execute:
            bot.execute()

        to_execute = list(filter(Bot.can_execute, targets.bots))

    return targets


def part1():
    targets = go()
    return next(filter(attrgetter('the_one'), targets.bots)).target_id


def part2():
    targets = go()

    prod = 1
    for output in filter(lambda b: int(b.target_id) < 3, targets.bins):
        for val in output.vals:
            prod *= val
    return prod


class TestBots(unittest.TestCase):
    def test_parse_bot_instruction(self):
        inst = 'bot 138 gives low to bot 100 and high to bot 74'
        self.assertEqual(parse_bot(inst), ('138', 'bot', '100', 'bot', '74'))

        inst = 'bot 195 gives low to output 4 and high to bot 66'
        self.assertEqual(parse_bot(inst), ('195', 'output', '4', 'bot', '66'))

    def test_parse_input(self):
        self.assertEqual(parse_input('value 3 goes to bot 29'), ('29', 3))


if __name__ == '__main__':
    if '--test' in sys.argv:
        sys.argv.pop(sys.argv.index('--test'))
        unittest.main()
    else:
        print((part1 if sys.argv[1] == '1' else part2)())
