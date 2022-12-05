import os


# TODO: this is the number of columns in the input
NUM = 3


def compute(input: str):
    mx = []
    lines = input.split("\n")

    l = 0
    breakout = False
    while not breakout:
        for i in range(1, (NUM * 3 + (NUM - 1)), 4):
            mx_i = (i - 1) // 4
            if len(mx) < mx_i + 1:
                mx.append("")
            if len(lines[l]) < i:
                mx[mx_i] += " "
                continue
            c = lines[l][i]
            if c == "1":
                breakout = True
                break
            mx[mx_i] += c

        l += 1

    for i in range(len(mx)):
        mx[i] = mx[i][::-1]
        mx[i] = list(filter(lambda x: x != " ", mx[i]))

    for i in range(l, len(lines)):
        if lines[i] == "":
            continue
        r = lines[i]
        _, num, _, from_queue, _, to_queue = r.split(" ")

        num = int(num)
        from_queue = int(from_queue) - 1
        to_queue = int(to_queue) - 1

        mx[to_queue] += mx[from_queue][-num:][::-1]
        mx[from_queue] = mx[from_queue][:-num]

    sol = ""
    for i in range(len(mx)):
        sol += mx[i][-1]

    return sol


INPUT = """    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"""


def test_compute():
    assert compute(INPUT) == "CMZ"


if __name__ == "__main__":
    p = os.path.split(__file__)[0]
    with open(f"{p}/input.txt") as f:
        input = f.read()
        sol = compute(input)
        print(f"{sol}")
