import os
import time


MAX_DIR_SIZE = 100_000


def compute(input: str):
    dirs = {}
    curr_dir = ""
    for l in input.split("\n"):
        fsize = 0

        # ls does nothing
        if l.startswith("$ ls"):
            continue

        # edit current directory
        if l.startswith("$ cd"):
            if l.endswith("/"):
                curr_dir = "home"
            elif l.endswith(".."):
                curr_dir = "/".join(curr_dir.split("/")[:-1])
            else:
                curr_dir = "/".join([curr_dir, l.split()[-1]])
            continue

        fsize, fname = l.split()

        if fsize.isdigit():
            d_split = curr_dir.split("/")
            for i, f in enumerate(d_split):
                dirname = "/".join(d_split[: i + 1])
                if dirname not in dirs:
                    dirs[dirname] = 0
                dirs[dirname] += int(fsize)

    return sum([v for v in dirs.values() if v <= MAX_DIR_SIZE])


INPUT = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""


def test_compute():
    assert compute(INPUT) == 95437


if __name__ == "__main__":
    p = os.path.split(__file__)[0]
    with open(f"{p}/input.txt") as f:
        input = f.read()

        st = time.time_ns()
        sol = compute(input)
        et = time.time_ns()

        print(f"{sol}")
        print(f"Execution time: {(et-st)/1e9:.4f}s")
