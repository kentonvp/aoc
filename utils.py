import argparse
import requests
import os
from dotenv import load_dotenv


load_dotenv()


def download_input(day: int, year: int = 2022):
    res = requests.get(
        f"https://adventofcode.com/{year}/day/{day}/input",
        cookies={"session": os.environ.get("SESSION_COOKIE", "SESSION_COOKIE")},
    )

    if day < 10:
        fname = f"aoc_{year}/day0{day}/input.txt"
    else:
        fname = f"aoc_{year}/day{day}/input.txt"

    with open(fname, "w") as f:
        f.write(res.text)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", dest="day", default=0, type=int)

    args = parser.parse_args()

    if args.day != 0:
        print(f"Downloading input for day: {args.day}")
        download_input(args.day)
