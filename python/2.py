from collections import Counter
from pathlib import Path
import re

text = Path("../input2.txt").read_text()

entries = []
for line in text.split("\n"):
    if not line:
        break

    m = re.match(r"(\d+)-(\d+) (.): ([a-z]+)", line)
    entries.append((int(m[1]), int(m[2]), m[3], m[4]))


def part1():
    valid = 0
    for least, most, l, password in entries:
        chars = Counter(password)
        if chars[l] >= least and chars[l] <= most:
            valid += 1

    print(valid)


def part2():
    valid = 0
    for pos1, pos2, l, password in entries:
        if (password[pos1 - 1] == l) ^ (password[pos2 - 1] == l):
            valid += 1

    print(valid)


part1()
part2()
