import os
import time


def compute(input: str):
    for i in range(14, len(input) + 1):
        if len(set(input[i - 14 : i])) == 14:
            return i

    return len(input)


INPUT = """mjqjpqmgbljsphdztnvjfqwrcgsmlb"""


def test_compute():
    assert compute(INPUT) == 19


if __name__ == "__main__":
    p = os.path.split(__file__)[0]
    with open(f"{p}/input.txt") as f:
        input = f.read()

        st = time.time_ns()
        sol = compute(input)
        et = time.time_ns()

        print(f"{'Solution ':#<30s}")
        print(f"{sol}")
        print(f"{'Timing ':#<30s}")
        print(f"{et-st} ns")
