import math


def _position(v0, t):
    return (-(t * t) + (2 * v0 + 1) * t) / 2

def max_x(v0):
    return _position(v0, v0)

def position_x(v0, t):
    if t >= v0:
        return max_x(v0)
    else:
        return _position(v0, t)

def position_y(v0, t):
    return _position(v0, t)


def solve_t(v0, s):
    b = v0 + 1/2
    delta = b*b - 2 * s
    # we always get the last t
    return b + math.sqrt(delta)


def t_range(v0, lower, upper):
    lower_t = solve_t(v0, lower)
    upper_t = solve_t(v0, upper)

    return range(
        math.ceil(min(lower_t, upper_t)),
        math.floor(max(lower_t, upper_t)) + 1
    )


def solve(lower_x, upper_x, lower_y, upper_y):
    count = 0
    highest = 0

    for v0x in range(0, upper_x + 1):
        for v0y in range(lower_y, -lower_y + 1):
            ts = t_range(v0y, lower_y, upper_y)

            for t in ts:
                pos_x = position_x(v0x, t)

                if (lower_x <= pos_x <= upper_x) and (lower_y <= position_y(v0y, t) <= upper_y):
                    count += 1

                    # assumes it is always symetric
                    t_maxy = math.floor((2* (v0y) + 1) / 2)
                    max_y = position_y(v0y, t_maxy)

                    highest = max(highest, max_y)

                    break


    return count, highest


print(solve(lower_x = 79, upper_x = 137, lower_y = -176, upper_y = -117))
#print(solve(lower_x = 20, upper_x = 30, lower_y = -10, upper_y = -5))
