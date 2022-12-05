import os


def compute(input):
    score = 0
    rucksack = set()
    for i, l in enumerate(input.split("\n")):
        if not rucksack:
            rucksack = set(l)
        else:
            rucksack = rucksack.intersection(set(l))

        if (i + 1) % 3 == 0:
            score += val(list(rucksack)[0])
            rucksack = set()
    return score


INPUT = """vJrwpWtwJgWrhcsFMMfFFhFp
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
    assert compute(INPUT) == 70


if __name__ == "__main__":
    with open("aoc_2022/day03/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
