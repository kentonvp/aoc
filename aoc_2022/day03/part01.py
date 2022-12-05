import os


def compute(input):
    score = 0
    for l in input.split("\n"):
        if len(l) == 0:
            continue

        midpt = len(l) // 2
        s1 = set(l[:midpt])
        s2 = set(l[midpt:])
        v = list(s1.intersection(s2))[0]
        score += val(v)
    return score


INPUT = """
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"""


def val(c: str):
    if c.isupper():
        return ord(c) - ord("A") + 27
    return ord(c) - ord("a") + 1


def test_compute():
    assert compute(INPUT) == 157


if __name__ == "__main__":
    with open("aoc_2022/day03/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
