import os


def compute(input):
    sol = 0
    for l in input.split("\n"):
        if len(l) == 0:
            continue

        j1, j2 = l.split(",")
        s1, e1 = j1.split("-")
        s2, e2 = j2.split("-")
        if (int(s1) <= int(s2) and int(e1) >= int(e2)) or (
            int(s2) <= int(s1) and int(e2) >= int(e1)
        ):
            sol += 1
    return sol


INPUT = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""


def test_compute():
    assert compute(INPUT) == 2


if __name__ == "__main__":
    with open("aoc_2022/day04/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
