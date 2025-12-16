import solve

def test(l, r, want):
    v = solve.process_line(l, r)
    assert v == want, f"{v} != {want}"

test("4", 1, 4)

test("34", 1, 4)
test("343", 1, 4)
test("43", 1, 4)

test("43", 2, 43)
test("34", 2, 34)

test("343", 2, 43)
test("334", 2, 34)


test("343", 3, 343)
test("334", 3, 334)

