import os


def compute(input: str):
    for i in range(len(input) + 1):
        if i < 14:
            continue
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
        sol = compute(input)
        print(f"{sol}")
