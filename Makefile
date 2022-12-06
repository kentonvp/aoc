
aoc-daily: aoc-template aoc-input

aoc-template:
	cp -r aoc_2022/dayXX aoc_2022/day${day}

aoc-input:
	poetry run python utils.py --input --day=${day}

aoc-test:
	poetry run pytest aoc_2022/day${day}/*.py
