from collections import Counter
from util import read_lines
from util import read_paragraphs

def part1():
    result = sum(
        len(Counter("".join(lines)))
        for lines in read_paragraphs(read_lines("../input6.txt"))
    )

    print(result)


def part2():
    result = sum(
        len(set.intersection(*(set(line) for line in lines)))
        for lines in read_paragraphs(read_lines("../input6.txt")))

    print(result)


part1()
part2()
