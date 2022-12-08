import os
import time


def compute(input: str):
    max_score = 0
    _map = []
    for i, l in enumerate(input.split("\n")):
        _map.append([])
        for j, c in enumerate(l):
            _map[i].append(int(c))

    for r in range(1, len(_map)):
        for c in range(1, len(_map[0])):
            curr = _map[r][c]

            # visable above/below/right/left
            score = 1

            # check UP
            cnt = 0
            for i in range(r - 1, -1, -1):
                cnt += 1
                if _map[i][c] >= curr:
                    break

            score *= cnt
            if score == 0:
                continue

            # check DOWN
            cnt = 0
            for i in range(r + 1, len(_map)):
                cnt += 1
                if _map[i][c] >= curr:
                    break
            score *= cnt
            if score == 0:
                continue

            # check LEFT
            cnt = 0
            for i in range(c - 1, -1, -1):
                cnt += 1
                if _map[r][i] >= curr:
                    break
            score *= cnt
            if score == 0:
                continue

            # check down
            cnt = 0
            for i in range(c + 1, len(_map[0])):
                cnt += 1
                if _map[r][i] >= curr:
                    break
            score *= cnt
            if score == 0:
                continue

            max_score = max(max_score, score)

    return max_score


INPUT = """30373
25512
65332
33549
35390"""


def test_compute():
    assert compute(INPUT) == 8


if __name__ == "__main__":
    p = os.path.split(__file__)[0]
    with open(f"{p}/input.txt") as f:
        input = f.read()

        st = time.time_ns()
        sol = compute(input)
        et = time.time_ns()

        print(f"{sol}")
        print(f"Execution time: {(et-st)/1e9:.4f}s")
