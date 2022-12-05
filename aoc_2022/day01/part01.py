import os


def compute(input):
    max_calories = 0
    curr_calories = 0
    for i, l in enumerate(input.split("\n")):
        if len(l.strip()) == 0:
            max_calories = max(max_calories, curr_calories)
            curr_calories = 0
        else:
            curr_calories += int(l.strip())

    return max_calories


INPUT = """
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"""


def test_compute():
    assert compute(INPUT) == 24000


if __name__ == "__main__":
    print(os.getcwd())
    with open("aoc_2022/day01/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
