import os


def compute(input: str):
    sol = 0
    return sol


INPUT = """ """


def test_compute():
    assert compute(INPUT) == 1024


if __name__ == "__main__":
    p = os.path.split(__file__)[0]
    with open(f"{p}/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
