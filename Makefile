
aoc-daily: aoc-template aoc-input

aoc-template:
	cp -r aoc_2022/dayXX aoc_2022/day${num}

aoc-input:
	poetry run python utils.py --input ${num}

aoc-test:
	poetry run pytest aoc_2022/day${num}/*.py
