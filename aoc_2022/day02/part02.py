import os

OTHER_ROCK = "A"
OTHER_PAPER = "B"
OTHER_SISSORS = "C"

LOSE = "X"
DRAW = "Y"
WIN = "Z"

ROCK = "X"
PAPER = "Y"
SISSORS = "Z"

HAND_VAL = {ROCK: 1, PAPER: 2, SISSORS: 3}

WIN_SCORE = 6
DRAW_SCORE = 3
LOSE_SCORE = 0


def compute(input):
    score = 0
    for l in input.split("\n"):
        if len(l) == 0:
            continue

        other = l[0]
        outcome = l[2]

        # score
        if outcome == WIN:
            if other == OTHER_ROCK:
                me = PAPER
            elif other == OTHER_PAPER:
                me = SISSORS
            else:
                me = ROCK
        elif outcome == LOSE:
            if other == OTHER_ROCK:
                me = SISSORS
            elif other == OTHER_PAPER:
                me = ROCK
            else:
                me = PAPER
        else:
            if other == OTHER_ROCK:
                me = ROCK
            elif other == OTHER_PAPER:
                me = PAPER
            else:
                me = SISSORS

        # calc score
        if (
            (other == OTHER_ROCK and me == PAPER)
            or (other == OTHER_PAPER and me == SISSORS)
            or (other == OTHER_SISSORS and me == ROCK)
        ):
            score += WIN_SCORE
        elif (
            (other == OTHER_ROCK and me == ROCK)
            or (other == OTHER_PAPER and me == PAPER)
            or (other == OTHER_SISSORS and me == SISSORS)
        ):
            score += DRAW_SCORE

        score += HAND_VAL[me]

    return score


INPUT = """
A Y
B X
C Z
"""


def test_compute():
    assert compute(INPUT) == 12


if __name__ == "__main__":
    print(os.getcwd())
    with open("aoc_2022/day02/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
