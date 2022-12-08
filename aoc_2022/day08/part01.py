import os
import time


def compute(input: str):
    sol = 0
    _map = []
    mask = []
    for i, l in enumerate(input.split("\n")):
        _map.append([])
        for j, c in enumerate(l):
            _map[i].append(int(c))

    for r in range(len(_map)):
        for c in range(len(_map[0])):
            # edge
            if r == 0 or c == 0 or r == len(_map) - 1 or c == len(_map[0]) - 1:
                sol += 1
                continue

            # visable above/below/right/left
            if (
                all([_map[r][c] > _map[i][c] for i in range(r)])
                or all([_map[r][c] > _map[i][c] for i in range(r + 1, len(_map))])
                or all([_map[r][c] > _map[r][i] for i in range(c)])
                or all([_map[r][c] > _map[r][i] for i in range(c + 1, len(_map[0]))])
            ):
                sol += 1

    return sol


INPUT = """30373
25512
65332
33549
35390"""


def test_compute():
    assert compute(INPUT) == 21


if __name__ == "__main__":
    p = os.path.split(__file__)[0]
    with open(f"{p}/input.txt") as f:
        input = f.read()

        st = time.time_ns()
        sol = compute(input)
        et = time.time_ns()

        print(f"{sol}")
        print(f"Execution time: {(et-st)/1e9:.4f}s")
