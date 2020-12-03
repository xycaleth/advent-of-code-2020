from collections import Counter
from pathlib import Path
import re

text = Path("../input3.txt").read_text()

grid = []
for line in text.split("\n"):
    if not line:
        break

    grid.append([c == "#" for c in line])


def is_tree(grid, x, y):
    row = grid[y]
    return row[x % len(row)]


def count_trees(grid, move_x, move_y):
    count = 0
    x = 0
    y = 0
    while y < len(grid):
        if is_tree(grid, x, y):
            count += 1

        x += move_x
        y += move_y


    return count


def part1():
    count = count_trees(grid, 3, 1)
    print(count)


def part2():
    result = 1
    moves = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ]

    for move_x, move_y in moves:
        result *= count_trees(grid, move_x, move_y)

    print(result)


part1()
part2()
