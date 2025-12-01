import argparse
import os
from pathlib import Path

import requests
from dotenv import load_dotenv

load_dotenv()

YEAR = 2025


def download_input(day: int, year: int = YEAR):
    print(f"Downloading input for day: {year} - {day}")
    try:
        res = requests.get(
            f"https://adventofcode.com/{year}/day/{day}/input",
            cookies={"session": os.environ.get("SESSION_COOKIE", "SESSION_COOKIE")},
        )
    except Exception as e:
        print(f"Error downloading input: {e}")
        return

    fname = f"aoc_{year}/inputs/day{day:0>2}.txt"
    Path(fname).parent.mkdir(parents=True, exist_ok=True)
    with open(fname, "w") as f:
        f.write(res.text)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("day", type=int)

    args = parser.parse_args()
    assert 1 <= args.day <= 31, "Day must be between 1 and 31"

    download_input(args.day)


if __name__ == "__main__":
    main()
