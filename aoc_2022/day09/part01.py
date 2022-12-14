from dataclasses import dataclass
import os
import time


@dataclass(unsafe_hash=True)
class Pos:
    x: int
    y: int


class Rope:
    def __init__(self):
        self.positions = set()

        self.head = Pos(0, 0)
        self.tail = Pos(0, 0)

        self.add()

    def add(self):
        self.positions.add(str(self.tail))

    def right(self):
        if self.tail.x < self.head.x:
            self.tail.x = self.head.x
            self.tail.y = self.head.y
            self.add()

        self.head.x += 1

    def left(self):
        if self.tail.x > self.head.x:
            self.tail.x = self.head.x
            self.tail.y = self.head.y
            self.add()

        self.head.x -= 1

    def up(self):
        if self.tail.y > self.head.y:
            self.tail.x = self.head.x
            self.tail.y = self.head.y
            self.add()

        self.head.y -= 1

    def down(self):
        if self.tail.y < self.head.y:
            self.tail.x = self.head.x
            self.tail.y = self.head.y
            self.add()

        self.head.y += 1


def compute(input: str):
    rope = Rope()
    for l in input.split("\n"):
        direction, num = l.split()
        for _ in range(int(num)):
            if direction == "R":
                rope.right()
            elif direction == "L":
                rope.left()
            elif direction == "U":
                rope.up()
            else:
                rope.down()

    return len(rope.positions)


INPUT = """R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"""


def test_compute():
    assert compute(INPUT) == 13


if __name__ == "__main__":
    test_compute()

    p = os.path.split(__file__)[0]
    with open(f"{p}/input.txt") as f:
        input = f.read()

        st = time.time_ns()
        sol = compute(input)
        et = time.time_ns()

        print(f"{sol}")
        print(f"Execution time: {(et-st)/1e9:.4f}s")
