from dataclasses import dataclass
import os
import time


@dataclass(unsafe_hash=True)
class Pos:
    x: int = 0
    y: int = 0


class Rope:
    def __init__(self, x=0, y=0):
        self.positions = set()

        self.head = Pos(x, y)
        self.tails = [Pos(), Pos(), Pos(), Pos(), Pos(), Pos(), Pos(), Pos(), Pos()]

        self.add()

    def add(self):
        self.positions.add(str(self.tails[-1]))

    def right(self):
        self.head.x += 1
        self.follow()

    def left(self):
        self.head.x -= 1
        self.follow()

    def up(self):
        self.head.y -= 1
        self.follow()

    def down(self):
        self.head.y += 1
        self.follow()

    def follow(self):
        prev = Pos(self.head.x, self.head.y)
        for tail in self.tails:
            if abs(prev.x - tail.x) > 1:
                # need to make move in x direction
                if prev.x < tail.x:
                    tail.x -= 1
                else:
                    tail.x += 1
                if prev.y != tail.y:
                    if prev.y < tail.y:
                        tail.y -= 1
                    else:
                        tail.y += 1

            elif abs(prev.y - tail.y) > 1:
                # need to make move in y direction
                if prev.y < tail.y:
                    tail.y -= 1
                else:
                    tail.y += 1

                if prev.x != tail.x:
                    if prev.x < tail.x:
                        tail.x -= 1
                    else:
                        tail.x += 1

            prev.x = tail.x
            prev.y = tail.y

        self.add()


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


INPUT = """R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"""


def test_compute():
    assert compute(INPUT) == 36


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
