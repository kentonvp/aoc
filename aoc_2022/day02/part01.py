import os

OTHER_ROCK = "A"
OTHER_PAPER = "B"
OTHER_SISSORS = "C"

ROCK = "X"
PAPER = "Y"
SISSORS = "Z"

SCORE_VAL = {ROCK: 1, PAPER: 2, SISSORS: 3}

WIN = 6
DRAW = 3
LOSS = 0


def compute(input):
    score = 0
    for l in input.split("\n"):
        if len(l) == 0:
            continue

        other = l[0]
        me = l[2]

        # score
        if (
            (other == OTHER_ROCK and me == PAPER)
            or (other == OTHER_PAPER and me == SISSORS)
            or (other == OTHER_SISSORS and me == ROCK)
        ):
            score += WIN
        elif (
            (other == OTHER_ROCK and me == ROCK)
            or (other == OTHER_PAPER and me == PAPER)
            or (other == OTHER_SISSORS and me == SISSORS)
        ):
            score += DRAW

        score += SCORE_VAL[me]

    return score


INPUT = """
A Y
B X
C Z
"""


def test_compute():
    assert compute(INPUT) == 15


if __name__ == "__main__":
    print(os.getcwd())
    with open("aoc_2022/day02/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
