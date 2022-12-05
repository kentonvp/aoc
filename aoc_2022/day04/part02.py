import os


def compute(input):
    sol = 0
    for l in input.split("\n"):
        if len(l) == 0:
            continue

        j1, j2 = l.split(",")
        s1, e1 = j1.split("-")
        s2, e2 = j2.split("-")

        job1 = set(range(int(s1), int(e1) + 1))
        job2 = set(range(int(s2), int(e2) + 1))

        if len(job1.intersection(job2)):
            sol += 1
    return sol


INPUT = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""


def test_compute():
    assert compute(INPUT) == 4


if __name__ == "__main__":
    with open("aoc_2022/day04/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
